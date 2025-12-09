# Day 1: Secret Entrance

## Problem Summary

You need to crack a safe to enter the North Pole base. The safe has a circular dial numbered 0 through 99. You start at position 50 and follow a series of rotation instructions.

Each instruction tells you to rotate the dial:

- `L` means rotate left (toward lower numbers)
- `R` means rotate right (toward higher numbers)
- The number after the direction tells you how many clicks to rotate

Because the dial is circular, going left from 0 wraps around to 99, and going right from 99 wraps to 0.

### Part 1

Count how many times the dial lands on 0 after completing each rotation.

### Part 2

Count every time the dial passes through 0, including during the rotation itself, not just at the end.

## Solution Explanation

### Part 1 Solution

The solution in `part1.rs` works as follows:

1. Start at position 50
2. For each instruction:
   - Parse the direction (L or R) and the rotation amount
   - Calculate the new position using modular arithmetic
   - If the final position is 0, increment the counter
3. Print the total count

**Key technique**: We use `rem_euclid(100)` to handle the circular nature of the dial. This ensures that:

- Subtracting from 0 wraps to 99
- Adding to 99 wraps to 0

For example:

- Position 5, rotate L10: `(5 - 10) % 100 = -5 % 100 = 95`
- Position 95, rotate R10: `(95 + 10) % 100 = 105 % 100 = 5`

### Part 2 Solution

The solution in `part2.rs` builds on Part 1 but simulates each individual click:

1. Start at position 50
2. For each instruction:
   - Parse the direction and amount
   - Instead of jumping directly to the final position, move one click at a time
   - Check if the dial is at 0 after every single click
   - Increment the counter each time we pass through 0
3. Print the total count

**Key difference**: Rather than calculating the final position directly, we use a loop to simulate each step of the rotation. This allows us to detect when the dial passes through 0 during a rotation, not just at the end.

For example, if we're at position 5 and rotate R50:

- Part 1 would only check the final position (55)
- Part 2 checks positions 6, 7, 8, ..., 0, 1, 2, ..., 55 and counts the 0

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

| Part   | Result | Time   |
| :----- | :----- | :----- |
| Part 1 | 1052   | ~168µs |
| Part 2 | 6295   | ~3ms   |
