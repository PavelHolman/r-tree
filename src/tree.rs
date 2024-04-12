use std::mem::size_of;

use crate::bounding_box::Bound;
use crate::node::TreeNode;
use crate::point::SpatialIndex;

#[derive(Debug)]
pub struct RTree<K, V>
    where K: SpatialIndex, {
    root: TreeNode<K, V>,
    count: usize,
    max_size: usize,
}

impl<K, V> RTree<K, V>
    where K: SpatialIndex {
    const LEAF_SIZE_IN_BYTES: usize = 2048;
    const TYPE_SIZE_IN_BYTES: usize = size_of::<K>() + size_of::<V>();

    pub fn new() -> Self {
        Self {
            root: TreeNode::new(),
            count: 0,
            max_size: Self::LEAF_SIZE_IN_BYTES / Self::TYPE_SIZE_IN_BYTES,
        }
    }

    pub fn with_node_capacity(capacity: usize) -> Self {
        Self {
            root: TreeNode::new(),
            count: 0,
            max_size: capacity,
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    /// Insert an item into the tree.
    /// References to items intersecting the inserted item are returned.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.count += 1;
        self.root.insert(key, value)
    }

    /// Search for items in the tree in the given bounds.
    /// References to found items are returned.
    pub fn search(&self, bound: &impl Bound) -> impl Iterator<Item=&(K, V)> {
        self.root.search(bound)
    }

    /// Remove items from the tree in the given bounds.
    /// Removed items are returned.
    pub fn remove(&mut self, bound: &impl Bound) -> impl Iterator<Item=(K, V)> {
        let removed_items: Vec<_> = self.root.remove(bound).collect();
        self.count -= removed_items.len();
        removed_items.into_iter()
    }
}
