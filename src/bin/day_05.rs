//! `cargo run --bin day_05`

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;

fn main(){
    let mut input_string: String = String::new();
    let mut result = VecDeque::new();

    for line in BufReader::new(File::open("src/data/day_5_input.txt").unwrap()).lines() {
        input_string = line.unwrap().to_string();
    }

    for c in input_string.chars() {
        if let Some(prev_char) = result.front().copied() {
            if (c.is_uppercase() && c.to_ascii_lowercase() == prev_char) ||
                (c.is_lowercase() && c.to_ascii_uppercase() == prev_char) {
                result.pop_front();
                continue;
            }
        }

        result.push_front(c);
    }

    println!("Part 1: {:?}", result.len());

    result.clear();

    let mut smallest = 0;

    for i in 65..=90 {
        for c in input_string.chars() {
            if c as u8 == i || c as u8 == i + 32 {
                // skip this unit type
                continue;
            }

            if let Some(prev_char) = result.front().copied() {
                if (c.is_uppercase() && c.to_ascii_lowercase() == prev_char) ||
                    (c.is_lowercase() && c.to_ascii_uppercase() == prev_char) {
                    result.pop_front();
                    continue;
                }
            }

            result.push_front(c);
        }

        let size = result.len();
        if smallest == 0 || size < smallest {
            smallest = size;
        }

        result.clear();
    }

    println!("Part 2: {}", smallest);
}