//! `cargo run --bin day_one`

use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashSet;

fn main() -> Result<()>{
    let mut frequency = 0;

    for line in BufReader::new(File::open("src/bin/day_one_input.txt")?).lines() {
        frequency = frequency + line?.parse::<i32>().unwrap();
    }

    println!("Part 1: {}", frequency);

    frequency = 0;

    let mut frequency_set: HashSet<i32> = HashSet::with_capacity(2048);
    let mut repeating_frequency_found = false;

    while !repeating_frequency_found {
        for line in BufReader::new(File::open("src/bin/day_one_input.txt")?).lines() {
            frequency = frequency + line?.parse::<i32>().unwrap();
            if !frequency_set.insert(frequency) {
                // frequency has been seen before, exit loops
                repeating_frequency_found = true;
                break;
            }
        }
    }

    println!("Part 2: {}", frequency);
    Ok(())
}