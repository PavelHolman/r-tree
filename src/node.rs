use std::borrow::Borrow;
use std::marker::PhantomData;
use crate::bounding_box::BoundingBox;
use crate::spatial_indexable::SpatialIndexable;

#[derive(Debug)]
enum NodeOrData<Coord, Index, const N: usize>
    where Coord: PartialOrd + Copy,
          Index: SpatialIndexable<N> {
    Node(Node<Coord, Index, N>),
    Data(Index),
}

#[derive(Debug)]
pub struct Node<Coord, Index, const N: usize>
    where Coord: PartialOrd + Copy,
          Index: SpatialIndexable<N> {
    children: Vec<NodeOrData<Coord, Index, N>>,
    pd: PhantomData<Coord>,
}

impl<Coord, Index, const N: usize> Node<Coord, Index, N>
    where Coord: PartialOrd + Copy,
          Index: SpatialIndexable<N> {
    pub fn new() -> Self { Self { children: vec![], pd: PhantomData } }

    pub fn insert(&mut self, item: Index) {
        todo!()
    }

    pub(crate) fn search(&self, bound: impl Borrow<BoundingBox<Coord, N>>) -> impl Iterator<Item=&Index> {
        todo!();
    }

    pub fn remove(&mut self, item: Index) {
        todo!()
    }
}
