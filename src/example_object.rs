use crate::spatial_indexable::SpatialIndexable;

#[derive(Debug)]
pub(crate) struct ExampleObject {
    pub(crate) width: f64,
    pub(crate) length: f64,
    pub(crate) x: f64,
    pub(crate) y: f64,
}

impl SpatialIndexable<2> for ExampleObject {
    type Coord = f64;

    fn min(&self) -> [Self::Coord; 2] {
        [self.x, self.y]
    }

    fn max(&self) -> [Self::Coord; 2] {
        [self.x + self.width, self.y + self.length]
    }
}
