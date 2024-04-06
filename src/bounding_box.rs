use crate::spatial_indexable::SpatialIndexable;
use crate::utils::Unzip;

pub struct BoundingBox<Coord, const Dim: usize>
    where Coord: PartialOrd + Copy {
    min: [Coord; Dim],
    max: [Coord; Dim],
}

impl<Coord, const Dim: usize> BoundingBox<Coord, Dim>
    where Coord: PartialOrd + Copy {
    pub fn from_ranges(bounds: [(Coord, Coord); Dim]) -> Self {
        let (min, max) = bounds.unzip();
        Self { min, max }
    }

    pub fn from_bounds(min: [Coord; Dim], max: [Coord; Dim]) -> Self {
        Self { min, max }
    }
}

impl<Coord, const Dim: usize> SpatialIndexable<Dim> for BoundingBox<Coord, Dim>
    where Coord: PartialOrd + Copy {
    type Coord = Coord;

    fn min(&self) -> [Self::Coord; Dim]
    {
        self.min
    }

    fn max(&self) -> [Self::Coord; Dim] {
        self.max
    }
}
