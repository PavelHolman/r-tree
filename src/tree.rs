use std::mem::size_of;

use crate::bounding_box::BoundingBox;
use crate::node::Node;
use crate::spatial_indexable::SpatialIndexable;

#[derive(Debug)]
pub struct RTree<Index, const N: usize>
    where Index: SpatialIndexable<N>, {
    root: Node<Index, N>,
    max_size: usize,
}

impl<Index, const N: usize> RTree<Index, N>
    where Index: SpatialIndexable<N>, {
    const LEAF_SIZE_IN_BYTES: usize = 2048;
    const TYPE_SIZE_IN_BYTES: usize = size_of::<Index>();

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

    pub fn insert(&mut self, data: impl IntoIterator<Item=Index>) {
        for item in data {
            self.root.insert(item)
        }
    }

    pub fn search(&self, bound: BoundingBox<Index::Coord, N>) -> impl Iterator<Item=&Index> {
        self.root.search(bound)
    }

    pub fn remove(&mut self, data: impl IntoIterator<Item=Index>) {
        for item in data {
            self.root.remove(item);
        }
    }
}
