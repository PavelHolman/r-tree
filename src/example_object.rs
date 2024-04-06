use generic_array::{arr, GenericArray, typenum};

use crate::spatial_indexable::SpatialIndexable;

#[derive(Debug)]
pub(crate) struct ExampleObject {
    pub(crate) width: f64,
    pub(crate) length: f64,
    pub(crate) x: f64,
    pub(crate) y: f64,
}

impl SpatialIndexable for ExampleObject {
    type Coord = f64;
    type Dim = typenum::U2;

    fn min(&self) -> GenericArray<Self::Coord, Self::Dim> {
        arr![self.x, self.y]
    }

    fn max(&self) -> GenericArray<Self::Coord, Self::Dim> {
        arr![self.x + self.width, self.y + self.length]
    }
}
