use crate::bounding_box::BoundingBox;
use crate::example_object::ExampleObject;
use crate::tree::RTree;

mod utils;
mod node;
mod example_object;
pub mod bounding_box;
pub mod spatial_indexable;
pub mod tree;


#[test]
fn test() {
    let mut db = RTree::new();

    let obj1 = ExampleObject { width: 10., length: 20., x: 10., y: 5. };
    let obj2 = ExampleObject { width: 5., length: 10., x: 25., y: 5. };

    db.insert([obj1, obj2]);

    let objects: Vec<_> = db.search(BoundingBox::from_ranges([(0., 40.), (0., 40.)])).collect();

    assert_eq!(objects.len(), 2);

    println!("{:?}", db);
    println!("{:?}", objects);
}
