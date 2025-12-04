# Day 4: Printing Department

## Problem Summary

You are in the printing department and need to help the Elves optimize their forklift operations to clear a path through a wall of paper rolls. The paper rolls (`@`) are arranged in a grid.

### Part 1
The forklifts can only access a roll of paper if there are **fewer than four** rolls of paper in the eight adjacent positions (horizontal, vertical, and diagonal).

Count how many rolls of paper are currently accessible.

### Part 2
Once a roll of paper is accessed, it can be removed. Removing a roll might make other rolls accessible. The Elves keep removing accessible rolls until no more rolls can be accessed.

Calculate the **total number of rolls** that can be removed by repeating this process.

## Solution Explanation

### Part 1 Solution

The solution in `part1.rs` iterates through the grid to check the condition for each paper roll:

1. Parse the input into a 2D grid.
2. Iterate through every cell `(r, c)`.
3. If the cell contains a paper roll (`@`):
   - Count its neighbors (checking all 8 directions).
   - If the neighbor count is less than 4, increment the `accessible_count`.
4. Print the total count.

**Key concept**: Simple grid iteration and neighbor counting.

### Part 2 Solution

The solution in `part2.rs` simulates the removal process:

1. Parse the input into a mutable 2D grid.
2. Enter a loop:
   - Scan the entire grid to find all rolls (`@`) that currently have fewer than 4 neighbors.
   - Store the coordinates of these accessible rolls in a list `to_remove`.
   - If `to_remove` is empty, break the loop (process complete).
   - Add the number of rolls in `to_remove` to `total_removed`.
   - Update the grid by changing all rolls in `to_remove` to empty space (`.`).
3. Print the `total_removed`.

**Key concept**: Cellular automaton / Simulation. We repeatedly apply the rule and update the state until it stabilizes.

## Running the Solutions

```bash
cargo run
```

This will run both Part 1 and Part 2 solutions and display the results.

## Results

- Part 1: **1491**
- Part 2: **8722**
