use crate::{BoundingBox, RTree};
use crate::node::{Data, Node, Nodes};

pub struct LeafView<'a, T, const N: usize> {
    pub bbox: &'a BoundingBox<N>,
    pub data: &'a T,
}

pub struct NodeView<'a, T, const N: usize> {
    pub bbox: &'a BoundingBox<N>,
    pub nodes: NodesView<'a, T, N>,
}

pub enum NodesView<'a, T, const N: usize> {
    Leaf(LeafView<'a, T, N>),
    Inners(Vec<NodeView<'a, T, N>>),
}

pub struct RootView<'a, T, const N: usize> {
    pub nodes: NodesView<'a, T, N>,
}

impl<'a, T, const N: usize> RootView<'a, T, N> {
    pub(crate) fn new(tree: &'a mut RTree<T, N>) -> Self {
        RootView { nodes: Self::rec(&tree.root, &mut tree.nodes) }
    }

    fn rec(node: &'a Node<T, N>, nodes: &mut Nodes<T, N>) -> NodesView<'a, T, N> {
        match &node.data {
            Data::Leaf(leaf) => NodesView::Leaf(
                LeafView {
                    bbox: &node.bbox,
                    data: leaf,
                }),
            Data::Inner(inner_nodes) => {
                NodesView::Inners(
                    inner_nodes.iter().map(|node_id|
                        {
                            let node = nodes.get_node(node_id);
                            NodeView {
                                bbox: &node.bbox,
                                nodes: Self::rec(node, nodes),
                            }
                        }).collect()
                )
            }
        }
    }
}
