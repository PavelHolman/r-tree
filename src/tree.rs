use std::mem::size_of;

use crate::bounding_box::BoundingBox;
use crate::node::Node;
use crate::spatial_indexable::SpatialIndexable;

#[derive(Debug)]
pub struct RTree<Item>
    where Item: SpatialIndexable, {
    root: Node<Item>,
    count: usize,
    max_size: usize,
}

impl<Item> RTree<Item>
    where Item: SpatialIndexable, {
    const LEAF_SIZE_IN_BYTES: usize = 2048;
    const TYPE_SIZE_IN_BYTES: usize = size_of::<Item>();

    pub fn new() -> Self {
        Self {
            root: Node::new(),
            count: 0,
            max_size: Self::LEAF_SIZE_IN_BYTES / Self::TYPE_SIZE_IN_BYTES,
        }
    }

    pub fn with_node_capacity(capacity: usize) -> Self {
        Self {
            root: Node::new(),
            count: 0,
            max_size: capacity,
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    /// Insert an item into the tree.
    /// References to items intersecting the inserted item are returned.
    pub fn insert(&mut self, item: Item) -> impl Iterator<Item=&Item> {
        self.count += 1;
        self.root.insert(BoundingBox::<Item>::from_item(&item), item)
    }

    /// Search for items in the tree in the given bounds.
    /// References to found items are returned.
    pub fn search(&self, bound: &BoundingBox<Item>) -> impl Iterator<Item=&Item> {
        self.root.search(bound)
    }

    /// Remove items from the tree in the given bounds.
    /// Removed items are returned.
    pub fn remove(&mut self, bound: &BoundingBox<Item>) -> impl Iterator<Item=Item> {
        let removed_items: Vec<Item> = self.root.remove(bound).collect();
        self.count -= removed_items.len();
        removed_items.into_iter()
    }
}
