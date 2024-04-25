use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::BoundingBox;
use crate::node::Node;
use crate::tree::T;

/// Iterator over all leaf nodes in the tree.
pub struct LeafIter<'a, const N: usize> {
    nodes_to_visit: Vec<&'a Node<N>>,
}

impl<'a, const N: usize> LeafIter<'a, N> {
    pub(crate) fn new(node: &'a Node<N>) -> Self {
        LeafIter { nodes_to_visit: vec![node] }
    }
}

impl<'a, const N: usize> Iterator for LeafIter<'a, N> {
    type Item = (&'a BoundingBox<N>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.nodes_to_visit.pop() {
            match &node.data {
                crate::node::Data::Leaf(leaf) => return Some((&node.bbox, &leaf)),
                crate::node::Data::Inner(nodes) => {
                    for child in nodes.iter().rev() {
                        self.nodes_to_visit.push(child);
                    }
                }
            }
        }
        None
    }
}

/// Iterator over all leaf nodes in the tree that are contained within the given bounding box.
pub struct SearchIter<'a, const N: usize> {
    nodes_to_visit: Vec<&'a Node<N>>,
    bbox: &'a BoundingBox<N>,
}

impl<'a, const N: usize> SearchIter<'a, N> {
    pub(crate) fn new(node: &'a Node<N>, bbox: &'a BoundingBox<N>) -> Self {
        SearchIter { nodes_to_visit: vec![node], bbox }
    }
}

impl<'a, const N: usize> Iterator for SearchIter<'a, N> {
    type Item = (&'a BoundingBox<N>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.nodes_to_visit.pop() {
            match &node.data {
                crate::node::Data::Leaf(leaf) => {
                    if self.bbox.contains(&node.bbox) {
                        return Some((&node.bbox, &leaf));
                    }
                }
                crate::node::Data::Inner(nodes) => {
                    for child in nodes.iter().rev() {
                        if self.bbox.intersects(&child.bbox) {
                            self.nodes_to_visit.push(child);
                        }
                    }
                }
            }
        }
        None
    }
}

/// Iterator over all leaf nodes in the tree in order of distance to the given bounding box.
pub struct NearestNeighborIter<'a, const N: usize> {
    nodes_to_visit: BinaryHeap<NearestNeighborNode<'a, N>>,
    bbox: &'a BoundingBox<N>,
}

impl<'a, const N: usize> NearestNeighborIter<'a, N> {
    pub(crate) fn new(node: &'a Node<N>, bbox: &'a BoundingBox<N>) -> Self {
        let distance = bbox.distance(&node.bbox);
        NearestNeighborIter { nodes_to_visit: BinaryHeap::from([NearestNeighborNode { distance, node }]), bbox }
    }
}

impl<'a, const N: usize> Iterator for NearestNeighborIter<'a, N> {
    type Item = (&'a BoundingBox<N>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(neighbor) = self.nodes_to_visit.pop() {
            match &neighbor.node.data {
                crate::node::Data::Leaf(leaf) => return Some((&neighbor.node.bbox, &leaf)),
                crate::node::Data::Inner(nodes) => {
                    for child in nodes.iter() {
                        let distance = self.bbox.distance(&child.bbox);
                        self.nodes_to_visit.push(NearestNeighborNode { distance, node: child });
                    }
                }
            }
        }
        None
    }
}

struct NearestNeighborNode<'a, const N: usize> {
    distance: f64,
    node: &'a Node<N>,
}

impl<'a, const N: usize> Eq for NearestNeighborNode<'a, N> {}

impl<'a, const N: usize> PartialEq<Self> for NearestNeighborNode<'a, N> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl<'a, const N: usize> PartialOrd<Self> for NearestNeighborNode<'a, N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl<'a, const N: usize> Ord for NearestNeighborNode<'a, N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.total_cmp(&other.distance)
            .reverse()
    }
}
