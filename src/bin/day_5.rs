//! `cargo run --bin day_5`

use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::VecDeque;

fn main() -> Result<()>{
    let mut input_string: String = String::new();
    let mut result = VecDeque::new();

    for line in BufReader::new(File::open("src/data/day_5_input.txt")?).lines() {
        input_string = line?.to_string();
    }

    let mut chars = input_string.chars();

    for i in 0..input_string.len() {

        let c = chars.next().unwrap().clone();

        if i == 0 || result.len() == 0 {
            result.push_front(c);
            continue;
        }

        let prev_char = result.front().unwrap().clone();

        if (c.is_uppercase() && c.to_ascii_lowercase() == prev_char) ||
            (c.is_lowercase() && c.to_ascii_uppercase() == prev_char) {
            result.pop_front();
        }
        else {
            result.push_front(c);
        }
    }

    println!("Part 1: {:?}", result.len());

    result.clear();

    let mut smallest = 0;
    let mut smallest_queue = VecDeque::new();

    for i in 65..90 {
        let mut chars_iter = input_string.chars();

        for j in 0..input_string.len() {

            let c = chars_iter.next().unwrap().clone();

            if j == 0 || result.len() == 0 {
                result.push_front(c);
                continue;
            }

            let prev_char = result.front().unwrap().clone();

            if c as u8 == i || c as u8 == i + 32 {
                // skip
                continue;
            }

            if (c.is_uppercase() && c.to_ascii_lowercase() == prev_char) ||
                (c.is_lowercase() && c.to_ascii_uppercase() == prev_char) {
                result.pop_front();
            }
            else {
                result.push_front(c);
            }
        }

        let size = result.len();
        if smallest == 0 ||  size < smallest {
            smallest = size;
            smallest_queue.push_front(format!("Smallest {}, letters {}/{}", smallest, i as u8 as char, (i + 32) as u8 as char));
        }

        result.clear();
    }

    println!("Part 2: {}", smallest_queue.pop_front().unwrap());


    Ok(())
}