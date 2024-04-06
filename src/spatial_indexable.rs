use generic_array::{ArrayLength, GenericArray};

pub trait SpatialIndexable {
    type Coord: PartialOrd + Copy;
    type Dim: ArrayLength;

    fn min(&self) -> GenericArray<Self::Coord, Self::Dim>;
    fn max(&self) -> GenericArray<Self::Coord, Self::Dim>;
}
