use crate::bounding_box::BoundingBox;
use crate::spatial_indexable::SpatialIndexable;

#[derive(Debug)]
enum NodeOrData<Item, const N: usize>
    where Item: SpatialIndexable<N> {
    Node(Node<Item, N>),
    Data(Item),
}

#[derive(Debug)]
pub(crate) struct Node<Item, const N: usize>
    where Item: SpatialIndexable<N> {
    children: Vec<NodeOrData<Item, N>>,
}

impl<Item, const N: usize> Node<Item, N>
    where Item: SpatialIndexable<N> {
    pub(crate) fn new() -> Self { Self { children: vec![] } }

    pub(crate) fn insert(&mut self, item: Item) {
        todo!()
    }

    pub(crate) fn search(&self, bound: BoundingBox<Item::Coord, N>) -> impl Iterator<Item=&Item> {
        todo!();
        vec![].iter()
    }

    pub(crate) fn remove(&mut self, item: Item) {
        todo!()
    }
}
