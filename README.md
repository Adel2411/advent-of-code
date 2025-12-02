# Advent of Code 2025

My solutions for [Advent of Code 2025](https://adventofcode.com/2025), implemented in Rust 🦀.

## What is Advent of Code?

Advent of Code is an annual programming challenge event that runs during December. Each day, a new puzzle is released with a Christmas-themed story. The puzzles are designed to be solved with code in any programming language you choose.

### Key Features:

- **Daily Puzzles**: Each day from December 1st to 12th features a unique challenge
- **Two-Part Problems**: Every puzzle has two parts. The second part unlocks after completing the first
- **Star Rewards**: You earn one star per part completed (24 stars total in 2025)
- **Language Agnostic**: Solve puzzles in any programming language
- **No Time Pressure**: Puzzles remain accessible indefinitely

### 2025 Changes

This year brings some significant updates:

- **Shorter Duration**: The event runs for 12 days instead of the traditional 25
- **No Global Leaderboard**: The competitive global leaderboard has been discontinued to reduce stress and infrastructure pressure
- **Private Leaderboards**: You can still compete with friends using private leaderboards

## Project Structure

This repository is organized with each day as a separate Rust project:

```
advent-of-code/
├── README.md          # This file
├── day01/
│   ├── Cargo.toml     # Rust project configuration
│   ├── README.md      # Day 1 problem and solution explanation
│   ├── input.txt      # Puzzle input (unique to each user)
│   └── src/
│       ├── main.rs    # Entry point that runs both parts
│       ├── part1.rs   # Part 1 solution
│       └── part2.rs   # Part 2 solution
├── day02/
│   └── ...
```

Each day's folder contains:

- **README.md**: Problem summary and solution explanation
- **input.txt**: The puzzle input provided by Advent of Code
- **src/main.rs**: Runs both parts and displays results
- **src/part1.rs**: Solution for Part 1
- **src/part2.rs**: Solution for Part 2

## Getting Started

### Prerequisites

You'll need Rust installed on your system. If you don't have it yet:

1. Visit [rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2. Follow the installation instructions for your operating system
3. Verify installation by running: `rustc --version`

### Running a Solution

To run a specific day's solution:

```bash
# Navigate to the day's directory
cd day01

# Run both parts
cargo run

# Run in release mode (faster, useful for computationally heavy puzzles)
cargo run --release
```

## Progress Tracker

Track your progress through the challenges:

| Day | Part 1 | Part 2 | Notes                                            |
| :-: | :----: | :----: | ------------------------------------------------ |
| 01  |   ⭐   |   ⭐   | Circular dial simulation with modular arithmetic |
| 02  |   -    |   -    |                                                  |
| 03  |   -    |   -    |                                                  |
| 04  |   -    |   -    |                                                  |
| 05  |   -    |   -    |                                                  |
| 06  |   -    |   -    |                                                  |
| 07  |   -    |   -    |                                                  |
| 08  |   -    |   -    |                                                  |
| 09  |   -    |   -    |                                                  |
| 10  |   -    |   -    |                                                  |
| 11  |   -    |   -    |                                                  |
| 12  |   -    |   -    |                                                  |

Legend:

- ⭐ = Completed
- - = Not yet attempted

## Learning Resources

If you're new to Rust or Advent of Code:

### Rust Resources:

- [The Rust Book](https://doc.rust-lang.org/book/) - Comprehensive guide to Rust
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn through examples
- [Rustlings](https://github.com/rust-lang/rustlings) - Small exercises to get started

### Advent of Code Resources:

- [Official Website](https://adventofcode.com/)
- [Subreddit](https://www.reddit.com/r/adventofcode/) - Community discussions and help

## Contributing

This is a personal learning repository, but feel free to:

- Open issues if you spot bugs in my solutions
- Share alternative approaches or optimizations
- Use this as inspiration for your own solutions

## License

This project is open source and available for learning purposes. Puzzle statements and inputs are property of [Advent of Code](https://adventofcode.com/).
