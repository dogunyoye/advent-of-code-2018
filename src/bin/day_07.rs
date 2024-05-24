//! `cargo run --bin day_07`

use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::{HashMap, HashSet};
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug)]
struct Worker {
    step: char,
    time_left: usize
}

impl Worker {

    fn set_step(&mut self, step: char) {
        self.step = step.clone();
    }

    fn set_work_time(&mut self, work_time: usize) {
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

    let mut start_nodes: Vec<char> = Vec::new();
    let mut workers: Vec<Worker> = Vec::new();
    let mut work_queue: Vec<char> = Vec::new();
    let mut time_taken = 0;
    let alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let mut dependents: HashMap<char, Vec<char>> = HashMap::new();
    let mut completed: HashSet<char> = HashSet::new();

    for (key, val) in dependencies_map.iter() {
        if let None = dependencies_counter_map.get_mut(&key) {
            start_nodes.push(key.clone());
        }

        for v in val {
            if !dependents.contains_key(v) {
                let mut parents: Vec<char> = Vec::<char>::new();
                parents.push(*key);
                dependents.insert(*v, parents);
            }
            else {
                dependents.get_mut(v).unwrap().push(*key)
            }
        }
    }

    start_nodes.sort();

    for _ in 0..5 {
        workers.push(Worker{ step: '#', time_left: 0})
    }

    for i in 0..start_nodes.len() {
        let w: &mut Worker = workers.get_mut(i).unwrap();
        let c = *start_nodes.get(i).unwrap();
        w.set_step(c);
        w.set_work_time(60 + alphabet.iter().position(|&r| r == c).unwrap() + 1);
    }

    while completed.len() != result_string.len() {
        time_taken += 1;

        let mut unlocked: HashSet<char> = HashSet::new();

        for e in &mut workers {
            if e.time_left != 0 {
                e.work();
                if e.time_left == 0 {
                    completed.insert(e.step);
                    for (k, v) in dependents.iter_mut() {
                        let idx = v.iter().position(|&r| r == e.step);
                        if idx.is_some() {
                            v.remove(idx.unwrap());
                            if v.len() == 0 {
                                unlocked.insert(*k);
                            }
                        }
                    }
                    dependents.remove_entry(&e.step);
                    e.set_step('#');
                }
            }
        }

        for u in unlocked {
            work_queue.push(u)
        }

        work_queue.sort();

        for e in &mut workers {
            if e.time_left == 0 {
                if work_queue.len() > 0 {
                    let s = work_queue.remove(0);
                    e.set_step(s);
                    e.set_work_time(60 + alphabet.iter().position(|&r| r == s).unwrap() + 1);
                }
            }
        }
    }

    println!("Part 2: {}", time_taken);

    Ok(())
}