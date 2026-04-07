# threadline

A minimal diff tool written in Rust. Compares two text files line by line and outputs what was added, removed, or unchanged.

## Usage

```bash
cargo build --release
./target/release/threadline a.txt b.txt
```

## How it works

Uses the Longest Common Subsequence (LCS) algorithm to find lines that appear in both files in the same order. Everything not in the LCS is marked as added or removed.

## Output

![demo](docs/screenshot.png)
