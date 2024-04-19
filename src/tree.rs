use crate::bounding_box::BoundingBox;
use crate::node::Node;

#[derive(Debug)]
pub struct RTree<T, const N: usize> {
    root: Option<Node<T, N>>,
    max_size: usize,
    length: usize,
}

impl<T, const N: usize> RTree<T, N> {
    pub fn new() -> RTree<T, N> {
        RTree {
            root: None,
            max_size: 20,
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn insert(&mut self, bbox: BoundingBox<N>, data: T) {
        match &mut self.root {
            Some(root) => { root.insert(bbox, data); }
            None => { self.root = Some(Node::new(bbox, data)); }
        }

        self.length += 1;
    }

    pub fn search(&mut self, bbox: &BoundingBox<N>) -> impl Iterator<Item=&(&BoundingBox<N>, &T)> {
        todo!();
        vec![].iter()
    }

    pub fn remove(&mut self, bbox: &BoundingBox<N>) -> impl Iterator<Item=&(BoundingBox<N>, T)> {
        todo!();
        vec![].iter()
    }
}
