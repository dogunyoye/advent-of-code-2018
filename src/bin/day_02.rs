//! `cargo run --bin day_02`

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> (){
    let mut twos_count = 0;
    let mut threes_count = 0;

    let mut found_two = false;
    let mut found_three = false;

    let mut lines_vec: Vec<String> = Vec::new();

    for line in BufReader::new(File::open("src/data/day_2_input.txt").unwrap()).lines() {
        let line = line.unwrap();
        let mut chars = line.chars();

        lines_vec.push(line.clone());

        let mut i = 0;
        while i < line.len() {
            let curr_char = chars.next().unwrap();
            let v: Vec<&str> = line.matches(curr_char).collect();

            if v.len() == 2 && !found_two {
                twos_count += 1;
                found_two = true;
            }
            else if v.len() == 3 && !found_three {
                threes_count += 1;
                found_three = true;
            }
            i += 1;
        }

        found_two = false;
        found_three = false;
    }

    println!("Part 1: {}", twos_count * threes_count);

    let mut mismatch_count = 0;
    let mut mismatch_index = 0;
    let mut found_pair = false;

    for i in 0..lines_vec.len() {
        let curr_line = lines_vec.get(i).unwrap();

        for j in i+1..lines_vec.len() {
            let next_line = lines_vec.get(j).unwrap();

            for k in 0..curr_line.len() {
                let c1 = curr_line.chars().nth(k).unwrap();
                let c2 = next_line.chars().nth(k).unwrap();

                if c1 != c2  {
                    mismatch_count += 1;
                    mismatch_index = k;
                    if mismatch_count > 1 {
                        // more than 1 mismatch, break early
                        // don't waste time iterating
                        mismatch_count = 0;
                        mismatch_index = 0;
                        break;
                    }
                }
            }

            if mismatch_count == 1 {
                found_pair = true;
                break;
            }
        }

        if found_pair {
            // stop iterating
            let mut cloned = curr_line.clone();
            cloned.remove(mismatch_index);
            println!("Part 2: {}", cloned);
            break;
        }
    }
}