//! `cargo run --bin day_07`

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug)]
struct Worker {
    step: char,
    time_left: usize
}

impl Worker {

    fn set_step(&mut self, step: char) {
        self.step = step;
    }

    fn set_work_time(&mut self, work_time: usize) {
        self.time_left = work_time;
    }

    fn work(&mut self) {
        self.time_left -= 1;
    }
}

fn main(){

    let mut dependencies_map: HashMap<char, Vec<char>> = HashMap::new();
    let mut dependencies_counter_map: HashMap<char, i32> = HashMap::new();

    let mut order: VecDeque<char> = VecDeque::new();

    for line in BufReader::new(File::open("src/data/day_7_input.txt").unwrap()).lines() {
        let line = line.unwrap();
        let mut chars = line.chars();
        let dependent: char = chars.nth(5).unwrap();
        let dependency: char = chars.nth(30).unwrap();

        if !order.contains(&dependent) {
            order.push_back(dependent);
        }

        match dependencies_map.entry(dependent) {
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(vec![dependency]);
            }
            std::collections::hash_map::Entry::Occupied(mut e) => {
                e.get_mut().push(dependency);
                e.get_mut().sort();
            }
        }

        match dependencies_counter_map.entry(dependency) {
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(1);
            }
            std::collections::hash_map::Entry::Occupied(mut e) => {
                *e.get_mut() += 1;
            }
        }
    }

    let mut vec: Vec<char> = Vec::new();

    for key in dependencies_map.keys() {
        if dependencies_counter_map.get_mut(key).is_none() {
            vec.push(*key);
        }
    }

    vec.sort();

    let mut result: Vec<char> = Vec::new();

    while !vec.is_empty() {
        let c = vec.remove(0);
        result.push(c);

        if let Some(list) = dependencies_map.get_mut(&c) {
            for x in list {
                if let Some(count) = dependencies_counter_map.get_mut(x) {
                    *count -= 1;
                    if *count == 0 {
                        vec.push(*x);
                    }
                }
                else {
                    vec.push(*x);
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
        if dependencies_counter_map.get_mut(key).is_none() {
            start_nodes.push(*key);
        }

        for v in val {
            match dependents.entry(*v) {
                std::collections::hash_map::Entry::Vacant(e) => {
                    e.insert(vec![*key]);
                }
                std::collections::hash_map::Entry::Occupied(mut e) => {
                    e.get_mut().push(*key);
                }
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
                        if let Some(idx) = v.iter().position(|&r| r == e.step) {
                            v.remove(idx);
                            if v.is_empty() {
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
            if e.time_left == 0
                && !work_queue.is_empty() {
                    let s = work_queue.remove(0);
                    e.set_step(s);
                    e.set_work_time(60 + alphabet.iter().position(|&r| r == s).unwrap() + 1);
                }
        }
    }

    println!("Part 2: {}", time_taken);
}