use crate::bounding_box::BoundingBox;
use crate::node::Node;

#[derive(Debug)]
pub struct RTree<T, const N: usize> {
    root: Node<T, N>,
    capacity: usize,
    length: usize,
}

impl<T, const N: usize> RTree<T, N> {
    const DISK_PAGE_SIZE_IN_BYTES: usize = 4096;
    const DISK_PAGE_SIZE: usize = 8 * Self::DISK_PAGE_SIZE_IN_BYTES;
    const DEFAULT_T_SIZE: usize = std::mem::size_of::<T>();
    const DEFAULT_CAPACITY: usize = Self::DISK_PAGE_SIZE / Self::DEFAULT_T_SIZE;

    /// Initialize a new R-Tree with a default node capacity optimized for disk pages.
    pub fn new() -> RTree<T, N> {
        RTree {
            root: Node::new_node(vec![]),
            capacity: Self::DEFAULT_CAPACITY,
            length: 0,
        }
    }

    /// Initialize a new R-Tree with a custom node capacity.
    pub fn with_capacity(capacity: usize) -> RTree<T, N> {
        RTree {
            root: Node::new_node(vec![]),
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
        if let Some(old) = self.root.insert(bbox, data, self.capacity) {
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
    pub fn search<'a, 'b>(&'a self, bbox: &'b BoundingBox<N>) -> impl Iterator<Item=(&BoundingBox<N>, &T)>
        where 'b : 'a {
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
