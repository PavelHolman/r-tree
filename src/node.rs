use crate::bounding_box::Bound;
use crate::point::SpatialIndex;

#[derive(Debug)]
enum Node<K, V>
    where K: SpatialIndex {
    Node(TreeNode<K, V>),
    Leaf((K, V)),
}

#[derive(Debug)]
pub(crate) struct TreeNode<K, V>
    where K: SpatialIndex {
    children: Vec<Node<K, V>>,
}

impl<K, V> TreeNode<K, V>
    where K: SpatialIndex {
    pub(crate) fn new() -> Self { Self { children: vec![] } }

    pub(crate) fn insert(&mut self, key: K, value: V) -> Option<V> {
        todo!();
    }

    pub(crate) fn search(&self, bound: &impl Bound) -> impl Iterator<Item=&(K, V)> {
        todo!();
        vec![].iter()
    }

    pub(crate) fn remove(&mut self, bound: &impl Bound) -> impl Iterator<Item=(K, V)> {
        todo!();
        vec![].into_iter()
    }
}
