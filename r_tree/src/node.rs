use crate::bounding_box::BoundingBox;
use crate::iterators::{LeafIter, NearestNeighborIter, SearchIter};
use crate::tree::SplitPolicy;

#[derive(Debug)]
pub(crate) enum Data<T, const N: usize> {
    Leaf(T),
    Inner(Vec<Node<T, N>>),
}

#[derive(Debug)]
pub(crate) struct Node<T, const N: usize> {
    pub(crate) bbox: BoundingBox<N>,
    pub(crate) data: Data<T, N>,
}

impl<T, const N: usize> Node<T, N> {
    pub(crate) fn new_leaf(bbox: BoundingBox<N>, data: T) -> Self {
        Self {
            bbox,
            data: Data::Leaf(data),
        }
    }

    pub(crate) fn new_node(nodes: Vec<Self>) -> Self {
        Self {
            bbox: BoundingBox::union(&nodes.iter().map(|node| &node.bbox).collect::<Vec<_>>()),
            data: Data::Inner(nodes),
        }
    }

    /// Insert a new bounding box with data into the tree.
    /// Returns the old value if it was replaced.
    pub(crate) fn insert(&mut self, bbox: BoundingBox<N>, data: T, capacity: usize, policy: &SplitPolicy) -> Option<T> {
        // TODO: Wrap implementation in a struct with all the parameters to avoid passing them around.
        // The root node could also be treated separately by adding Root kind to Data and avoid the unsafe block.
        // Additionally, no [0,0] bbox would be created for empty root node.
        // I really should consider actually doing this and not keeping this TODO around. :)
        let (split, old) = self.insert_impl(bbox, data, capacity, &policy);
        if let Some(node) = split {
            unsafe {
                let old_root = std::ptr::read(self);
                let new_root = Self::new_node(vec![old_root, node]);
                std::ptr::write(self, new_root);
            }
            if let Data::Inner(nodes) = &mut self.data {
                for node in nodes {
                    node.shrink();
                }
            }
        }

        self.shrink();
        old
    }

    /// Insert a new bounding box with data into the tree.
    /// The recursion returns:
    /// - a new node to add if the node has to be split in case of inner nodes,
    /// - or a new leaf node if it was created and needs to be added
    ///
    /// and
    /// - an old value, if it was replaced.
    fn insert_impl(&mut self, bbox: BoundingBox<N>, data: T, capacity: usize, policy: &SplitPolicy) -> (Option<Self>, Option<T>) {
        match &mut self.data {
            Data::Leaf(leaf) => {
                if self.bbox == bbox {
                    return (None, Some(std::mem::replace(leaf, data)));
                }
                (Some(Self::new_leaf(bbox, data)), None)
            }
            Data::Inner(nodes) => {
                if nodes.is_empty() {
                    nodes.push(Self::new_leaf(bbox, data));
                    self.bbox = bbox;
                    return (None, None);
                }

                let best_node = Self::choose_best_node(nodes, &bbox);

                let (node_to_add, old) = best_node.insert_impl(bbox, data, capacity, &policy);

                if let Some(node_to_add) = node_to_add {
                    nodes.push(node_to_add);
                    self.bbox = BoundingBox::union(&[&self.bbox, &bbox]);
                    if nodes.len() > capacity {
                        let other_nodes = Self::split_nodes(nodes, &policy);
                        self.shrink();
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
    fn choose_best_node<'a>(nodes: &'a mut Vec<Self>, bbox: &BoundingBox<N>) -> &'a mut Node<T, N> {
        nodes
            .iter_mut()
            .min_by_key(|node|
                (BoundingBox::union(&[&node.bbox, bbox]).area() - node.bbox.area(),
                 match &node.data {
                     Data::Leaf(_) => 1,
                     Data::Inner(nodes) => nodes.len(),
                 }))
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

    fn shrink(&mut self) {
        match &mut self.data {
            Data::Leaf(_) => {}
            Data::Inner(nodes) => {
                self.bbox = BoundingBox::union(&nodes.iter().map(|node| &node.bbox).collect::<Vec<_>>());
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
