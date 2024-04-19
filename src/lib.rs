use crate::bounding_box::BoundingBox;
use crate::tree::RTree;

mod bounding_box;
mod node;
mod tree;

#[test]
fn test() {
    let mut db = RTree::new();

    db.insert(BoundingBox::new([(10., 20.), (20., 30.)]), "Hi!".to_owned());
    db.insert(BoundingBox::new([(5., 15.), (10., 20.)]), "Hello!".to_owned());

    let bound = BoundingBox::new([(0., 40.), (0., 40.)]);

    let searched: Vec<_> = db.search(&bound).collect();

    assert_eq!(searched.len(), 2);

    println!("{:?}", db);
    println!("{:?}", searched);

    let removed: Vec<_> = db.remove(&bound).collect::<Vec<_>>();

    println!("{:?}", db);
    println!("{:?}", removed);

    assert_eq!(db.len(), 0);

    println!("{:?}", db);
}
