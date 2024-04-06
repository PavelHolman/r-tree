pub trait SpatialIndexable<const N: usize> {
    type Coord: PartialOrd + Copy;

    fn min(&self) -> [Self::Coord; N];
    fn max(&self) -> [Self::Coord; N];
}
