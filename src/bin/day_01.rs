//! `cargo run --bin day_01`

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn main() -> (){
    let mut frequency = 0;

    for line in BufReader::new(File::open("src/data/day_1_input.txt").unwrap()).lines() {
        frequency = frequency + line.unwrap().parse::<i32>().unwrap();
    }

    println!("Part 1: {}", frequency);

    frequency = 0;

    let mut frequency_set: HashSet<i32> = HashSet::with_capacity(2048);
    let mut repeating_frequency_found = false;

    while !repeating_frequency_found {
        for line in BufReader::new(File::open("src/bin/day_1_input.txt").unwrap()).lines() {
            frequency = frequency + line.unwrap().parse::<i32>().unwrap();
            if !frequency_set.insert(frequency) {
                // frequency has been seen before, exit loops
                repeating_frequency_found = true;
                break;
            }
        }
    }

    println!("Part 2: {}", frequency);
}