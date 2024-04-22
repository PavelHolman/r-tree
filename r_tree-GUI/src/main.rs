use r_tree::BoundingBox;
use r_tree::RTree;

fn main() {
    // TODO: Visualize the tree.
    let mut db = RTree::with_capacity(4);

    db.insert(BoundingBox::new([(0., 10.), (0., 5.)]), "Hi!".to_owned());
    db.insert(BoundingBox::new([(10., 20.), (0., 5.)]), "Hello!".to_owned());
    let bound = BoundingBox::new([(0., 20.), (0., 20.)]);

    println!("{:?}", db);
    println!("{:?}", db.iter().collect::<Vec<_>>());
    println!("{:?}", db.search(&bound).collect::<Vec<_>>());
    println!("{:?}", db.nearest(&bound).take(1).collect::<Vec<_>>());
    println!("{:?}", db.nearest(&bound).take(2).collect::<Vec<_>>());
}
