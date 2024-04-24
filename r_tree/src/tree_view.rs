use crate::{BoundingBox, RTree};
use crate::node::{Data, Node};

#[derive(Debug)]
pub struct LeafView<'a, T, const N: usize> {
    pub bbox: &'a BoundingBox<N>,
    pub data: &'a T,
}

#[derive(Debug)]
pub struct NodeView<'a, T, const N: usize> {
    pub bbox: &'a BoundingBox<N>,
    pub nodes: NodesView<'a, T, N>,
}

#[derive(Debug)]
pub enum NodesView<'a, T, const N: usize> {
    Leaf(LeafView<'a, T, N>),
    Inners(Vec<NodeView<'a, T, N>>),
}

#[derive(Debug)]
pub struct RootView<'a, T, const N: usize> {
    pub nodes: NodesView<'a, T, N>,
}

impl<'a, T, const N: usize> RootView<'a, T, N> {
    pub fn new(tree: &'a RTree<T, N>) -> Self {
        RootView { nodes: Self::rec(&tree.root) }
    }

    fn rec(node: &'a Node<T, N>) -> NodesView<'a, T, N> {
        match &node.data {
            Data::Leaf(leaf) => NodesView::Leaf(
                LeafView {
                    bbox: &node.bbox,
                    data: leaf,
                }),
            Data::Inner(nodes) => {
                NodesView::Inners(
                    nodes.iter().map(|node| NodeView {
                        bbox: &node.bbox,
                        nodes: Self::rec(node),
                    }).collect()
                )
            }
        }
    }
}
