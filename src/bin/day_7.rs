//! `cargo run --bin day_7`

use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Clone)]
struct Worker {
    id: i32,
    step: char,
    time_left: i32
}

impl Worker {

    fn set_step(&mut self, step: char) {
        self.step = step.clone();
    }

    fn set_work_time(&mut self, work_time: i32) {
        self.time_left = work_time;
    }

    fn work(&mut self) {
        self.time_left -= 1;
    }
}

fn main() -> Result<()>{

    let mut dependencies_map: HashMap<char, Vec<char>> = HashMap::new();
    let mut dependencies_counter_map: HashMap<char, i32> = HashMap::new();

    let mut order: VecDeque<char> = VecDeque::new();

    for line in BufReader::new(File::open("src/data/day_7_input.txt")?).lines() {
        let line = line.unwrap();
        let mut chars = line.chars();
        let dependent: char = chars.nth(5).unwrap();
        let dependency: char = chars.nth(30).unwrap();

        if !order.contains(&dependent) {
            order.push_back(dependent.clone());
        }

        if !dependencies_map.contains_key(&dependent) {
            let mut dependencies_vec = Vec::new();
            dependencies_vec.push(dependency.clone());
            dependencies_map.insert(dependent.clone(), dependencies_vec);
        }
        else {
            if let Some(dependencies_vec) = dependencies_map.get_mut(&dependent) {
                dependencies_vec.push(dependency.clone());
                dependencies_vec.sort()
            }
        }

        if !dependencies_counter_map.contains_key(&dependency) {
            dependencies_counter_map.insert(dependency.clone(), 1);
        }
        else {
            if let Some(dependency_count) = dependencies_counter_map.get_mut(&dependency) {
                *dependency_count += 1;
            }
        }
    }

    let mut vec: Vec<char> = Vec::new();

    for (key, _val) in dependencies_map.iter() {
        if let None = dependencies_counter_map.get_mut(&key) {
            vec.push(key.clone());
        }
    }

    vec.sort();

    let mut result: Vec<char> = Vec::new();

    while vec.len() != 0 {
        let c = vec.remove(0);
        result.push(c);

        if let Some(list) = dependencies_map.get_mut(&c) {
            for x in list {
                if let Some(count) = dependencies_counter_map.get_mut(&x) {
                    *count -= 1;
                    if *count == 0 {
                        vec.push(x.clone());
                    }
                }
                else {
                    vec.push(x.clone());
                }
            }
            vec.sort();
        }
    }

    let mut result_string = String::new();
    for x in result {
        result_string.push(x);
    }
    println!("Part 1: {}", result_string);

    let mut result2: Vec<char> = Vec::new();
    let mut start_nodes: Vec<char> = Vec::new();

    for (key, _val) in dependencies_map.iter() {
        if let None = dependencies_counter_map.get_mut(&key) {
            start_nodes.push(key.clone());
        }
    }

    start_nodes.sort();

    let mut total_work_time: i32 = 0;
    //let initial_job_time = start_nodes.get(0).unwrap().clone();
    //total_work_time = ((initial_job_time as u8) - ('A' as u8) + 1) as i32;

    let worker1 = Worker { id: 1, step: '#', time_left: 0 };
    let worker2 = Worker { id: 2, step: '#', time_left: 0 };
//    let worker3 = Worker { id: 2, step: '#', time_left: 0 };
//    let worker4 = Worker { id: 2, step: '#', time_left: 0 };
//    let worker5 = Worker { id: 2, step: '#', time_left: 0 };

    Ok(())
}