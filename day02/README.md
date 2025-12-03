# Day 2: Gift Shop

## Problem Summary

You need to help identify invalid product IDs in the gift shop's database. The young Elf was creating silly patterns with product IDs, and you need to find them within given ranges.

The input is a series of comma-separated ranges (e.g., `11-22,95-115,998-1012`), where each range is formatted as `start-end`.

### Part 1
Find all invalid IDs where the ID is made of a sequence of digits repeated **exactly twice**. 

Examples:
- `55` (5 repeated twice)
- `6464` (64 repeated twice)
- `123123` (123 repeated twice)

Count all such IDs within the given ranges and sum them up.

### Part 2
Find all invalid IDs where the ID is made of a sequence of digits repeated **at least twice**.

Examples:
- `12341234` (1234 repeated 2 times)
- `123123123` (123 repeated 3 times)
- `1212121212` (12 repeated 5 times)
- `1111111` (1 repeated 7 times)

Count all such IDs within the given ranges and sum them up.

## Solution Explanation

### Part 1 Solution

The solution in `part1.rs` works as follows:

1. Parse the input to extract all ranges (split by commas)
2. For each range, extract the start and end values
3. Iterate through each number in the range
4. Check if the number is invalid using the `is_invalid_id` function:
   - Convert the number to a string
   - Check if the length is even (must be to split in half)
   - Split the string in half and compare both halves
   - If they match, the number is invalid
5. Sum all invalid IDs and print the result

**Key technique**: String manipulation to check if a number can be split into two identical halves.

For example:
- `1212`: Split into "12" and "12" → Match! Invalid.
- `123123`: Split into "123" and "123" → Match! Invalid.
- `1234`: Split into "12" and "34" → No match. Valid.

### Part 2 Solution

The solution in `part2.rs` builds on Part 1 but checks for any repeating pattern:

1. Parse the input the same way as Part 1
2. For each number, check if it's invalid using an enhanced `is_invalid_id` function:
   - Try all possible substring lengths from 1 to ⌊length/2⌋
   - For each valid substring length (where length is divisible by it):
     - Extract the first substring of that length
     - Check if repeating that substring creates the entire number
   - If any pattern works, the number is invalid
3. Sum all invalid IDs and print the result

**Key difference**: Instead of just checking if a number splits in half, we check all possible repeating patterns.

For example, `123123123`:
- Try substring length 1: "1" repeated → "111111111" ≠ "123123123"
- Try substring length 3: "123" repeated → "123123123" ✓ Match! Invalid.

## Running the Solutions

```bash
cargo run
```

This will run both Part 1 and Part 2 solutions and display the sums of invalid IDs.

## Results

- Part 1: **44854383294**
- Part 2: **55647141923**
