pub use crate::bounding_box::{BoundingBox, Interval};
pub use crate::node::SplitPolicy;
pub use crate::tree::RTree;
pub use crate::tree_view::RootView;

mod bounding_box;
mod node;
mod tree;
mod iterators;
mod tree_view;

#[test]
fn test_view() {
    let mut db = RTree::with_capacity(2);
    db.insert(BoundingBox::from_intervals([Interval::new(0, 0).unwrap(), Interval::new(0, 0).unwrap()]).unwrap(), "Hi!".to_owned());
    db.insert(BoundingBox::from_intervals([Interval::new(1, 1).unwrap(), Interval::new(1, 1).unwrap()]).unwrap(), "Hello!".to_owned());
    db.insert(BoundingBox::from_intervals([Interval::new(2, 2).unwrap(), Interval::new(2, 2).unwrap()]).unwrap(), "Good morning!".to_owned());
    db.insert(BoundingBox::from_intervals([Interval::new(3, 3).unwrap(), Interval::new(3, 3).unwrap()]).unwrap(), "Good evening!".to_owned());
    db.insert(BoundingBox::from_intervals([Interval::new(4, 4).unwrap(), Interval::new(4, 4).unwrap()]).unwrap(), "Good afternoon!".to_owned());
    
    println!("{:?}", db.view())
}

#[test]
fn example_usage() {
    // Create new R-Tree with specified capacity or use RTree::new(); for optimal capacity for paging
    let mut db = RTree::with_capacity(4);

    db.insert(BoundingBox::from_intervals([Interval::new(0, 10).unwrap(), Interval::new(0, 5).unwrap()]).unwrap(), "Hi!".to_owned());
    db.insert(BoundingBox::from_intervals([Interval::new(10, 20).unwrap(), Interval::new(0, 5).unwrap()]).unwrap(), "Hello!".to_owned());

    // Get and collect all tree entries
    let all_entries = db.iter().collect::<Vec<_>>();
    println!("All entries:");
    println!("[Hi!, Hello!] : {:?}", all_entries);

    // Get and collect all entries in the bound
    let bound = BoundingBox::from_intervals([Interval::new(0, 20).unwrap(), Interval::new(0, 20).unwrap()]).unwrap();
    let entries_in_bound = db.search(&bound).collect::<Vec<_>>();
    println!("Entries in bound:");
    println!("[Hi!, Hello!] : {:?}", entries_in_bound);

    // Get all entries in order of distance to the point
    let point = BoundingBox::from_intervals([Interval::new(0, 5).unwrap(), Interval::new(0, 5).unwrap()]).unwrap();
    let mut nearest = db.nearest(&point);

    println!("Nearest iteration:");
    println!("Hi!           : {:?}", nearest.next());
    println!("Hello!        : {:?}", nearest.next());
    println!("-             : {:?}", nearest.next());

    // Get only k nearest entries using Iterator::take(k)
    let k = 1;
    let nearest_k_entries = db.nearest(&point).take(k).collect::<Vec<_>>();
    println!("Nearest {k} entries:");
    println!("[Hi!]         : {:?}", nearest_k_entries);
}

#[test]
fn test() {
    let mut db = RTree::with_capacity_and_split_policy(4, SplitPolicy::BruteForce);

    type BBox = BoundingBox<2>;

    let (p0, m0) = (BBox::from_intervals([Interval::new(0, 0).unwrap(), Interval::new(0, 0).unwrap()]).unwrap(), 0);
    let (p1, m1) = (BBox::from_intervals([Interval::new(0, 0).unwrap(), Interval::new(1, 1).unwrap()]).unwrap(), 1);
    let (p2, m2) = (BBox::from_intervals([Interval::new(0, 0).unwrap(), Interval::new(2, 2).unwrap()]).unwrap(), 2);
    let (p3, m3) = (BBox::from_intervals([Interval::new(0, 0).unwrap(), Interval::new(3, 3).unwrap()]).unwrap(), 3);

    db.insert(p0, m0);
    db.insert(p1, m1);
    db.insert(p2, m2);
    db.insert(p3, m3);

    let bound = BBox::from_intervals([Interval::new(0, 0).unwrap(), Interval::new(0, 3).unwrap()]).unwrap();

    assert_eq!(db.iter().collect::<Vec<_>>(), vec![(&p0, &m0), (&p1, &m1), (&p2, &m2), (&p3, &m3)]);
    assert_eq!(db.search(&bound).collect::<Vec<_>>(), vec![(&p0, &m0), (&p1, &m1), (&p2, &m2), (&p3, &m3)]);

    let mut nearest = db.nearest(&p0);

    assert_eq!(nearest.next(), Some((&p0, &m0)));
    assert_eq!(nearest.next(), Some((&p1, &m1)));
    assert_eq!(nearest.next(), Some((&p2, &m2)));
    assert_eq!(nearest.next(), Some((&p3, &m3)));

    drop(nearest);

    let (p4, m4) = (BBox::from_intervals([Interval::new(0, 0).unwrap(), Interval::new(4, 4).unwrap()]).unwrap(), 4);
    let (p5, m5) = (BBox::from_intervals([Interval::new(0, 0).unwrap(), Interval::new(5, 5).unwrap()]).unwrap(), 5);
    let (p6, m6) = (BBox::from_intervals([Interval::new(0, 0).unwrap(), Interval::new(6, 6).unwrap()]).unwrap(), 6);
    let (p7, m7) = (BBox::from_intervals([Interval::new(0, 0).unwrap(), Interval::new(7, 7).unwrap()]).unwrap(), 7);
    let (p8, m8) = (BBox::from_intervals([Interval::new(0, 0).unwrap(), Interval::new(8, 8).unwrap()]).unwrap(), 8);
    let (p9, m9) = (BBox::from_intervals([Interval::new(0, 0).unwrap(), Interval::new(9, 9).unwrap()]).unwrap(), 9);

    db.insert(p4, m4);
    db.insert(p5, m5);
    db.insert(p6, m6);
    db.insert(p7, m7);
    db.insert(p8, m8);
    db.insert(p9, m9);

    println!("{:?}", db);
}
