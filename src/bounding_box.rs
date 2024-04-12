use crate::point::Point;

pub trait Bound {}

#[derive(Debug)]
pub struct BoundingBox<const N: usize> {
    bounds: [(f64, f64); N],
}

impl<const N: usize> Bound for BoundingBox<N> {}

impl<const N: usize> BoundingBox<N> {
    pub fn from_bounds(bounds: [(f64, f64); N]) -> Self {
        Self { bounds }
    }

    pub fn contains(&self, point: &Point<N>) -> bool {
        self.bounds.iter().zip(point.a.iter())
            .all(|((min, max), p)| min <= p && p <= max)
    }

    pub fn distance(&self, point: &Point<N>) -> f64 {
        self.bounds.iter().zip(point.a.iter())
            .fold(Some(0.0), |acc, ((min, max), p)| {
                acc.and_then(|acc| {
                    if p < min {
                        Some(acc + (min - p).powi(2))
                    } else if p > max {
                        Some(acc + (p - max).powi(2))
                    } else {
                        None
                    }
                })
            })
            .map_or(0.0, f64::sqrt)
    }
}
