# Day 11: Reactor

## Problem Summary

You discover a large toroidal reactor that powers the factory. The Elves need help debugging communication issues between the reactor and a new server rack. The problem involves analyzing data paths through connected devices.

### Part 1

Each device has outputs that connect to other devices. Data flows only forward through outputs (never backwards). You need to find all possible paths from the device labeled `you` to the device labeled `out`.

**Goal**: Count the total number of distinct paths from `you` to `out`.

Example:

```
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
```

Possible paths:

- `you → bbb → ddd → ggg → out`
- `you → bbb → eee → out`
- `you → ccc → ddd → ggg → out`
- `you → ccc → eee → out`
- `you → ccc → fff → out`

Total: **5 paths**

### Part 2

The Elves have narrowed down the issue: the problematic data path passes through both `dac` (digital-to-analog converter) and `fft` (fast Fourier transform device). Now you need to find all paths from `svr` (server rack) to `out` that visit **both** `dac` and `fft` in any order.

**Goal**: Count paths from `svr` to `out` that pass through both required devices.

Example:

```
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
```

Total paths from `svr` to `out`: 8
Paths visiting both `dac` and `fft`: **2**

Valid paths:

- `svr → aaa → fft → ccc → eee → dac → fff → ggg → out`
- `svr → aaa → fft → ccc → eee → dac → fff → hhh → out`

## Solution Explanation

### Part 1 Solution

The solution in `part1.rs` uses **Depth-First Search (DFS)** with recursive path counting:

1. Parse the input to build a directed graph as an adjacency list (`HashMap<device, Vec<neighbors>>`)
2. Start DFS from the `"you"` device with target `"out"`
3. For each device:
   - **Base case**: If we reach `"out"`, count this as 1 path
   - **Recursive case**: Sum the path counts from all neighbor devices
   - **Dead end**: If a device has no outputs and isn't the target, return 0
4. Return the total count of all paths

**Key insight**: This is a classic path enumeration problem in a directed acyclic graph (DAG). Since there are no cycles (data can't flow backwards), we can use simple recursive DFS without tracking visited nodes.

**Why no cycle detection?** The problem states "data only ever flows from a device through its outputs; it can't flow backwards," which implies the graph is a DAG. Even if the same device appears in multiple paths, each occurrence represents a distinct path through the graph.

**Complexity**: O(V + E + P) where V is vertices, E is edges, and P is the total number of paths. In worst case (complete graph), P could be exponential, but for typical sparse reactor graphs, this is efficient.

### Part 2 Solution

The solution in `part2.rs` uses **Decomposition and Memoization** (Dynamic Programming) to handle the large number of paths efficiently:

1. Parse the input into an adjacency list graph structure.
2. Decompose the problem: A path visiting both `dac` and `fft` must follow one of two orders:
   - `svr` → ... → `dac` → ... → `fft` → ... → `out`
   - `svr` → ... → `fft` → ... → `dac` → ... → `out`
3. Implement a helper function `count_paths(u, v)` that returns the number of paths from `u` to `v` using DFS with **memoization**.
4. Calculate the total paths by multiplying the counts of segments:
   - Path 1 count = `count(svr, dac) * count(dac, fft) * count(fft, out)`
   - Path 2 count = `count(svr, fft) * count(fft, dac) * count(dac, out)`
5. Sum the results of both orders.

**Key insight**: Since the graph is a DAG, we can count paths between any two nodes efficiently using memoization. This avoids the exponential explosion of traversing every single path individually.

**Why this works**: By breaking the path into segments defined by the required nodes, we transform a complex enumeration problem into simple path counting. Memoization ensures each node is processed only once per query.

**Complexity**: O(V + E). We visit each node and edge a constant number of times. This is exponentially faster than simple DFS for large graphs with many paths.

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

| Part   | Result          | Time   |
| :----- | :-------------- | :----- |
| Part 1 | 508             | ~350µs |
| Part 2 | 315116216513280 | ~700µs |
