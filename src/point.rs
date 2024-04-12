pub trait SpatialIndex {}

#[derive(Debug)]
pub struct Point<const N: usize> {
    pub(crate) a: [f64; N],
}

impl<const N: usize> SpatialIndex for Point<N> {}

impl<const N: usize> Point<N> {
    pub fn new(a: [f64; N]) -> Self {
        if a.len() <= 0 {
            panic!("Point dimension has to be positive.");
        }
        Self { a }
    }

    pub fn distance(&self, other: &Self) -> f64 {
        self.a.iter().zip(other.a.iter())
            .fold(0.0, |acc, (x, y)| acc + (x - y).powi(2))
            .sqrt()
    }
}
