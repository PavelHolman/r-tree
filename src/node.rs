use crate::bounding_box::BoundingBox;
use crate::spatial_indexable::SpatialIndexable;

#[derive(Debug)]
enum NodeOrData<Index, const N: usize>
    where Index: SpatialIndexable<N> {
    Node(Node<Index, N>),
    Data(Index),
}

#[derive(Debug)]
pub(crate) struct Node<Index, const N: usize>
    where Index: SpatialIndexable<N> {
    children: Vec<NodeOrData<Index, N>>,
}

impl<Index, const N: usize> Node<Index, N>
    where Index: SpatialIndexable<N> {
    pub(crate) fn new() -> Self { Self { children: vec![] } }

    pub(crate) fn insert(&mut self, item: Index) {
        todo!()
    }

    pub(crate) fn search(&self, bound: BoundingBox<Index::Coord, N>) -> impl Iterator<Item=&Index> {
        todo!();
        vec![].iter()
    }

    pub(crate) fn remove(&mut self, item: Index) {
        todo!()
    }
}
