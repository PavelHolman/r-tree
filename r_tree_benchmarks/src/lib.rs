use std::ops::Range;
use std::time::Instant;

use itertools::Itertools;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use r_tree::{BoundingBox, Interval, RTree};

fn generate_bboxes(seed: u64, n_intervals: usize, range: Range<u64>) -> Vec<BoundingBox<2>> {
    let mut rng = StdRng::seed_from_u64(seed);

    (0..n_intervals)
        .map(|_| {
            let x1 = rng.gen_range(range.clone());
            let x2 = rng.gen_range(x1..range.end);
            let y1 = rng.gen_range(range.clone());
            let y2 = rng.gen_range(y1..range.end);
            let bbox = BoundingBox::from_intervals(
                [Interval::new(x1, x2).unwrap(), Interval::new(y1, y2).unwrap()]).unwrap();
            return bbox;
        })
        .collect_vec()
}

#[test]
fn bench_tree_vs_seq() {
    let n_intervals = 1000;
    let n_searches = 100000;
    let insert_range = 0..100000;
    let search_range = 0..100;

    let mut tree = RTree::new();
    let mut seq = vec![];

    let insert = generate_bboxes(42, n_intervals, insert_range.clone());

    let start_tree_insert = Instant::now();
    insert.clone().into_iter().enumerate().for_each(|(i, bbox)| { tree.insert(bbox, i); });
    println!("Tree insert: {}", start_tree_insert.elapsed().as_secs_f64());

    let start_seq_insert = Instant::now();
    insert.into_iter().enumerate().for_each(|(i, bbox)| { seq.push((bbox, i)); });
    println!("Seq insert: {}", start_seq_insert.elapsed().as_secs_f64());

    let search = generate_bboxes(69, n_searches, search_range);

    let start_tree_search = Instant::now();
    search.iter().for_each(|search_bbox| { let tree_res = tree.search(&search_bbox).collect_vec(); });
    println!("Tree search: {}", start_tree_search.elapsed().as_secs_f64());

    let start_seq_search = Instant::now();
    search.iter().for_each(|search_bbox| { let seq_res = seq.iter().filter(|(bbox, i)| search_bbox.contains(bbox)).collect_vec(); });
    println!("Seq search: {}", start_seq_search.elapsed().as_secs_f64());

    let search_bbox = search.first().unwrap();

    let tree_res = tree.search(&search_bbox).map(|(bbox, i)| i).sorted().collect_vec();
    let seq_res = seq.iter().filter(|(bbox, i)| search_bbox.contains(bbox)).map(|(bbox, i)| i).sorted().collect_vec();
    assert_eq!(tree_res, seq_res)
}


fn generate_points(seed: u64, n_intervals: usize, range: Range<u64>) -> Vec<BoundingBox<2>> {
    let mut rng = StdRng::seed_from_u64(seed);

    (0..n_intervals)
        .map(|_| {
            let x = rng.gen_range(range.clone());
            let y = rng.gen_range(range.clone());
            let bbox = BoundingBox::from_intervals(
                [Interval::new(x, x).unwrap(), Interval::new(y, y).unwrap()]).unwrap();
            return bbox;
        })
        .collect_vec()
}

#[test]
fn bench_points_tree_vs_seq() {
    let n_intervals = 1000;
    let n_searches = 100000;
    let insert_range = 0..100000;
    let search_range = 0..100;

    let mut tree = RTree::new();
    let mut seq = vec![];

    let insert = generate_points(42, n_intervals, insert_range.clone());

    let start_tree_insert = Instant::now();
    insert.clone().into_iter().enumerate().for_each(|(i, bbox)| { tree.insert(bbox, i); });
    println!("Tree insert: {}", start_tree_insert.elapsed().as_secs_f64());

    let start_seq_insert = Instant::now();
    insert.into_iter().enumerate().for_each(|(i, bbox)| { seq.push((bbox, i)); });
    println!("Seq insert: {}", start_seq_insert.elapsed().as_secs_f64());

    let search = generate_points(69, n_searches, search_range);

    let start_tree_search = Instant::now();
    search.iter().for_each(|search_bbox| { let tree_res = tree.search(&search_bbox).collect_vec(); });
    println!("Tree search: {}", start_tree_search.elapsed().as_secs_f64());

    let start_seq_search = Instant::now();
    search.iter().for_each(|search_bbox| { let seq_res = seq.iter().filter(|(bbox, i)| search_bbox.contains(bbox)).collect_vec(); });
    println!("Seq search: {}", start_seq_search.elapsed().as_secs_f64());

    let search_bbox = search.first().unwrap();

    let tree_res = tree.search(&search_bbox).map(|(bbox, i)| i).sorted().collect_vec();
    let seq_res = seq.iter().filter(|(bbox, i)| search_bbox.contains(bbox)).map(|(bbox, i)| i).sorted().collect_vec();
    assert_eq!(tree_res, seq_res)
}
