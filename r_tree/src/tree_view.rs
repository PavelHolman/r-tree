use crate::{BoundingBox, RTree};

pub struct LeafView<'a, T, const N: usize> {
    pub bbox: &'a BoundingBox<N>,
    pub data: &'a T,
}

pub struct NodeView<'a, T, const N: usize> {
    pub bbox: &'a BoundingBox<N>,
    pub nodes: NodesView<'a, T, N>,
}

pub enum NodesView<'a, T, const N: usize> {
    Leaves(Vec<LeafView<'a, T, N>>),
    Inners(Vec<NodeView<'a, T, N>>),
}

pub struct RootView<'a, T, const N: usize> {
    pub nodes: NodesView<'a, T, N>,
}

impl<'a, T, const N: usize> RootView<'a, T, N> {
    pub fn new(tree: &'a RTree<T, N>) -> Self {
        todo!()
    }
}
