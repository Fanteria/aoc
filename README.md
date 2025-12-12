# Advent of Code solutions

This repository contains my solutions for [Advent of Code](https://adventofcode.com/) challenges. Each solution (should) includes both the normal puzzle and bonus (Part 2) implementations, along with testing and benchmarking.

### `data` directory structure

Each day's directory contains:

- `in.txt` - Puzzle input
- `example_in.txt` - Example input from problem description
- `normal_out.txt` - Part 1 solution output
- `normal_example_out.txt` - Expected output for Part 1 example
- `bonus_out.txt` - Part 2 solution output
- `bonus_example_out.txt` - Expected output for Part 2 example

Some days include multiple examples (e.g., `example_2_in.txt`, `example_3_in.txt`) or have different examples for normal (`normal_example_in.txt`) and bonus (`bonus_example_in.txt`) puzzles. Data directories are expected to be in a private repository or only on local storage, not in this repository, because they should not be publicly redistributed. For more information, check the [about page](adventofcode.com/2025/about).

## Usage

### Running Solutions

```bash
# Run a specific day's solution
cargo run --release -- <year> <day>

# Examples:
cargo run --release -- 2024 1
cargo run --release -- 2025 9
```

For more check `--help`.

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for a specific day in all years
cargo test task_01

# Run test for specific day in specific year
cargo test --test 2025 task_01
```

### Benchmarking

Solutions are not optimized for performance. Purpose of benchmarks is for me to check which of two my implementation is faster and if some change that I expect improve performance really improve performance. So they are here just for fun and learning purposes.

```bash
# Run all benchmarks
cargo bench

# Benchmark a specific day
cargo bench day01
```

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

