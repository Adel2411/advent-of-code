# Day 5: Cafeteria

## Problem Summary

You need to help the Elves identify fresh ingredients based on a database of valid ID ranges. The database provides a list of inclusive ranges (e.g., `3-5`) that are considered fresh.

### Part 1
Given a list of available ingredient IDs, determine how many of them are fresh. An ID is fresh if it falls into *any* of the valid ranges.

Example:
- Ranges: `3-5`, `10-14`
- IDs: `1`, `5`, `8`, `11`
- Fresh: `5` (in 3-5), `11` (in 10-14) -> Count: 2

### Part 2
Calculate the **total number** of unique ingredient IDs that are considered fresh according to the ranges. Since ranges can overlap, we need to merge them to avoid double-counting.

Example:
- Ranges: `3-5`, `4-6`
- Merged: `3-6` -> IDs: 3, 4, 5, 6 -> Count: 4

## Solution Explanation

### Part 1 Solution

The solution in `part1.rs` works as follows:

1. Parse the input into two sections: ranges and IDs.
2. Store ranges as a list of `(start, end)` tuples.
3. Iterate through each available ID.
4. Check if the ID exists within any of the ranges.
5. Count and print the number of valid IDs.

**Key technique**: Simple iteration and range inclusion check.

### Part 2 Solution

The solution in `part2.rs` handles the potentially large and overlapping ranges:

1. Parse the input to extract only the ranges.
2. **Sort** the ranges by their start position.
3. **Merge** overlapping or adjacent ranges:
   - Iterate through the sorted ranges.
   - If the current range overlaps with or touches the previous merged range, extend the previous range.
   - Otherwise, start a new merged range.
4. Calculate the size of each merged range (`end - start + 1`) and sum them up.

**Key technique**: Range merging algorithm (O(N log N) due to sorting). This is essential because iterating through all individual IDs would be too slow given the large range sizes.

## Running the Solutions

```bash
cargo run
```

This will run both Part 1 and Part 2 solutions and display the results.

## Results

- Part 1 Fresh Ingredients: **679**
- Part 2 Total Fresh Ingredients: **358155203664116**
