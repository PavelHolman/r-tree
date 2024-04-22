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
    const MAX_SIZE: usize = 42;

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
    /// The recursion returns a new node to add if the node has to be split in case of inner nodes,
    /// or a new leaf node if it was created and needs to be added.
    pub(crate) fn insert(&mut self, bbox: BoundingBox<N>, data: T) -> Option<Self> {
        match &mut self.data {
            Data::Leaf(leaf) => {
                if self.bbox == bbox {
                    *leaf = data;
                    return None;
                }
                Some(Self::new_leaf(bbox, data))
            }
            Data::Inner(nodes) => {
                if nodes.is_empty() {
                    nodes.push(Self::new_leaf(bbox, data));
                    self.bbox = bbox;
                    return None;
                }

                let mut best_node = Self::choose_best_node(nodes, &bbox);

                if let Some(node_to_add) = best_node.insert(bbox, data) {
                    nodes.push(node_to_add);
                    self.bbox = BoundingBox::union(&[&self.bbox, &bbox]);
                    if nodes.len() > Self::MAX_SIZE {
                        return Some(self.split());
                    }
                }
                None
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

    fn split(&mut self) -> Self {
        todo!("split the node into two -- keep something in self, move rest into other");
    }
}
