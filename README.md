# advent-of-code-2018

Solutions to the [2018 Advent of Code](https://adventofcode.com/2018) written in Rust.

## Usage

Each day is a standalone binary:

```sh
cargo run --bin day_01      # day 1
cargo run --release --bin 17 # day 17 (input file lives in src/data)
```

Puzzle inputs are read from `src/data/day_<n>_input.txt`. These files are not
committed (per the Advent of Code request); download your own and place them
there before running a day.

## Layout

- `src/bin/day_NN.rs` — solution for each day.
- `src/data/` — puzzle inputs (git-ignored).
- `visualisations/` — Python scripts generating figures for a couple of days
  (run with `python3 visualisations/make_day_17.py`).
