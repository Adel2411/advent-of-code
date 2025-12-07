# Day 7: Laboratories

## Problem Summary

You need to repair a broken teleporter by analyzing its tachyon manifold. The manifold is a grid containing empty space (`.`) and splitters (`^`). A tachyon beam enters at `S` and travels downwards.

### Part 1
A tachyon beam splits into two (left and right) whenever it hits a splitter. You need to count the **total number of times** a beam is splits.

### Part 2
Using the "many-worlds interpretation", each split actually creates two distinct timelines. You need to calculate the **total number of timelines** formed by a single particle traversing all possible paths through the manifold.

## Solution Explanation

### Part 1 Solution

The solution in `part1.rs` simulates the propagation of beams row by row.
- **State**: A set of active column indices for the current row.
- **Simulation**:
  - Iterate through each row.
  - For each active beam:
    - If it's on `.`, it continues to the next row (same column).
    - If it's on `^`, it **splits**:
      - Increment the `split_count`.
      - New beams are created at `col-1` and `col+1` for the next row.
  - Use a `HashSet` to naturally handle merging beams (multiple beams hitting the same spot merge into one).

### Part 2 Solution

The solution in `part2.rs` counts distinct timelines using Dynamic Programming (or just counting path weights).
- **State**: A `HashMap` mapping `column -> count`, representing the number of timelines reaching that column in the current row.
- **Algorithm**:
  - Initialize `S` with count 1.
  - Iterate row by row.
  - For each position `(col, count)`:
    - If it's `.`: Pass `count` to `(row+1, col)`.
    - If it's `^`:
      - Pass `count` to `(row+1, col-1)` (left split).
      - Pass `count` to `(row+1, col+1)` (right split).
      - If a split goes out of bounds, add `count` to the `finished_timelines`.
  - Accumulate counts for any beams falling off the bottom of the grid.
- **Key Concept**: Instead of tracking 2^N individual particles, we simply sum the number of ways to reach each point. If 5 timelines reach a point and split, 5 go left and 5 go right.

## Running the Solutions

```bash
cargo run
```

This will run both Part 1 and Part 2 solutions and display the results.

## Results

- Part 1 Split Count: **1592**
- Part 2 Total Timelines: **17921968177009**
