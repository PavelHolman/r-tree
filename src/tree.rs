use crate::bounding_box::BoundingBox;
use crate::node::Node;

#[derive(Debug)]
pub struct RTree<T, const N: usize> {
    root: Node<T, N>,
    capacity: usize,
    length: usize,
}

impl<T, const N: usize> RTree<T, N> {
    const DISK_PAGE_SIZE_IN_BYTES: usize = 4096;
    const DISK_PAGE_SIZE: usize = 8 * Self::DISK_PAGE_SIZE_IN_BYTES;
    const DEFAULT_T_SIZE: usize = std::mem::size_of::<T>();
    const DEFAULT_CAPACITY: usize = Self::DISK_PAGE_SIZE / Self::DEFAULT_T_SIZE;

    pub fn new() -> RTree<T, N> {
        RTree {
            root: Node::new_node(vec![]),
            capacity: Self::DEFAULT_CAPACITY,
            length: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> RTree<T, N> {
        RTree {
            root: Node::new_node(vec![]),
            capacity,
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn insert(&mut self, bbox: BoundingBox<N>, data: T) -> Option<T> {
        if let Some(old) = self.root.insert(bbox, data, self.capacity) {
            Some(old)
        } else {
            self.length += 1;
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item=(&BoundingBox<N>, &T)> {
        self.root.iter()
    }

    pub fn search<'a, 'b>(&'a self, bbox: &'b BoundingBox<N>) -> impl Iterator<Item=(&'a BoundingBox<N>, &'a T)>
        where 'b : 'a {
        self.root.search(bbox)
    }

    pub fn remove(&mut self, bbox: &BoundingBox<N>) -> impl Iterator<Item=&(BoundingBox<N>, T)> {
        todo!();
        vec![].iter()
    }
}
