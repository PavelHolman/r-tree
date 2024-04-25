use std::io::{Read, Seek, Write};

use bincode::{deserialize, serialize};

use crate::bounding_box::BoundingBox;
use crate::iterators::{LeafIter, NearestNeighborIter, SearchIter};
use crate::tree::SplitPolicy;

pub(crate) type NodeId = usize;

#[derive(Debug)]
pub(crate) enum Data<T, const N: usize> {
    Leaf(T),
    Inner(Vec<NodeId>),
}

#[derive(Debug)]
pub(crate) struct Node<T, const N: usize> {
    pub(crate) id: NodeId,
    pub(crate) bbox: BoundingBox<N>,
    pub(crate) data: Data<T, N>,
}

#[derive(Debug)]
pub(crate) struct Nodes<T, const N: usize> {
    cache: Vec<Node<T, N>>,
    file: std::fs::File,
    next_id: NodeId,
    capacity: usize,
}

impl<T, const N: usize> Nodes<T, N> {
    pub(crate) fn new(capacity: usize, file: std::fs::File) -> Self {
        let cache = vec![];
        let next_id = 0;
        Self { cache, file, next_id, capacity }
    }

    fn write_node(&mut self, node: &Node<T, N>) {
        let offset = node.id * std::mem::size_of::<Node<T, N>>();
        let encoded_node = serialize(&node).expect("Node serialization failed.");

        self.file.seek(std::io::SeekFrom::Start(offset as u64)).expect("Node write failed.");
        self.file.write(&encoded_node).expect("Node write failed.");
    }

    fn read_node(&mut self, id: NodeId) -> &mut Node<T, N> {
        let offset = id * std::mem::size_of::<Node<T, N>>();
        self.file.seek(std::io::SeekFrom::Start(offset as u64)).expect("Node read failed.");

        let mut buffer = vec![0; std::mem::size_of::<Node<T, N>>()];
        self.file.read_exact(&mut buffer).expect("Node read failed.");

        let mut node = deserialize(&buffer).expect("Node deserialization failed.");
        self.cache.insert(id % self.capacity, node);
        &mut self.cache.last().unwrap()
    }

    pub(crate) fn new_leaf(&mut self, bbox: BoundingBox<N>, data: T) -> Node<T, N> {
        let node = Node {
            id: self.next_id,
            bbox,
            data: Data::Leaf(data),
        };
        self.next_id += 1;
        self.write_node(&node);
        node
    }

    pub(crate) fn new_node(&mut self, nodes: Vec<NodeId>) -> Node<T, N> {
        let bbox = BoundingBox::union(&nodes.iter().map(|id| &self.read_node(*id).bbox).collect::<Vec<_>>());
        let node = Node {
            id: self.next_id,
            bbox,
            data: Data::Inner(nodes),
        };
        self.next_id += 1;
        self.write_node(&node);
        node
    }

    pub(crate) fn get_node(&mut self, id: &NodeId) -> &mut Node<T, N> {
        let node = &mut self.cache.get_mut(*id % self.capacity);
        if node.as_ref().is_some_and(|node| node.id == *id) {
            return node.take().unwrap();
        }

        self.read_node(*id)
    }
}

impl<T, const N: usize> Node<T, N> {
    /// Insert a new bounding box with data into the tree.
    /// Returns the old value if it was replaced.
    pub(crate) fn insert(&mut self, bbox: BoundingBox<N>, data: T, capacity: usize, policy: &SplitPolicy, nodes: &mut Nodes<T, N>) -> Option<T> {
        // TODO: Wrap implementation in a struct with all the parameters to avoid passing them around.
        // The root node could also be treated separately by adding Root kind to Data and avoid the unsafe block.
        // Additionally, no [0,0] bbox would be created for empty root node.
        // I really should consider actually doing this and not keeping this TODO around. :)
        let (split, old) = self.insert_impl(bbox, data, capacity, &policy, nodes);
        if let Some(node) = split {
            unsafe {
                let old_root = std::ptr::read(self);
                let new_root = nodes.new_node(vec![old_root.id, node.id]);
                std::ptr::write(self, new_root);
            }
            if let Data::Inner(node_ids) = &mut self.data {
                for node_id in node_ids {
                    let node = nodes.get_node(node_id);
                    node.shrink(nodes);
                }
            }
        }

        self.shrink(nodes);
        old
    }

    /// Insert a new bounding box with data into the tree.
    /// The recursion returns:
    /// - a new node to add if the node has to be split in case of inner nodes,
    /// - or a new leaf node if it was created and needs to be added
    ///
    /// and
    /// - an old value, if it was replaced.
    fn insert_impl(&mut self, bbox: BoundingBox<N>, data: T, capacity: usize, policy: &SplitPolicy, nodes: &mut Nodes<T, N>) -> (Option<Self>, Option<T>) {
        match &mut self.data {
            Data::Leaf(leaf) => {
                if self.bbox == bbox {
                    return (None, Some(std::mem::replace(leaf, data)));
                }
                (Some(nodes.new_leaf(bbox, data)), None)
            }
            Data::Inner(node_ids) => {
                if node_ids.is_empty() {
                    let node = nodes.new_leaf(bbox, data);
                    node_ids.push(node.id);
                    self.bbox = bbox;
                    return (None, None);
                }

                let best_node_id = Self::choose_best_node(node_ids, &bbox, nodes);
                let best_node = nodes.get_node(best_node_id);

                let (node_to_add, old) = best_node.insert_impl(bbox, data, capacity, &policy, nodes);

                if let Some(node_to_add) = node_to_add {
                    node_ids.push(node_to_add.id);
                    self.bbox = BoundingBox::union(&[&self.bbox, &bbox]);
                    if node_ids.len() > capacity {
                        let other_nodes = Self::split_nodes(node_ids, &policy);
                        self.shrink(nodes);
                        return (Some(Self::new_node(other_nodes)), old);
                    }
                }
                (None, old)
            }
        }
    }

    /// Chooses the best node to insert the new bounding box into.
    /// The best node is the one with the smallest increase in area after adding the bounding box.
    /// If the area increase is the same, the node with the least amount of children is chosen.
    fn choose_best_node<'a>(node_ids: &'a Vec<NodeId>, bbox: &BoundingBox<N>, nodes: &mut Nodes<T, N>) -> &'a Node<T, N> {
        node_ids
            .iter()
            .min_by_key(|node_id| {
                let node = nodes.get_node(node_id);
                (BoundingBox::union(&[&node.bbox, bbox]).area() - node.bbox.area(),
                 match &node.data {
                     Data::Leaf(_) => 1,
                     Data::Inner(nodes) => nodes.len(),
                 })
            })
            .expect("Should be non empty, because leaf should have been inserted for empty.")
    }

    fn split_nodes(nodes: &mut Vec<Self>, policy: &SplitPolicy) -> Vec<Self> {
        use rand::Rng;
        use rand::seq::SliceRandom;
        use itertools::Itertools;

        match policy {
            SplitPolicy::Random => {
                nodes.shuffle(&mut rand::thread_rng());
                nodes.split_off(rand::thread_rng().gen_range(1..nodes.len()))
            }
            SplitPolicy::Half => {
                nodes.split_off(nodes.len() / 2)
            }
            SplitPolicy::Greedy => {
                let ((i, j), _) = nodes.iter().enumerate()
                    .flat_map(|(i, l)| nodes.iter().enumerate().skip(i + 1)
                        .map(move |(j, r)| ((i, j), l.bbox.distance(&r.bbox))))
                    .min_by(|(_, d1), (_, d2)| d1.total_cmp(d2))
                    .expect("Should be non empty, because splitting node without at least 2 elements makes no sense.");

                let mut left = vec![nodes.swap_remove(usize::max(i, j))];
                let mut right = vec![nodes.swap_remove(usize::min(i, j))];

                nodes
                    .sort_by(|node1, node2|
                        f64::min(node1.bbox.distance(&left[0].bbox), node1.bbox.distance(&right[0].bbox))
                            .total_cmp(&f64::min(node2.bbox.distance(&left[0].bbox), node2.bbox.distance(&right[0].bbox)))
                            .reverse());

                for i in (0..nodes.len()).rev() {
                    let bbox = &nodes[i].bbox;
                    let left_bboxes = left.iter().map(|node| &node.bbox).chain([bbox]).collect_vec();
                    let right_bboxes = right.iter().map(|node| &node.bbox).chain([bbox]).collect_vec();

                    if BoundingBox::union(&left_bboxes).area() < BoundingBox::union(&right_bboxes).area() {
                        left.push(nodes.swap_remove(i));
                    } else {
                        right.push(nodes.swap_remove(i));
                    }
                }

                *nodes = left;
                right
            }
            SplitPolicy::BruteForce => {
                let (mut permutation, k) =
                    (1..(nodes.len() - 1))
                        .flat_map(|k| {
                            (0..nodes.len())
                                .permutations(nodes.len())
                                .map(move |permutation| (permutation, k))
                        })
                        .min_by_key(|(permutation, k)| {
                            let (left_indices, right_indices) = permutation.split_at(*k);
                            let left = left_indices.iter().map(|&i| &nodes[i]);
                            let right = right_indices.iter().map(|&i| &nodes[i]);

                            let left_area = BoundingBox::union(&left.map(|node| &node.bbox).collect_vec()).area();
                            let right_area = BoundingBox::union(&right.map(|node| &node.bbox).collect_vec()).area();
                            left_area + right_area
                        })
                        .expect("Should be non empty, because splitting node without at least 2 elements makes no sense.");

                permutation.split_off(k)
                    .iter()
                    .sorted()
                    .rev()
                    .map(|i| nodes.swap_remove(*i))
                    .collect_vec()
            }
        }
    }

    fn shrink(&mut self, nodes: &mut Nodes<T, N>) {
        match &mut self.data {
            Data::Leaf(_) => {}
            Data::Inner(node_ids) => {
                self.bbox = BoundingBox::union(&node_ids.iter().map(|id| &nodes.get_node(id).bbox).collect::<Vec<_>>());
            }
        }
    }

    pub(crate) fn iter(&self) -> LeafIter<T, N> {
        LeafIter::new(self)
    }

    pub(crate) fn search<'a>(&'a self, bbox: &'a BoundingBox<N>) -> SearchIter<T, N> {
        SearchIter::new(self, bbox)
    }

    pub(crate) fn nearest<'a>(&'a self, bbox: &'a BoundingBox<N>) -> NearestNeighborIter<T, N> {
        NearestNeighborIter::new(self, bbox)
    }
}
