mod test {
    use std::ops::Range;
    use std::time::Instant;

    use itertools::Itertools;
    use rand::{Rng, SeedableRng};
    use rand::rngs::StdRng;

    use r_tree::{BoundingBox, Interval, RTree, SplitPolicy};

    fn generate_points<const N: usize>(seed: u64, n_intervals: usize, range: Range<u64>) -> Vec<BoundingBox<N>> {
        let mut rng = StdRng::seed_from_u64(seed);

        (0..n_intervals)
            .map(|_| {
                let intervals = (0..N)
                    .map(|_| rng.gen_range(range.clone()))
                    .map(|x| Interval::new(x, x).unwrap())
                    .collect_vec()
                    .try_into()
                    .unwrap();

                let bbox = BoundingBox::from_intervals(intervals).unwrap();
                return bbox;
            })
            .collect_vec()
    }

    #[test]
    fn bench_tree_vs_seq() {
        println!("Bench points tree vs seq");
        let n_intervals = 10000;
        let n_searches = 10000;
        let insert_range = 0..100000;
        let search_range = 0..1000;

        let mut tree = RTree::new();
        let mut seq = vec![];

        let insert = generate_points::<2>(42, n_intervals, insert_range.clone());

        let start_tree_insert = Instant::now();
        insert.clone().into_iter().enumerate().for_each(|(i, bbox)| { tree.insert(bbox, i); });
        println!("Tree insert: {}", start_tree_insert.elapsed().as_secs_f64());

        let start_seq_insert = Instant::now();
        insert.into_iter().enumerate().for_each(|(i, bbox)| { seq.push((bbox, i)); });
        println!("Seq insert: {}", start_seq_insert.elapsed().as_secs_f64());

        let search = generate_points(69, n_searches, search_range);

        let start_tree_search = Instant::now();
        search.iter().for_each(|search_bbox| { let _ = tree.search(&search_bbox).collect_vec(); });
        println!("Tree search: {}", start_tree_search.elapsed().as_secs_f64());

        let start_seq_search = Instant::now();
        search.iter().for_each(|search_bbox| { let _ = seq.iter().filter(|(bbox, _)| search_bbox.contains(bbox)).collect_vec(); });
        println!("Seq search: {}", start_seq_search.elapsed().as_secs_f64());

        let search_bbox = search.first().unwrap();

        let tree_res = tree.search(&search_bbox).map(|(_, i)| i).sorted().collect_vec();
        let seq_res = seq.iter().filter(|(bbox, _)| search_bbox.contains(bbox)).map(|(_, i)| i).sorted().collect_vec();
        assert_eq!(tree_res, seq_res);
        println!()
    }

    #[test]
    fn bench_points_tree_dim1() {
        println!("Bench points tree dim 1");
        let n_intervals = 10000;
        let n_searches = 10000;
        let insert_range = 0..100000;
        let search_range = 0..1000;

        let mut tree = RTree::new();

        let insert = generate_points::<1>(42, n_intervals, insert_range.clone());

        let start_tree_insert = Instant::now();
        insert.clone().into_iter().enumerate().for_each(|(i, bbox)| { tree.insert(bbox, i); });
        println!("Tree insert: {}", start_tree_insert.elapsed().as_secs_f64());

        let search = generate_points(69, n_searches, search_range);

        let start_tree_search = Instant::now();
        search.iter().for_each(|search_bbox| { let _ = tree.search(&search_bbox).collect_vec(); });
        println!("Tree search: {}", start_tree_search.elapsed().as_secs_f64());

        assert!(true);
        println!()
    }
    
    #[test]
    fn bench_points_tree_dim3() {
        println!("Bench points tree dim 3");
        let n_intervals = 10000;
        let n_searches = 10000;
        let insert_range = 0..100000;
        let search_range = 0..1000;

        let mut tree = RTree::new();

        let insert = generate_points::<3>(42, n_intervals, insert_range.clone());

        let start_tree_insert = Instant::now();
        insert.clone().into_iter().enumerate().for_each(|(i, bbox)| { tree.insert(bbox, i); });
        println!("Tree insert: {}", start_tree_insert.elapsed().as_secs_f64());

        let search = generate_points(69, n_searches, search_range);

        let start_tree_search = Instant::now();
        search.iter().for_each(|search_bbox| { let _ = tree.search(&search_bbox).collect_vec(); });
        println!("Tree search: {}", start_tree_search.elapsed().as_secs_f64());

        assert!(true);
        println!()
    }

    #[test]
    fn bench_points_tree_low_capacity_bruteforce() {
        println!("Bench points tree with capacity 4 and brute-force");
        let n_intervals = 10000;
        let n_searches = 10000;
        let insert_range = 0..100000;
        let search_range = 0..1000;

        let mut tree = RTree::with_capacity_and_split_policy(4, SplitPolicy::BruteForce);

        let insert = generate_points::<2>(42, n_intervals, insert_range.clone());

        let start_tree_insert = Instant::now();
        insert.clone().into_iter().enumerate().for_each(|(i, bbox)| { tree.insert(bbox, i); });
        println!("Tree insert: {}", start_tree_insert.elapsed().as_secs_f64());

        let search = generate_points(69, n_searches, search_range);

        let start_tree_search = Instant::now();
        search.iter().for_each(|search_bbox| { let _ = tree.search(&search_bbox).collect_vec(); });
        println!("Tree search: {}", start_tree_search.elapsed().as_secs_f64());

        assert!(true);
        println!()
    }

    #[test]
    fn bench_points_tree_low_capacity_greedy() {
        println!("Bench points tree with capacity 4 and greedy");
        let n_intervals = 10000;
        let n_searches = 10000;
        let insert_range = 0..100000;
        let search_range = 0..1000;

        let mut tree = RTree::with_capacity_and_split_policy(4, SplitPolicy::Greedy);

        let insert = generate_points::<2>(42, n_intervals, insert_range.clone());

        let start_tree_insert = Instant::now();
        insert.clone().into_iter().enumerate().for_each(|(i, bbox)| { tree.insert(bbox, i); });
        println!("Tree insert: {}", start_tree_insert.elapsed().as_secs_f64());

        let search = generate_points(69, n_searches, search_range);

        let start_tree_search = Instant::now();
        search.iter().for_each(|search_bbox| { let _ = tree.search(&search_bbox).collect_vec(); });
        println!("Tree search: {}", start_tree_search.elapsed().as_secs_f64());

        assert!(true);
        println!()
    }

    #[test]
    fn bench_points_tree_low_capacity_half() {
        println!("Bench points tree with capacity 4 and half");
        let n_intervals = 10000;
        let n_searches = 10000;
        let insert_range = 0..100000;
        let search_range = 0..1000;

        let mut tree = RTree::with_capacity_and_split_policy(4, SplitPolicy::Half);

        let insert = generate_points::<2>(42, n_intervals, insert_range.clone());

        let start_tree_insert = Instant::now();
        insert.clone().into_iter().enumerate().for_each(|(i, bbox)| { tree.insert(bbox, i); });
        println!("Tree insert: {}", start_tree_insert.elapsed().as_secs_f64());

        let search = generate_points(69, n_searches, search_range);

        let start_tree_search = Instant::now();
        search.iter().for_each(|search_bbox| { let _ = tree.search(&search_bbox).collect_vec(); });
        println!("Tree search: {}", start_tree_search.elapsed().as_secs_f64());

        assert!(true);
        println!()
    }

    #[test]
    fn bench_points_tree_low_capacity_random() {
        println!("Bench points tree with capacity 4 and random");
        let n_intervals = 10000;
        let n_searches = 10000;
        let insert_range = 0..100000;
        let search_range = 0..1000;

        let mut tree = RTree::with_capacity_and_split_policy(4, SplitPolicy::Random);

        let insert = generate_points::<2>(42, n_intervals, insert_range.clone());

        let start_tree_insert = Instant::now();
        insert.clone().into_iter().enumerate().for_each(|(i, bbox)| { tree.insert(bbox, i); });
        println!("Tree insert: {}", start_tree_insert.elapsed().as_secs_f64());

        let search = generate_points(69, n_searches, search_range);

        let start_tree_search = Instant::now();
        search.iter().for_each(|search_bbox| { let _ = tree.search(&search_bbox).collect_vec(); });
        println!("Tree search: {}", start_tree_search.elapsed().as_secs_f64());

        assert!(true);
        println!()
    }
}
