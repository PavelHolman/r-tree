use crate::spatial_indexable::SpatialIndexable;
use crate::utils::Unzip;

pub struct BoundingBox<Coord, const N: usize>
    where Coord: PartialOrd + Copy {
    min: [Coord; N],
    max: [Coord; N],
}

impl<Coord, const N: usize> BoundingBox<Coord, N>
    where Coord: PartialOrd + Copy {
    pub fn from_ranges(bounds: [(Coord, Coord); N]) -> Self {
        let (min, max) = bounds.unzip();
        Self { min, max }
    }

    pub fn from_bounds(min: [Coord; N], max: [Coord; N]) -> Self {
        Self { min, max }
    }
}

impl<Coord, const N: usize> SpatialIndexable<N> for BoundingBox<Coord, N>
    where Coord: PartialOrd + Copy {
    type Coord = Coord;

    fn min(&self) -> [Self::Coord; N]
    {
        self.min
    }

    fn max(&self) -> [Self::Coord; N] {
        self.max
    }
}
