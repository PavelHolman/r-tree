/// Bounding box implementation for R-Tree.
/// The bounds are stored as a tuple of (min, max) for each dimension.
/// The intervals are inclusive on both sides.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox<const N: usize> {
    pub bounds: [(f64, f64); N],
}

impl<const N: usize> BoundingBox<N> {
    /// Create a new bounding box with the given bounds.
    pub fn new(bounds: [(f64, f64); N]) -> Self {
        if bounds.len() <= 0 {
            panic!("Bounding box dimension has to be positive.");
        }
        if bounds.iter().any(|(a, b)| a > b) {
            panic!("Bounding box min has to be less than max.");
        }
        Self { bounds }
    }

    /// Create a new bounding box intersection of all given bounding boxes.
    pub fn intersection(boxes: &[&BoundingBox<N>]) -> Option<Self> {
        let mut bounds = [(0., 0.); N];

        for i in 0..N {
            bounds[i].0 = boxes.iter().map(|bbox| bbox.bounds[i].0).fold(f64::MIN, f64::max);
            bounds[i].1 = boxes.iter().map(|bbox| bbox.bounds[i].1).fold(f64::MAX, f64::min);

            if bounds[i].0 > bounds[i].1 {
                return None;
            }
        }

        Some(Self { bounds })
    }

    /// Create a new bounding box union over all given bounding boxes.
    pub fn union(boxes: &[&BoundingBox<N>]) -> Self {
        let mut bounds = [(0., 0.); N];

        for i in 0..N {
            bounds[i].0 = boxes.iter().map(|bbox| bbox.bounds[i].0).fold(f64::MAX, f64::min);
            bounds[i].1 = boxes.iter().map(|bbox| bbox.bounds[i].1).fold(f64::MIN, f64::max);
        }
        Self { bounds }
    }

    /// Get the area of the bounding box.
    pub fn area(&self) -> u64 {
        self.bounds.iter()
            .map(|a| (a.1 - a.0) as u64)
            .product()
    }

    /// Get the distance to the other bounding box.
    pub fn distance(&self, bbox: &BoundingBox<N>) -> f64 {
        self.bounds.iter().zip(bbox.bounds.iter())
            .fold(0.0, |acc, (a, b)| acc + f64::powi(f64::max(b.0 - a.0, b.1 - a.1), 2))
            .sqrt()
    }

    /// Check if the bounding box fully contains the other bounding box.
    pub fn contains(&self, bbox: &BoundingBox<N>) -> bool {
        self.bounds.iter().zip(bbox.bounds.iter())
            .all(|(a, b)| a.0 <= b.0 && b.1 <= a.1)
    }

    /// Check if the bounding box intersects with the other bounding box.
    pub fn intersects(&self, bbox: &BoundingBox<N>) -> bool {
        self.bounds.iter().zip(bbox.bounds.iter())
            .any(|(a, b)| a.1 >= b.0 && b.1 >= a.0)
    }
}
