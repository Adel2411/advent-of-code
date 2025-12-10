# Day 10: Factory

## Problem Summary

You're in a factory with machines that need initialization. Each machine has indicator lights and buttons that control them. The manual provides diagrams showing the target states and button configurations.

### Part 1

Each machine has indicator lights (initially all off) that need to match a target pattern. Buttons toggle specific lights. Each button can be pressed any number of times, and pressing a button toggles all its associated lights.

**Goal**: Find the minimum number of button presses needed to configure all machines' indicator lights.

Example: `[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1)`

- Target: lights 0=off, 1=on, 2=on, 3=off
- Buttons toggle specific light indices
- Minimum: 2 presses (buttons `(0,2)` and `(0,1)`)

### Part 2

Now the machines need joltage configuration. Each machine has counters (initially 0) that need to reach target values. Buttons now increment specific counters instead of toggling lights.

**Goal**: Find the minimum number of button presses to configure all machines' joltage counters.

Example: `{3,5,4,7}` with buttons `(3) (1,3) (2) (2,3) (0,2) (0,1)`

- Target: counters [3, 5, 4, 7]
- Each button press increments its associated counters by 1
- Minimum: 10 presses

## Solution Explanation

### Part 1 Solution

The solution in `part1.rs` uses **Gaussian elimination over GF(2)** to reduce the toggle system to a canonical form and then searches the nullspace for the minimum-weight solution.

1. Parse each machine's light diagram into a bitmask `target` and parse button definitions into column masks (a button's mask has bits set for lights it toggles).
2. Build a binary matrix (rows = lights, columns = buttons) and a right-hand side vector from the target bits.
3. Perform Gaussian elimination over GF(2) to find a particular solution `x0` and a basis for the nullspace (free variables).
4. The general solution is `x = x0 XOR v` where `v` is any linear combination of nullspace basis vectors. To find the minimum number of button presses (minimum Hamming weight of `x`):
   - If the nullspace dimension `k` is small (≤ 26), enumerate all 2^k combinations of basis vectors and pick the solution with the fewest set bits.
   - If `k` is large, apply a small local heuristic: iterate a few passes trying to XOR the current best solution with basis vectors that decrease the Hamming weight.
5. Sum the minimal press counts across all machines.

**Key insight**: Modeling toggles as linear equations over GF(2) turns the problem into finding a minimum-weight solution within an affine subspace. Exhaustive enumeration of the nullspace is feasible when its dimension is small; otherwise, a heuristic provides a good approximation.

**Complexity**: Gaussian elimination is O(L _ B^2) for L lights and B buttons; enumerating the nullspace is O(2^k _ B) where k is the nullspace dimension. The hybrid exact+heuristic approach keeps the method practical for real inputs.

### Part 2 Solution

The solution in `part2.rs` uses **Integer Linear Programming (ILP)** with the HiGHS solver via `good_lp`:

1. Model the problem as an ILP:
   - **Variables**: Number of times each button is pressed (non-negative **integers**)
   - **Objective**: Minimize the sum of all button presses
   - **Constraints**: For each counter, the sum of contributions from all buttons must equal the target joltage
2. Build the constraint expressions where button i contributes to counter j if i affects j
3. Use `good_lp` with the HiGHS backend to solve the ILP
4. Sum the optimal press counts across all machines

**Key insight**: This is an Integer Linear Programming problem. Unlike continuous LP, ILP guarantees integer solutions which is critical here. The HiGHS solver uses branch-and-bound algorithms to efficiently find the globally optimal integer solution.

**Why Integer Programming matters**:

- Continuous LP (like `minilp`) can give fractional solutions like 2.5 button presses
- ILP enforces integer constraints natively in the solver
- Simply rounding continuous LP solutions can give wrong or suboptimal answers
- HiGHS is a state-of-the-art ILP solver that handles these problems efficiently

**Advantages of ILP approach**:

- Guarantees globally optimal solution
- Handles large systems efficiently (despite being NP-hard theoretically)
- No need for heuristics or manual tuning
- Leverages sophisticated optimization algorithms (branch-and-bound, cutting planes)

**Complexity**: Exponential in worst case (NP-hard), but modern ILP solvers like HiGHS are highly optimized and solve most practical instances quickly.

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
| Part 1 | 530    | ~900µs |
| Part 2 | 20172  | ~227ms |
