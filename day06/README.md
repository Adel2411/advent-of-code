# Day 6: Trash Compactor

## Problem Summary

You find yourself in a garbage compactor with a family of cephalopods who need help with their math homework. The math problems are presented in a strange 2D grid format. Numbers and operators are arranged spatially rather than linearly.

### Part 1
The worksheet contains multiple problems separated by empty columns.
- **Orientation**: Standard horizontal numbers (each row in a block is a number).
- **Structure**: 
  - Several lines of numbers.
  - A final line containing the operator (`+` or `*`).
- **Goal**: Calculate the result of each problem and find the grand total used to check the work.

Example problem:
```
123 
 45 
  6
*
```
This is interpreted as `123 * 45 * 6`.

### Part 2
The cephalopods explain that their math is actually written **right-to-left in columns**.
- **Orientation**: Vertical numbers (each column in a block is a number, read top-to-bottom).
- **Structure**:
  - Each column represents a number (most significant digit at top).
  - Problems are read right-to-left (rightmost column is the first number).
- **Goal**: Re-calculate the grand total with this new interpretation.

Example block re-interpreted:
- A column `1`, ` `, ` ` is number `1`.
- A column `2`, `4`, `9` is number `249`.
- The operator remains at the bottom of the block.

## Solution Explanation

### Part 1 Solution

The solution processes the input as a 2D grid:
1. **Identify Blocks**: Scan columns to find "separators" (columns consisting entirely of spaces). These split the grid into distinct problem blocks.
2. **Parse Blocks**: For each block:
   - Read rows as horizontal numbers.
   - Ignore spaces and align numbers.
   - Identify the operator from the last line.
3. **Calculate**: Perform the operation (`+` or `*`) on the numbers.
4. **Sum**: Add all results to get the grand total.

### Part 2 Solution

The solution re-uses the block identification logic but parses differently:
1. **Identify Blocks**: Same as Part 1 (empty columns separate problems).
2. **Parse Blocks**: For each block:
   - Iterate columns from **right to left**.
   - For each column, read characters from **top to bottom** to form a number.
   - Identify the operator from the bottom row.
3. **Calculate**: Perform the operation on the extracted numbers.
4. **Sum**: Add all results to get the grand total.

**Key Difference**: 
- Part 1 reads numbers **horizontally**.
- Part 2 reads numbers **vertically** and processes them **right-to-left**.

## Running the Solutions

```bash
cargo run
```

This will run both Part 1 and Part 2 solutions and display the grand totals.

## Results

- Part 1 Grand Total: **6503327062445**
- Part 2 Grand Total: **9640641878593**
