# Day 8: Playground

## Problem Summary

You find yourself in a vast underground space containing a giant playground with suspended electrical junction boxes in 3D space. The Elves need to connect junction boxes with strings of lights to form circuits.

### Part 1

Connect the **1000 pairs of junction boxes** that are closest together (by straight-line distance). Calculate the product of the sizes of the **three largest circuits** formed.

### Part 2

Continue connecting junction boxes until they **all form a single circuit**. Find the product of the **X coordinates** of the last two junction boxes that needed to be connected.

## Solution Explanation

### Part 1 Solution

The solution in `part1.rs` uses the **Union-Find (Disjoint Set Union)** data structure:

- **Parse Input**: Read all junction box positions as 3D coordinates (X, Y, Z).
- **Calculate Distances**: Compute squared Euclidean distances for all pairs to avoid floating-point.
- **Sort Pairs**: Sort all pairs by distance in ascending order.
- **Connect 1000 Pairs**: Use Union-Find to connect the 1000 closest pairs.
  - **Path Compression**: Optimizes `find` operations.
  - **Union by Rank**: Keeps trees balanced.
- **Count Circuits**: After connecting, count the size of each circuit.
- **Result**: Multiply the sizes of the 3 largest circuits.

### Part 2 Solution

The solution in `part2.rs` extends Part 1:

- **Same Setup**: Parse points, calculate distances, sort by distance.
- **Connect Until One Circuit**: Instead of stopping at 1000 pairs, continue until:
  - Each successful `union` reduces the circuit count by 1.
  - Stop when `num_circuits == 1`.
- **Track Last Pair**: Record the indices of the last two junction boxes merged.
- **Result**: Multiply the X coordinates of those two junction boxes.

### Key Data Structure: Union-Find

The Union-Find structure efficiently tracks connected components:

```rust
struct UnionFind {
    parent: Vec<usize>,  // parent[i] = parent of node i
    rank: Vec<usize>,    // rank[i] = tree depth (for balancing)
}
```

- **`find(x)`**: Returns the root of x's component (with path compression).
- **`union(x, y)`**: Merges x and y's components (returns `true` if they were different).

## Running the Solutions

To run the solutions and see the results:

```bash
cargo run --release
```

To run the tests:

```bash
cargo test
```

## Results & Benchmarks

| Part   | Result     | Time  |
| :----- | :--------- | :---- |
| Part 1 | 47040      | ~38µs |
| Part 2 | 4884971896 | ~35µs |
