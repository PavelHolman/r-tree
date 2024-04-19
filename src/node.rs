use crate::bounding_box::BoundingBox;

#[derive(Debug)]
enum Data<T, const N: usize> {
    Leaf(T),
    Inner(Vec<Node<T, N>>),
}

#[derive(Debug)]
pub(crate) struct Node<T, const N: usize> {
    bbox: BoundingBox<N>,
    data: Data<T, N>,
}

impl<T, const N: usize> Node<T, N> {
    const MAX_SIZE: usize = 42;

    pub(crate) fn new(bbox: BoundingBox<N>, data: T) -> Node<T, N> {
        todo!()
    }

    pub(crate) fn insert(&mut self, bbox: BoundingBox<N>, data: T) {
        todo!();
    }

    fn split() {}
}
