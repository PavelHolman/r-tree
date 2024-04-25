use std::error::Error;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::bounding_box::BoundingBox;
use crate::node::Node;
use crate::tree_view::RootView;

pub type T = usize;

/// Defines the policy for splitting nodes when they reach the maximum capacity.
/// The policy can be one of:
/// - Half: Split the nodes in half in order of insertion.
/// - Random: Split the nodes randomly.
/// - Greedy: Split the two furthest nodes and the rest is added to one of them based on area increase.
/// - BruteForce: Split the nodes such that the area is minimal by trying all possibilities.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum SplitPolicy {
    Half,
    Random,
    Greedy,
    BruteForce,
}

/// R-Tree data structure.
/// Provides insertion, search and nearest neighbor search.
/// These operation are logarithmic in the number of elements in the tree.
/// The tree is generic over the data type T and the compile time dimension N of the bounding boxes.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct RTree<const N: usize> {
    pub(crate) root: Node<N>,
    split_policy: SplitPolicy,
    capacity: usize,
    length: usize,
}

impl<const N: usize> Default for RTree<N> {
    fn default() -> Self {
        Self {
            root: Node::new_node(vec![]),
            split_policy: Self::DEFAULT_SPLIT_POLICY,
            capacity: Self::DEFAULT_CAPACITY,
            length: 0,
        }
    }
}

impl<const N: usize> RTree<N> {
    const DISK_PAGE_SIZE: usize = 4096;
    const NODE_SIZE: usize = std::mem::size_of::<Node<N>>();
    const DEFAULT_CAPACITY: usize = Self::DISK_PAGE_SIZE / Self::NODE_SIZE;
    const DEFAULT_SPLIT_POLICY: SplitPolicy = if Self::DEFAULT_CAPACITY < 8 { SplitPolicy::BruteForce } else { SplitPolicy::Greedy };

    /// Initialize a new R-Tree with a default node capacity optimized for disk pages.
    pub fn new() -> RTree<N> {
        Self::with_capacity_and_split_policy(Self::DEFAULT_CAPACITY, Self::DEFAULT_SPLIT_POLICY)
    }

    /// Initialize a new R-Tree with a custom node capacity.
    pub fn with_capacity(capacity: usize) -> RTree<N> {
        Self::with_capacity_and_split_policy(capacity, if capacity < 8 { SplitPolicy::BruteForce } else { SplitPolicy::Greedy })
    }

    /// Initialize a new R-Tree with a custom split policy.
    pub fn with_split_policy(split_policy: SplitPolicy) -> RTree<N> {
        Self::with_capacity_and_split_policy(Self::DEFAULT_CAPACITY, split_policy)
    }

    /// Initialize a new R-Tree with a custom node capacity and split policy.
    pub fn with_capacity_and_split_policy(capacity: usize, split_policy: SplitPolicy) -> RTree<N> {
        if capacity < 2 {
            panic!("Node capacity has to be at least 2!")
        }
        RTree {
            split_policy,
            capacity,
            ..Default::default()
        }
    }

    pub fn view<'a, 'b>(&'b self) -> RootView<'a, N>
        where 'b: 'a {
        RootView::new(&self)
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

    pub fn save(&self, path: &Path) -> Result<(), impl Error> {
        let mut file = std::fs::File::create(path)?;
        bincode::serialize_into(&mut file, &self)
    }

    pub fn load(path: &Path) -> Result<Self, impl Error> {
        let file = std::fs::File::open(path)?;
        bincode::deserialize_from(file)
    }
}
