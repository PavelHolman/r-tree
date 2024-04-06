use generic_array::GenericArray;

use crate::spatial_indexable::SpatialIndexable;
use crate::utils::Unzip;

pub struct BoundingBox<Item>
    where Item: SpatialIndexable {
    min: GenericArray<Item::Coord, Item::Dim>,
    max: GenericArray<Item::Coord, Item::Dim>,
}

impl<Item> BoundingBox<Item>
    where Item: SpatialIndexable {
    pub fn from_ranges(bounds: GenericArray<(Item::Coord, Item::Coord), Item::Dim>) -> Self {
        let (min, max) = bounds.unzip();
        Self { min, max }
    }

    pub fn from_bounds(min: GenericArray<Item::Coord, Item::Dim>, max: GenericArray<Item::Coord, Item::Dim>) -> Self {
        Self { min, max }
    }
}

impl<Item> SpatialIndexable for BoundingBox<Item>
    where Item: SpatialIndexable {
    type Coord = Item::Coord;
    type Dim = Item::Dim;

    fn min(&self) -> GenericArray<Self::Coord, Self::Dim> {
        self.min.clone()
    }

    fn max(&self) -> GenericArray<Self::Coord, Self::Dim> {
        self.max.clone()
    }
}
