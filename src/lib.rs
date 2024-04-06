use crate::bounding_box::BoundingBox;
use crate::example_object::ExampleObject;
use crate::spatial_indexable::SpatialIndexable;
use crate::tree::RTree;

mod bounding_box;
mod spatial_indexable;
mod utils;
mod node;
mod tree;
mod example_object;


#[test]
pub fn test() {
    let mut db = RTree::new();

    let obj1 = ExampleObject { width: 10., length: 20., x: 10., y: 5. };
    let obj2 = ExampleObject { width: 5., length: 10., x: 25., y: 5. };

    db.insert([obj1, obj2]);

    let objects: Vec<_> = db.search(BoundingBox::from_ranges([(0., 40.), (0., 40.)])).collect();

    assert_eq!(objects.len(), 2);

    println!("{:?}", db);
    println!("{:?}", objects);
}
