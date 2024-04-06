use crate::bounding_box::BoundingBox;
use crate::spatial_indexable::SpatialIndexable;

#[derive(Debug)]
enum NodeOrData<Item>
    where Item: SpatialIndexable {
    Node(Node<Item>),
    Data(Item),
}

#[derive(Debug)]
pub(crate) struct Node<Item>
    where Item: SpatialIndexable {
    children: Vec<NodeOrData<Item>>,
}

impl<Item> Node<Item>
    where Item: SpatialIndexable {
    pub(crate) fn new() -> Self { Self { children: vec![] } }

    pub(crate) fn insert(&mut self, bound: BoundingBox<Item>, item: Item) -> impl Iterator<Item=&Item> {
        todo!();
        vec![].iter()
    }

    pub(crate) fn search(&self, bound: &BoundingBox<Item>) -> impl Iterator<Item=&Item> {
        todo!();
        vec![].iter()
    }

    pub(crate) fn remove(&mut self, bound: &BoundingBox<Item>) -> impl Iterator<Item=Item> {
        todo!();
        vec![].into_iter()
    }
}
