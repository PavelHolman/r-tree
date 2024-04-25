use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::BoundingBox;
use crate::node::{Node, Nodes};

/// Iterator over all leaf nodes in the tree.
pub struct LeafIter<'a, T, const N: usize> {
    nodes_to_visit: Vec<&'a Node<T, N>>,
    nodes: Nodes<T, N>,
}

impl<'a, T, const N: usize> LeafIter<'a, T, N> {
    pub(crate) fn new(node: &'a Node<T, N>) -> Self {
        LeafIter { nodes_to_visit: vec![node] }
    }
}

impl<'a, T, const N: usize> Iterator for LeafIter<'a, T, N> {
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
pub struct SearchIter<'a, T, const N: usize> {
    nodes_to_visit: Vec<&'a Node<T, N>>,
    bbox: &'a BoundingBox<N>,
}

impl<'a, T, const N: usize> SearchIter<'a, T, N> {
    pub(crate) fn new(node: &'a Node<T, N>, bbox: &'a BoundingBox<N>) -> Self {
        SearchIter { nodes_to_visit: vec![node], bbox }
    }
}

impl<'a, T, const N: usize> Iterator for SearchIter<'a, T, N> {
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
pub struct NearestNeighborIter<'a, T, const N: usize> {
    nodes_to_visit: BinaryHeap<NearestNeighborNode<'a, T, N>>,
    bbox: &'a BoundingBox<N>,
}

impl<'a, T, const N: usize> NearestNeighborIter<'a, T, N> {
    pub(crate) fn new(node: &'a Node<T, N>, bbox: &'a BoundingBox<N>) -> Self {
        let distance = bbox.distance(&node.bbox);
        NearestNeighborIter { nodes_to_visit: BinaryHeap::from([NearestNeighborNode { distance, node }]), bbox }
    }
}

impl<'a, T, const N: usize> Iterator for NearestNeighborIter<'a, T, N> {
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

struct NearestNeighborNode<'a, T, const N: usize> {
    distance: f64,
    node: &'a Node<T, N>,
}

impl<'a, T, const N: usize> Eq for NearestNeighborNode<'a, T, N> {}

impl<'a, T, const N: usize> PartialEq<Self> for NearestNeighborNode<'a, T, N> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl<'a, T, const N: usize> PartialOrd<Self> for NearestNeighborNode<'a, T, N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl<'a, T, const N: usize> Ord for NearestNeighborNode<'a, T, N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.total_cmp(&other.distance)
            .reverse()
    }
}
