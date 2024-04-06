pub trait SpatialIndexable<const Dim: usize> {
    type Coord: PartialOrd + Copy;

    fn min(&self) -> [Self::Coord; Dim];
    fn max(&self) -> [Self::Coord; Dim];
}
