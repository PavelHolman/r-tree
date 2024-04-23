use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval {
    min: u64,
    max: u64,
}

impl Interval {
    pub fn new(min: u64, max: u64) -> Self {
        if min > max {
            panic!("Interval min has to be less than max.");
        }
        Self { min, max }
    }

    pub fn min(&self) -> u64 {
        self.min
    }

    pub fn max(&self) -> u64 {
        self.max
    }
}

/// Bounding box implementation for R-Tree.
/// The bounds are stored as a tuple of (min, max) for each dimension.
/// The intervals are inclusive on both sides.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox<const N: usize> {
    pub bounds: [Interval; N],
}

impl<const N: usize> BoundingBox<N> {
    /// Create a new bounding box from the given points.
    pub fn from_points(min: [u64; N], max: [u64; N]) -> Self {
        if N <= 0 {
            panic!("Bounding box dimension has to be positive!");
        }
        Self {
            bounds: min.iter().zip(max.iter())
                .map(|(min, max)| Interval::new(*min, *max))
                .collect_vec()
                .try_into()
                .expect("Why would this fail even.")
        }
    }

    /// Create a new bounding box from the given intervals.
    pub fn from_intervals(intervals: [Interval; N]) -> Self {
        if N <= 0 {
            panic!("Bounding box dimension has to be positive!");
        }
        Self { bounds: intervals }
    }

    /// Create a new bounding box intersection of all given bounding boxes.
    pub fn intersection(boxes: &[&BoundingBox<N>]) -> Option<Self> {
        let mut bounds = [Interval { min: 0, max: 0 }; N];

        for i in 0..N {
            bounds[i].min = boxes.iter().map(|bbox| bbox.bounds[i].min).max()
                .unwrap_or(0);
            bounds[i].max = boxes.iter().map(|bbox| bbox.bounds[i].max).min()
                .unwrap_or(0);

            if bounds[i].min > bounds[i].max {
                return None;
            }
        }

        Some(Self { bounds })
    }

    /// Create a new bounding box union over all given bounding boxes.
    pub fn union(boxes: &[&BoundingBox<N>]) -> Self {
        let mut bounds = [Interval { min: 0, max: 0 }; N];

        for i in 0..N {
            bounds[i].min = boxes.iter().map(|bbox| bbox.bounds[i].min).min()
                .unwrap_or(0);
            bounds[i].max = boxes.iter().map(|bbox| bbox.bounds[i].max).max()
                .unwrap_or(0);
        }
        Self { bounds }
    }

    /// Get the area of the bounding box.
    pub fn area(&self) -> u64 {
        self.bounds.iter()
            .map(|a| u64::max(1, a.max - a.min)) // max 1 so that points on a line don't have 0 area
            .product()
    }

    /// Get the distance to the other bounding box.
    pub fn distance(&self, bbox: &BoundingBox<N>) -> f64 {
        (self.bounds.iter().zip(bbox.bounds.iter())
            .fold(0, |acc, (a, b)| acc + u64::pow(u64::max(b.min - a.min, b.max - a.max), 2)) as f64)
            .sqrt()
    }

    /// Check if the bounding box fully contains the other bounding box.
    pub fn contains(&self, bbox: &BoundingBox<N>) -> bool {
        self.bounds.iter().zip(bbox.bounds.iter())
            .all(|(a, b)| a.min <= b.min && b.max <= a.max)
    }

    /// Check if the bounding box intersects with the other bounding box.
    pub fn intersects(&self, bbox: &BoundingBox<N>) -> bool {
        self.bounds.iter().zip(bbox.bounds.iter())
            .any(|(a, b)| a.max >= b.min && b.max >= a.min)
    }
}
