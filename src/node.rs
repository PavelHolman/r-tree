use crate::bounding_box::BoundingBox;

#[derive(Debug)]
enum Data<T, const N: usize> {
    Leaf(T),
    Inner(Vec<Node<T, N>>),
}

#[derive(Debug)]
pub(crate) struct Node<T, const N: usize> {
    bbox: BoundingBox<N>,
    data: Data<T, N>,
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
    pub(crate) fn insert(&mut self, bbox: BoundingBox<N>, data: T, capacity: usize) -> Option<T> {
        let (split, old) = self.insert_impl(bbox, data, capacity);
        if let Some(node) = split {
            unsafe {
                let old_root = std::ptr::read(self);
                let new_root = Self::new_node(vec![old_root, node]);
                std::ptr::write(self, new_root);
            }
        }

        old
    }

    /// Insert a new bounding box with data into the tree.
    /// The recursion returns:
    /// - a new node to add if the node has to be split in case of inner nodes,
    /// - or a new leaf node if it was created and needs to be added
    ///
    /// and
    /// - an old value, if it was replaced.
    fn insert_impl(&mut self, bbox: BoundingBox<N>, data: T, capacity: usize) -> (Option<Self>, Option<T>) {
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

                let (node_to_add, old) = best_node.insert_impl(bbox, data, capacity);

                if let Some(node_to_add) = node_to_add {
                    nodes.push(node_to_add);
                    self.bbox = BoundingBox::union(&[&self.bbox, &bbox]);
                    if nodes.len() > capacity {
                        let other_nodes = Self::split_nodes(nodes);
                        self.bbox = BoundingBox::union(&nodes.iter().map(|node| &node.bbox).collect::<Vec<_>>());
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

    fn split_nodes(nodes: &mut Vec<Self>) -> Vec<Self> {
        // TODO: Better splitting policy
        nodes.split_off(nodes.len() / 2)
    }

    pub(crate) fn iter(&self) -> LeafIter<T, N> {
        LeafIter { nodes_to_visit: vec![self] }
    }

    pub(crate) fn search<'a, 'b>(&'a self, bbox: &'b BoundingBox<N>) -> impl Iterator<Item=(&BoundingBox<N>, &T)>
        where 'b: 'a {
        self
            .iter()
            .filter(|&(b, _)| bbox.contains(b))
    }
}

pub struct LeafIter<'a, T, const N: usize> {
    nodes_to_visit: Vec<&'a Node<T, N>>,
}

impl<'a, T, const N: usize> Iterator for LeafIter<'a, T, N> {
    type Item = (&'a BoundingBox<N>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.nodes_to_visit.pop() {
            match &node.data {
                Data::Leaf(leaf) => return Some((&node.bbox, &leaf)),
                Data::Inner(nodes) => {
                    for child in nodes.iter().rev() {
                        self.nodes_to_visit.push(child);
                    }
                }
            }
        }
        None
    }
}