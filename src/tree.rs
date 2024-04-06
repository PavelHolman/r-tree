use std::mem::size_of;

use crate::bounding_box::BoundingBox;
use crate::node::Node;
use crate::spatial_indexable::SpatialIndexable;

#[derive(Debug)]
pub struct RTree<Item>
    where Item: SpatialIndexable, {
    root: Node<Item>,
    count: usize,
    max_size: usize,
}

impl<Item> RTree<Item>
    where Item: SpatialIndexable, {
    const LEAF_SIZE_IN_BYTES: usize = 2048;
    const TYPE_SIZE_IN_BYTES: usize = size_of::<Item>();

    pub fn new() -> Self {
        Self {
            root: Node::new(),
            count: 0,
            max_size: Self::LEAF_SIZE_IN_BYTES / Self::TYPE_SIZE_IN_BYTES,
        }
    }

    pub fn with_node_capacity(capacity: usize) -> Self {
        Self {
            root: Node::new(),
            count: 0,
            max_size: capacity,
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn insert(&mut self, data: impl IntoIterator<Item=Item>) {
        for item in data {
            if self.root.insert(item) {
                self.count += 1;
            }
        }
    }

    pub fn search(&self, bound: BoundingBox<Item>) -> impl Iterator<Item=&Item> {
        self.root.search(bound)
    }

    pub fn remove(&mut self, data: impl IntoIterator<Item=Item>) {
        for item in data {
            if self.root.remove(item) {
                self.count -= 1;
            }
        }
    }
}
