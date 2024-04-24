use crate::bounding_box::BoundingBox;
use crate::node::{Node, SplitPolicy};

/// R-Tree data structure.
/// Provides insertion, search and nearest neighbor search.
/// These operation are logarithmic in the number of elements in the tree.
/// The tree is generic over the data type T and the compile time dimension N of the bounding boxes.
#[derive(Debug)]
pub struct RTree<T, const N: usize> {
    root: Node<T, N>,
    split_policy: SplitPolicy,
    capacity: usize,
    length: usize,
}

impl<T, const N: usize> RTree<T, N> {
    const DISK_PAGE_SIZE: usize = 4096;
    const NODE_SIZE: usize = std::mem::size_of::<Node<T, N>>();
    const DEFAULT_CAPACITY: usize = Self::DISK_PAGE_SIZE / Self::NODE_SIZE;
    const DEFAULT_SPLIT_POLICY: SplitPolicy = if Self::DEFAULT_CAPACITY < 8 { SplitPolicy::BruteForce } else { SplitPolicy::Greedy };

    /// Initialize a new R-Tree with a default node capacity optimized for disk pages.
    pub fn new() -> RTree<T, N> {
        RTree {
            root: Node::new_node(vec![]),
            split_policy: Self::DEFAULT_SPLIT_POLICY,
            capacity: Self::DEFAULT_CAPACITY,
            length: 0,
        }
    }

    /// Initialize a new R-Tree with a custom node capacity.
    pub fn with_capacity(capacity: usize) -> RTree<T, N> {
        if capacity < 2 {
            panic!("Node capacity has to be at least 2!")
        }
        RTree {
            root: Node::new_node(vec![]),
            split_policy: if capacity < 8 { SplitPolicy::BruteForce } else { SplitPolicy::Greedy },
            capacity,
            length: 0,
        }
    }

    /// Initialize a new R-Tree with a custom split policy.
    pub fn with_split_policy(split_policy: SplitPolicy) -> RTree<T, N> {
        RTree {
            root: Node::new_node(vec![]),
            split_policy,
            capacity: Self::DEFAULT_CAPACITY,
            length: 0,
        }
    }

    /// Initialize a new R-Tree with a custom node capacity and split policy.
    pub fn with_capacity_and_split_policy(capacity: usize, split_policy: SplitPolicy) -> RTree<T, N> {
        if capacity < 2 {
            panic!("Node capacity has to be at least 2!")
        }
        RTree {
            root: Node::new_node(vec![]),
            split_policy,
            capacity,
            length: 0,
        }
    }

    /// Get the number of elements in the tree.
    pub fn len(&self) -> usize {
        self.length
    }

    /// Insert a new bounding box with data into the tree.
    /// If there was an old value in this exact bounding box, it is returned.
    pub fn insert(&mut self, bbox: BoundingBox<N>, data: T) -> Option<T> {
        if let Some(old) = self.root.insert(bbox, data, self.capacity, &self.split_policy) {
            Some(old)
        } else {
            self.length += 1;
            None
        }
    }

    /// Sequentially iterate over all elements in the tree.
    pub fn iter(&self) -> impl Iterator<Item=(&BoundingBox<N>, &T)> {
        self.root.iter()
    }

    /// Search for all elements in the tree that intersect with the given bounding box.
    /// This search is optimized such that it only searches in the nodes that intersect with the given bounding box.
    /// Only the elements that are fully contained in the given bounding box are returned.
    pub fn search<'a>(&'a self, bbox: &'a BoundingBox<N>) -> impl Iterator<Item=(&BoundingBox<N>, &T)> {
        self.root.search(bbox)
    }

    /// Search for the nearest elements to the given bounding box.
    /// By default, all elements are iterated, if you only need the k nearest elements, use `nearest(bbox).take(k)`.
    /// The elements are returned in order of distance to the given bounding box, 
    /// but the order of elements inside the given bounding box is not defined.
    pub fn nearest<'a>(&'a self, bbox: &'a BoundingBox<N>) -> impl Iterator<Item=(&BoundingBox<N>, &T)> {
        self.root.nearest(bbox)
    }
}
