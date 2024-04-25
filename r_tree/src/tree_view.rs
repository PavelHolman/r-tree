use crate::{BoundingBox, RTree};
use crate::node::{Data, Node};
use crate::tree::T;

pub struct LeafView<'a, const N: usize> {
    pub bbox: &'a BoundingBox<N>,
    pub data: &'a T,
}

pub struct NodeView<'a, const N: usize> {
    pub bbox: &'a BoundingBox<N>,
    pub nodes: NodesView<'a, N>,
}

pub enum NodesView<'a, const N: usize> {
    Leaf(LeafView<'a, N>),
    Inners(Vec<NodeView<'a, N>>),
}

pub struct RootView<'a, const N: usize> {
    pub nodes: NodesView<'a, N>,
}

impl<'a, const N: usize> RootView<'a, N> {
    pub(crate) fn new(tree: &'a RTree<N>) -> Self {
        RootView { nodes: Self::rec(&tree.root) }
    }

    fn rec(node: &'a Node<N>) -> NodesView<'a, N> {
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
