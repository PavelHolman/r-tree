pub use crate::bounding_box::BoundingBox;
pub use crate::tree::RTree;

mod bounding_box;
mod node;
mod tree;

#[test]
fn test() {
    let mut db = RTree::with_capacity(4);

    db.insert(BoundingBox::new([(10., 20.), (20., 30.)]), "Hi!".to_owned());
    db.insert(BoundingBox::new([(15., 25.), (10., 20.)]), "Hello!".to_owned());
    db.insert(BoundingBox::new([(0., 15.), (0., 15.)]), "Here!".to_owned());
    db.insert(BoundingBox::new([(0., 40.), (0., 40.)]), "No!".to_owned());
    db.insert(BoundingBox::new([(80., 100.), (45., 55.)]), "Yes!".to_owned());
    db.insert(BoundingBox::new([(0., 0.), (0., 0.)]), "42!".to_owned());

    println!("{:?}", db);

    let bound = BoundingBox::new([(10., 30.), (10., 30.)]);

    let searched: Vec<_> = db.search(&bound).collect();

    assert_eq!(searched.len(), 2);

    println!("{:?}", searched);
    println!("{:?}", db);

    println!("{:?}", db.iter().collect::<Vec<_>>());

    println!("{:?}", db.nearest(&bound).take(1).collect::<Vec<_>>());
    println!("{:?}", db.nearest(&bound).take(2).collect::<Vec<_>>());
    println!("{:?}", db.nearest(&bound).take(3).collect::<Vec<_>>());
    println!("{:?}", db.nearest(&bound).take(4).collect::<Vec<_>>());
    println!("{:?}", db.nearest(&bound).take(5).collect::<Vec<_>>());
    println!("{:?}", db.nearest(&bound).take(6).collect::<Vec<_>>());
}
