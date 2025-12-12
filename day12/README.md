# Day 12: Shape Fitting

## Problem Summary

You discover an industrial area with pattern matching puzzles. The Elves have various shapes (defined by grids of `#` and `.` where `#` represents filled cells) and regions with specific dimensions. For each region, you're given counts of how many of each shape must be placed.

### Part 1

Given a list of shapes (numbered 0-5) and regions with dimension specifications (WxH) and counts for each shape, determine how many regions can potentially fit all the specified shapes based on area alone.

A region can fit all shapes if the total area of all shapes (multiplied by their counts) is less than or equal to the region's area.

**Example:**
```
0:
###
##.
##.

...

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
```

For `4x4: 0 0 0 0 2 0`:
- Region area = 4 × 4 = 16
- Shape 4 has 7 cells (count = 2): 7 × 2 = 14
- Total shape area = 14
- 14 ≤ 16, so it fits!

## Solution Explanation

### Part 1 Solution

The solution in `part1.rs` uses a simple area comparison:

1. **Parse Input**: 
   - Parse shape definitions (grids of `#` and `.`)
   - Parse region specifications (dimensions and shape counts)

2. **Calculate Shape Sizes**:
   - For each shape, count the number of `#` cells

3. **Check Each Region**:
   - Calculate region area: `width × height`
   - Calculate total shape area: `sum(shape_size × count)` for all shapes
   - Region fits if `region_area >= total_shape_area`

4. **Count Fitting Regions**:
   - Return the count of regions that can accommodate all shapes

**Complexity**: O(S × R × C) where S = number of shapes, R = number of regions, C = average cells per shape

## Running the Solutions

To run the solution and see the result:

```bash
cargo run --release
```

## Results & Benchmarks

| Part   | Result | Time    |
| :----- | :----- | :------ |
| Part 1 | 499    | ~437µs  |
