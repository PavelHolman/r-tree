use std::mem::size_of;

use crate::bounding_box::BoundingBox;
use crate::node::Node;
use crate::spatial_indexable::SpatialIndexable;

#[derive(Debug)]
pub struct RTree<Item, const N: usize>
    where Item: SpatialIndexable<N>, {
    root: Node<Item, N>,
    max_size: usize,
}

impl<Item, const N: usize> RTree<Item, N>
    where Item: SpatialIndexable<N>, {
    const LEAF_SIZE_IN_BYTES: usize = 2048;
    const TYPE_SIZE_IN_BYTES: usize = size_of::<Item>();

    pub fn new() -> Self {
        Self {
            root: Node::new(),
            max_size: Self::LEAF_SIZE_IN_BYTES / Self::TYPE_SIZE_IN_BYTES,
        }
    }

    pub fn with_node_capacity(capacity: usize) -> Self {
        Self {
            root: Node::new(),
            max_size: capacity,
        }
    }

    pub fn insert(&mut self, data: impl IntoIterator<Item=Item>) {
        for item in data {
            self.root.insert(item)
        }
    }

    pub fn search(&self, bound: BoundingBox<Item::Coord, N>) -> impl Iterator<Item=&Item> {
        self.root.search(bound)
    }

    pub fn remove(&mut self, data: impl IntoIterator<Item=Item>) {
        for item in data {
            self.root.remove(item);
        }
    }
}
