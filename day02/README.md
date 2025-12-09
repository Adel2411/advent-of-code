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

The solution in `part1.rs` uses a **mathematical closed-form approach** instead of iterating through every number:

1. **Pattern Recognition**: Invalid IDs follow a specific structure. For example, 2-digit IDs like `11, 22, ..., 99` are separated by a step of 11. Similarly, 4-digit IDs like `1010, 1111, 1212, ..., 9999` follow a step pattern.

2. **Lookup Table**: We define `FIRST` which contains `[total_digits, unit_size]` pairs:
   - `[2, 1]`: 2-digit numbers with 1-digit repeating unit (11, 22, ..., 99)
   - `[4, 2]`: 4-digit numbers with 2-digit repeating unit (1010, 1111, ..., 9999)
   - `[6, 3]`: 6-digit numbers with 3-digit repeating unit
   - And so on...

3. **Step Calculation**: For each pattern, we calculate:
   - `step = (10^digits - 1) / (10^size - 1)` — the difference between consecutive invalid IDs
   - `start` — the first valid invalid ID in this category
   - `end` — the last valid invalid ID in this category

4. **Arithmetic Series**: For each input range, we find the overlap with our pattern range and use the **arithmetic series formula** to compute the sum directly:
   ```
   sum = lower * (n + 1) + step * n * (n + 1) / 2
   ```
   where `n` is the count of invalid IDs in the range.

**Key insight**: Instead of checking millions of numbers, we compute the exact sum in O(1) per pattern category!

### Part 2 Solution

The solution in `part2.rs` extends Part 1 using the **inclusion-exclusion principle**:

1. **Additional Patterns**: We need to capture patterns repeated more than twice:
   - `SECOND`: Patterns for 3, 5, or 7 repetitions (e.g., `111`, `11111`, `1212121212`)
   - `THIRD`: Correction terms for overlapping counts

2. **Inclusion-Exclusion Formula**:

   ```
   Total = sum(FIRST) + sum(SECOND) - sum(THIRD)
   ```

3. **Why Subtraction?**: Some numbers are counted multiple times. For example:
   - `111111` (6 digits) is counted as "111" repeated twice (in FIRST)
   - `111111` is also counted as "1" repeated 6 times (partially in SECOND)
   - THIRD corrects for this overcounting

The lookup tables are carefully constructed to ensure each invalid ID is counted exactly once.

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

| Part   | Result      | Time  |
| :----- | :---------- | :---- |
| Part 1 | 44854383294 | ~29µs |
| Part 2 | 55647141923 | ~20µs |
