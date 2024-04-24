pub use crate::bounding_box::{BoundingBox, Interval};
pub use crate::node::SplitPolicy;
pub use crate::tree::RTree;

mod bounding_box;
mod node;
mod tree;
mod iterators;

#[test]
fn example_usage() {
    // Create new R-Tree with specified capacity or use RTree::new(); for optimal capacity for paging
    let mut db = RTree::with_capacity(4);

    db.insert(BoundingBox::from_intervals([Interval::new(0, 10), Interval::new(0, 5)]), "Hi!".to_owned());
    db.insert(BoundingBox::from_intervals([Interval::new(10, 20), Interval::new(0, 5)]), "Hello!".to_owned());

    // Get and collect all tree entries
    let all_entries = db.iter().collect::<Vec<_>>();
    println!("All entries:");
    println!("[Hi!, Hello!] : {:?}", all_entries);

    // Get and collect all entries in the bound
    let bound = BoundingBox::from_intervals([Interval::new(0, 20), Interval::new(0, 20)]);
    let entries_in_bound = db.search(&bound).collect::<Vec<_>>();
    println!("Entries in bound:");
    println!("[Hi!, Hello!] : {:?}", entries_in_bound);

    // Get all entries in order of distance to the point
    let point = BoundingBox::from_intervals([Interval::new(0, 5), Interval::new(0, 5)]);
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

    let (p0, m0) = (BBox::from_intervals([Interval::new(0, 0), Interval::new(0, 0)]), 0);
    let (p1, m1) = (BBox::from_intervals([Interval::new(0, 0), Interval::new(1, 1)]), 1);
    let (p2, m2) = (BBox::from_intervals([Interval::new(0, 0), Interval::new(2, 2)]), 2);
    let (p3, m3) = (BBox::from_intervals([Interval::new(0, 0), Interval::new(3, 3)]), 3);

    db.insert(p0, m0);
    db.insert(p1, m1);
    db.insert(p2, m2);
    db.insert(p3, m3);

    let bound = BBox::from_intervals([Interval::new(0, 0), Interval::new(0, 3)]);

    assert_eq!(db.iter().collect::<Vec<_>>(), vec![(&p0, &m0), (&p1, &m1), (&p2, &m2), (&p3, &m3)]);
    assert_eq!(db.search(&bound).collect::<Vec<_>>(), vec![(&p0, &m0), (&p1, &m1), (&p2, &m2), (&p3, &m3)]);

    let mut nearest = db.nearest(&p0);

    assert_eq!(nearest.next(), Some((&p0, &m0)));
    assert_eq!(nearest.next(), Some((&p1, &m1)));
    assert_eq!(nearest.next(), Some((&p2, &m2)));
    assert_eq!(nearest.next(), Some((&p3, &m3)));

    drop(nearest);

    let (p4, m4) = (BBox::from_intervals([Interval::new(0, 0), Interval::new(4, 4)]), 4);
    let (p5, m5) = (BBox::from_intervals([Interval::new(0, 0), Interval::new(5, 5)]), 5);
    let (p6, m6) = (BBox::from_intervals([Interval::new(0, 0), Interval::new(6, 6)]), 6);
    let (p7, m7) = (BBox::from_intervals([Interval::new(0, 0), Interval::new(7, 7)]), 7);
    let (p8, m8) = (BBox::from_intervals([Interval::new(0, 0), Interval::new(8, 8)]), 8);
    let (p9, m9) = (BBox::from_intervals([Interval::new(0, 0), Interval::new(9, 9)]), 9);

    db.insert(p4, m4);
    db.insert(p5, m5);
    db.insert(p6, m6);
    db.insert(p7, m7);
    db.insert(p8, m8);
    db.insert(p9, m9);

    println!("{:?}", db);
}
