use crate::bounding_box::BoundingBox;
use crate::spatial_indexable::SpatialIndexable;

// TODO: Can Dim be removed?
#[derive(Debug)]
enum NodeOrData<Item, const Dim: usize>
    where Item: SpatialIndexable<Dim> {
    Node(Node<Item, Dim>),
    Data(Item),
}

#[derive(Debug)]
pub(crate) struct Node<Item, const Dim: usize>
    where Item: SpatialIndexable<Dim> {
    children: Vec<NodeOrData<Item, Dim>>,
}

impl<Item, const Dim: usize> Node<Item, Dim>
    where Item: SpatialIndexable<Dim> {
    pub(crate) fn new() -> Self { Self { children: vec![] } }

    pub(crate) fn insert(&mut self, item: Item) {
        todo!()
    }

    pub(crate) fn search(&self, bound: BoundingBox<Item::Coord, Dim>) -> impl Iterator<Item=&Item> {
        todo!();
        vec![].iter()
    }

    pub(crate) fn remove(&mut self, item: Item) {
        todo!()
    }
}
