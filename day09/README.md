# Day 9: Movie Theater

## Problem Summary

You're in a movie theater with a tile floor. The Elves want to find the largest rectangle using red tiles as opposite corners.

### Part 1

Find the largest rectangle that can be formed by using **any two red tiles** as opposite corners. The rectangle can include any tiles (red, green, or other).

### Part 2

The Elves can only use **red or green tiles**. The red tiles in the input are ordered and connected by green tiles forming a closed polygon. Green tiles include:

- Lines connecting consecutive red tiles
- All tiles inside the polygon formed by red tiles

Find the largest rectangle with red corners that **only contains red or green tiles**.

## Solution Explanation

### Part 1 Solution

The solution in `part1.rs` is straightforward:

- **Parse Input**: Read all red tile coordinates (x, y)
- **Check All Pairs**: For every pair of red tiles:
  - Calculate width = `|x2 - x1| + 1` (inclusive)
  - Calculate height = `|y2 - y1| + 1` (inclusive)
  - Calculate area = width × height
- **Track Maximum**: Keep the largest area found

**Time Complexity**: O(n²) where n is the number of red tiles.

### Part 2 Solution

The solution in `part2.rs` uses **coordinate compression** to dramatically improve performance:

- **Coordinate Compression**: Instead of checking every coordinate (which could be 100,000 x 50,000 = 5 billion points), we compress the grid to only include unique X and Y coordinates from the red tiles.
- **Compressed Grid Creation**:
  1. Collect all unique X coordinates and Y coordinates
  2. Create mappings: `actual_coord → compressed_index`
  3. Build a compressed grid (typically ~500 x 500 instead of millions)

- **Mark Polygon**:
  1. Fill compressed grid cells on polygon edges with `'#'`
  2. Initialize unknown interior cells with `'X'`

- **Flood Fill (BFS)**:
  1. Start from outside boundary (0, 0)
  2. Use BFS to mark all exterior cells as `'.'`
  3. Remaining `'X'` cells become `'#'` (they're inside the polygon)

- **Validate Rectangles**:
  - For each pair of red tiles, check the compressed rectangle
  - If any cell is `'.'` (outside), the rectangle is invalid
  - Calculate actual area using original coordinates

**Key Optimization**: Checking a 500x500 compressed grid is ~20,000x faster than checking billions of actual coordinates!

**Time Complexity**: O(C² + n² × C²) where:

- C = number of unique coordinates (~500)
- n = number of red tiles (496)

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

| Part   | Result     | Time   |
| :----- | :--------- | :----- |
| Part 1 | 4782896435 | ~300µs |
| Part 2 | 1540060480 | ~58ms  |
