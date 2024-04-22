pub use crate::bounding_box::BoundingBox;
pub use crate::tree::RTree;

mod bounding_box;
mod node;
mod tree;

#[test]
fn test() {
    let mut db = RTree::with_capacity(4);

    type BBox = BoundingBox<2>;

    let (p0, m0) = (BBox::new([(0., 0.), (0., 0.)]), 0);
    let (p1, m1) = (BBox::new([(0., 0.), (1., 1.)]), 1);
    let (p2, m2) = (BBox::new([(0., 0.), (2., 2.)]), 2);
    let (p3, m3) = (BBox::new([(0., 0.), (3., 3.)]), 3);

    db.insert(p0, m0);
    db.insert(p1, m1);
    db.insert(p2, m2);
    db.insert(p3, m3);

    let bound = BBox::new([(0., 0.), (0., 3.)]);

    assert_eq!(db.iter().collect::<Vec<_>>(), vec![(&p0, &m0), (&p1, &m1), (&p2, &m2), (&p3, &m3)]);
    assert_eq!(db.search(&bound).collect::<Vec<_>>(), vec![(&p0, &m0), (&p1, &m1), (&p2, &m2), (&p3, &m3)]);

    let mut nearest = db.nearest(&p0);
    
    assert_eq!(nearest.next(), Some((&p0, &m0)));
    assert_eq!(nearest.next(), Some((&p1, &m1)));
    assert_eq!(nearest.next(), Some((&p2, &m2)));
    assert_eq!(nearest.next(), Some((&p3, &m3)));

    drop(nearest);
    
    let (p4, m4) = (BBox::new([(0., 0.), (4., 4.)]), 4);
    let (p5, m5) = (BBox::new([(0., 0.), (5., 5.)]), 5);
    let (p6, m6) = (BBox::new([(0., 0.), (6., 6.)]), 6);
    let (p7, m7) = (BBox::new([(0., 0.), (7., 7.)]), 7);
    let (p8, m8) = (BBox::new([(0., 0.), (8., 8.)]), 8);
    let (p9, m9) = (BBox::new([(0., 0.), (9., 9.)]), 9);
    
    db.insert(p4, m4);
    db.insert(p5, m5);
    db.insert(p6, m6);
    db.insert(p7, m7);
    db.insert(p8, m8);
    db.insert(p9, m9);
    
    println!("{:?}", db);
}
