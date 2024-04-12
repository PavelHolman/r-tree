use crate::bounding_box::BoundingBox;
use crate::point::Point;
use crate::tree::RTree;

mod point;
mod bounding_box;
mod node;
mod tree;

#[test]
fn test() {
    let mut db = RTree::new();

    db.insert(Point::new([10., 20.]), "Hi!".to_owned());
    db.insert(Point::new([5., 10.]), "Hello!".to_owned());

    let bound = BoundingBox::from_bounds([(0., 40.), (0., 40.)]);

    let searched: Vec<_> = db.search(&bound).collect();

    assert_eq!(searched.len(), 2);

    println!("{:?}", db);
    println!("{:?}", searched);

    let removed: Vec<_> = db.remove(&bound).collect::<Vec<_>>();

    println!("{:?}", db);
    println!("{:?}", removed);

    assert_eq!(db.count(), 0);

    println!("{:?}", db);
}
