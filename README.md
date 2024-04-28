# R-Tree

R-Tree is a tree data structure used for spatial access methods, i.e., 
for indexing multidimensional information such as geographical coordinates, rectangles or polygons. 
The R-Tree is an improvement of the B-Tree data structure. 
The R-Tree is a height-balanced tree, designed to handle multidimensional data.

This implementation contains the following features:
- Insertion
- Search (Range Query)
- K Nearest Neighbors
- Graphical Visualization
- Benchmarks

To run the visualization, run the following command:
```bash
cargo run --release
```

To run the benchmarks, run the following command:
```bash
cargo test -- --nocapture
```
