use itertools::Itertools;

use crate::bounding_box::BoundingBox;
use crate::iterators::{LeafIter, NearestNeighborIter, SearchIter};

trait BBoxable<const N: usize> {
    fn bbox(&self) -> BoundingBox<N>;
}

impl<T, const N: usize> BBoxable<N> for Leaf<T, N> {
    fn bbox(&self) -> BoundingBox<N> {
        self.bbox
    }
}

impl<T, const N: usize> BBoxable<N> for Node<T, N> {
    fn bbox(&self) -> BoundingBox<N> {
        self.bbox
    }
}

impl<T, const N: usize> BBoxable<N> for Nodes<T, N> {
    fn bbox(&self) -> BoundingBox<N> {
        match self {
            Nodes::Leaves(leaves) => BoundingBox::union(&leaves.iter().map(|leaf| &leaf.bbox).collect_vec()),
            Nodes::Inners(nodes) => BoundingBox::union(&nodes.iter().map(|node| &node.bbox).collect_vec()),
        }
    }
}

impl<T, BB: BBoxable<N>, const N: usize> BBoxable<N> for Vec<BB> {
    fn bbox(&self) -> BoundingBox<N> {
        BoundingBox::union(&self.iter().map(|bb| bb.bbox()).collect_vec().as_ref())
    }
}

// TODO: Only cache this, not in memory here.
pub(crate) struct Leaf<T, const N: usize> {
    pub(crate) bbox: BoundingBox<N>,
    pub(crate) data: T,
}

pub(crate) struct Node<T, const N: usize> {
    pub(crate) bbox: BoundingBox<N>,
    pub(crate) nodes: Nodes<T, N>,
}

// TODO: Single ID instead of Leaf vec here.
pub(crate) enum Nodes<T, const N: usize> {
    Leaves(Vec<Leaf<T, N>>),
    Inners(Vec<Node<T, N>>),
}

pub(crate) struct Root<T, const N: usize> {
    pub(crate) nodes: Nodes<T, N>,
}

/// Defines the policy for splitting nodes when they reach the maximum capacity.
/// The policy can be one of:
/// - Half: Split the nodes in half in order of insertion.
/// - Random: Split the nodes randomly.
/// - Greedy: Split the two furthest nodes and the rest is added to one of them based on area increase.
/// - BruteForce: Split the nodes such that the area is minimal by trying all possibilities.
#[derive(Debug)]
pub enum SplitPolicy {
    Half,
    Random,
    Greedy,
    BruteForce,
}
// TODO: Move to R-Tree, add config for the whole tree. (policy, leaf/node capacity, etc.)

impl<T, const N: usize> Root<T, N> {
    pub(crate) fn insert(&mut self, bbox: BoundingBox<N>, data: T, capacity: usize, policy: &SplitPolicy) -> Option<T> {
        if let Some(old) = self.nodes.insert(bbox, data) {
            return Some(old);
        }

        if match &self.nodes {
            Nodes::Leaves(_) => panic!("Leaves shouldn't be directly linked to root."),
            Nodes::Inners(nodes) => nodes.len()
        } > capacity {
            let other = Nodes::<T, N>::split_nodes(&mut self.nodes, &policy);

            unsafe {
                let nodes = std::ptr::read(&self.nodes);
                let nodes = Nodes::Inners(vec![
                    Node { bbox: nodes.bbox(), nodes },
                    Node { bbox: other.bbox(), nodes: Nodes::Inners(other) }]);

                std::ptr::write(&mut self.nodes, nodes);
            }


            // TODO splitter
        }

        None
    }
}

impl<T, const N: usize> Nodes<T, N> {
    fn insert(&mut self, bbox: BoundingBox<N>, data: T) -> Option<T> {
        let capacity = 8; // todo
        let policy = SplitPolicy::Greedy; // todo

        match &mut self {
            Nodes::Leaves(leaves) => {
                if let Some(leaf) = leaves.iter_mut().find(|leaf| leaf.bbox == bbox) {
                    return Some(std::mem::replace(&mut leaf.data, data));
                }
                leaves.push(Leaf { bbox, data });
                // There is no need for fixing since the tree is always created with at least one inner node.
                None
            }
            Nodes::Inners(nodes) => {
                let best_node = Self::choose_best_node(nodes, &bbox);
                if let Some(old_value) = best_node.nodes.insert(bbox, data) {
                    return Some(old_value);
                }

                if best_node.nodes.len() > capacity {
                    let other_nodes = Self::split_nodes(best_node, &policy);
                    nodes.push(Node { bbox: other_nodes.bbox(), nodes: other_nodes });
                }
                None
            }
        }
    }

    fn split_nodes(nodes: &mut Vec<impl BBoxable<N>>, policy: &SplitPolicy) -> Vec<impl BBoxable<N>>
    {
        use rand::Rng;
        use rand::seq::SliceRandom;

        match policy {
            SplitPolicy::Random => {
                nodes.shuffle(&mut rand::thread_rng());
                nodes.split_off(rand::thread_rng().gen_range(1..nodes.len()))
            }
            SplitPolicy::Half => {
                nodes.split_off(nodes.len() / 2)
            }
            SplitPolicy::Greedy => {
                let ((i, j), _) = nodes.iter().enumerate()
                    .flat_map(|(i, l)| nodes.iter().enumerate().skip(i + 1)
                        .map(move |(j, r)| ((i, j), l.bbox().distance(&r.bbox()))))
                    .min_by(|(_, d1), (_, d2)| d1.total_cmp(d2))
                    .expect("Should be non empty, because splitting node without at least 2 elements makes no sense.");

                let mut left = vec![nodes.swap_remove(usize::max(i, j))];
                let mut right = vec![nodes.swap_remove(usize::min(i, j))];

                for i in (0..nodes.len()).rev() {
                    let left_distance = left[0].bbox().distance(&nodes[i].bbox());
                    let right_distance = right[0].bbox().distance(&nodes[i].bbox());

                    if left_distance < right_distance {
                        left.push(nodes.swap_remove(i));
                    } else {
                        right.push(nodes.swap_remove(i));
                    }
                }

                *nodes = left;
                right
            }
            SplitPolicy::BruteForce => {
                let (mut permutation, k) =
                    (1..(nodes.len() - 1))
                        .flat_map(|k| {
                            (0..nodes.len())
                                .permutations(nodes.len())
                                .map(move |permutation| (permutation, k))
                        })
                        .min_by_key(|(permutation, k)| {
                            let (left_indices, right_indices) = permutation.split_at(*k);
                            let left = left_indices.iter().map(|&i| &nodes[i]);
                            let right = right_indices.iter().map(|&i| &nodes[i]);

                            let left_area = BoundingBox::union(&left.map(|node| &node.bbox()).collect_vec()).area();
                            let right_area = BoundingBox::union(&right.map(|node| &node.bbox()).collect_vec()).area();
                            left_area + right_area
                        })
                        .expect("Should be non empty, because splitting node without at least 2 elements makes no sense.");

                permutation.split_off(k)
                    .iter()
                    .sorted()
                    .rev()
                    .map(|i| nodes.swap_remove(*i))
                    .collect_vec()
            }
        }
    }

    fn choose_best_node<'a>(nodes: &'a mut Vec<Node<T, N>>, bbox: &BoundingBox<N>) -> &'a mut Node<T, N> {
        nodes
            .iter_mut()
            .min_by_key(|node|
                (BoundingBox::union(&[&node.bbox, bbox]).area() - node.bbox.area(),
                 if let Nodes::inner(nodes) = &node.nodes { nodes.len() } else { 1 }))
            .unwrap_or(|| {
                nodes.push(Node { bbox: bbox.clone(), nodes: Nodes::Inners(vec![]) });
                return nodes.first_mut().unwrap();
            })
    }

    pub(crate) fn iter(&self) -> LeafIter<T, N> {
        LeafIter::new(self)
    }

    pub(crate) fn search<'a>(&'a self, bbox: &'a BoundingBox<N>) -> SearchIter<T, N> {
        SearchIter::new(self, bbox)
    }

    pub(crate) fn nearest<'a>(&'a self, bbox: &'a BoundingBox<N>) -> NearestNeighborIter<T, N> {
        NearestNeighborIter::new(self, bbox)
    }
}
