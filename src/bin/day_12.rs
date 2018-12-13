//! `cargo run --bin day_12`

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

use std::iter::FromIterator;

fn calculate_sum(capacity: usize, generations: i64) -> i64 {
    let mut state: Vec<char> = Vec::with_capacity(capacity);
    for _ in 0..capacity {
        state.push('.');
    }

    let padding: i32 = 5;
    let mut data_insert_index: usize = padding as usize;

    let mut result: Vec<char> = Vec::new();
    let mut rules_map: HashMap<String, char> = HashMap::new();

    for line in BufReader::new(File::open("src/data/day_12_input.txt").unwrap()).lines() {
        let line = line.unwrap();
        if line.contains("initial state") {
            let initial_state = line.split(" ").collect::<Vec<&str>>();
            let mut chars = initial_state[2].trim().chars();
            for c in chars {
                if let Some(initial_data) = state.get_mut(data_insert_index) {
                    *initial_data = c;
                }
                data_insert_index += 1;
            }
        }
        else if line.contains("=>") {
            let rule = line.split("=>").collect::<Vec<&str>>();
            rules_map.insert(rule[0].trim().to_string(), rule[1].trim().chars().nth(0).unwrap());
        }
    }

    for _ in 1..generations + 1 {
        for j in 0..state.len() {
            let mut evaluate: Vec<char> = Vec::new();

            // set up left side - LL
            if j == 0 {
                evaluate.push('.');
                evaluate.push('.');
            }
            else if j == 1 {
                evaluate.push('.');
                evaluate.push(state.get(j-1).unwrap().clone());
            }
            else {
                evaluate.push(state.get(j-2).unwrap().clone());
                evaluate.push(state.get(j-1).unwrap().clone());
            }

            // set up centre - C
            evaluate.push(state.get(j).unwrap().clone());

            //set up right side - RR
            if j == state.len() - 1 {
                evaluate.push('.');
                evaluate.push('.');
            }
            else if j == state.len() - 2 {
                evaluate.push('.');
                evaluate.push(state.get(j+1).unwrap().clone());
            }
            else {
                evaluate.push(state.get(j+1).unwrap().clone());
                evaluate.push(state.get(j+2).unwrap().clone());
            }

            let s = String::from_iter(evaluate.clone());

            if let Some(res) = rules_map.get(&s) {
                result.push(res.clone());
            }
            else {
                result.push('.');
            }
        }

        state.clear();
        state = result.clone();
        result.clear();
    }

    let mut sum: i64 = 0;

    for x in 0..state.len() {
        let index: i32 = x as i32;
        if *state.get(x).unwrap() == '#' {
            sum += (index - padding) as i64;
        }
    }

    sum
}

fn main() -> (){
    println!("Part 1: {}", calculate_sum(200, 20));
}