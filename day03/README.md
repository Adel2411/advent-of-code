# Day 3: Lobby

## Problem Summary

You need to power the escalator using batteries. Each bank of batteries is represented by a line of digits (1-9). You must select exactly a certain number of batteries from each bank to maximize the joltage output. The joltage is formed by the digits of the selected batteries in their original order.

### Part 1
Select exactly **2 batteries** from each bank to form the largest possible 2-digit number.

Example with bank `987654321111111`:
- Selecting positions 0 and 1 gives `98` (the maximum)

Sum the maximum joltages from all banks.

### Part 2
Select exactly **12 batteries** from each bank to form the largest possible 12-digit number.

Example with bank `987654321111111`:
- The largest 12-digit subsequence is `987654321111`

Sum the maximum joltages from all banks.

## Solution Explanation

### Part 1 Solution

The solution in `part1.rs` works as follows:

1. Parse each line of digits
2. For each bank, iterate through all pairs of indices `(i, j)` where `i < j`
3. Form the 2-digit number `digits[i] * 10 + digits[j]`
4. Track the maximum for each bank
5. Sum all maximums

**Key technique**: Brute force all possible pairs. Since we only need 2 digits, this is O(n²) per line which is efficient enough.

For example, with `987654321111111`:
- Try pairs: (9,8)=98, (9,7)=97, (9,6)=96, (8,7)=87, ...
- Maximum is `98`

### Part 2 Solution

The solution in `part2.rs` uses a greedy algorithm with a monotonic stack:

1. Parse each line of digits
2. For each bank:
   - Use a stack to build the largest subsequence of length 12
   - Calculate how many digits to remove: `rem = length - 12`
   - Iterate through each digit:
     - While we have remaining removals and the stack's last digit is smaller than current digit:
       - Pop from stack (remove smaller digit to make room for larger one)
       - Decrement `rem`
     - Push current digit to stack
   - Truncate to exactly 12 digits
3. Convert the stack to a number and sum

**Key technique**: Greedy monotonic stack approach. This is the optimal algorithm for finding the largest k-digit subsequence in O(n) time.

For example, with `987654321111111` (15 digits, need 12):
- Remove 3 digits (the three 1s at the end)
- Keep: `987654321111`

The algorithm ensures we keep the largest digits in their leftmost positions.

## Running the Solutions

```bash
cargo run
```

This will run both Part 1 and Part 2 solutions and display the total joltages.

## Results

- Part 1: **17343**
- Part 2: **172664333119298**
