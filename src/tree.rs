use std::borrow::Borrow;
use std::mem::size_of;
use crate::bounding_box::BoundingBox;
use crate::node::Node;
use crate::spatial_indexable::SpatialIndexable;


// TODO: Can remove Coord?
#[derive(Debug)]
pub struct RTree<Coord, Index, const N: usize>
    where Coord: PartialOrd + Copy,
          Index: SpatialIndexable<N>,
{
    root: Node<Coord, Index, N>,
    max_size: usize,
}

impl<Coord, Index, const N: usize> RTree<Coord, Index, N>
    where Coord: PartialOrd + Copy,
          Index: SpatialIndexable<N>,

{
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

    pub fn search(&self, bound: impl Borrow<BoundingBox<Coord, N>>) -> impl Iterator<Item=&Index> {
        self.root.search(bound)
    }

    pub fn remove(&mut self, data: impl IntoIterator<Item=Index>) {
        for item in data {
            self.root.remove(item);
        }
    }
}
