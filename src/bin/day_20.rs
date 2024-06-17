//! `cargo run --bin day_20`

use std::{cmp, collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    i: i32,
    j: i32
}

#[allow(unused)]
fn print_map(map: &HashMap<Position, char>) {
    let min_j = map.keys().map(|p| p.j).min().unwrap();
    let max_j = map.keys().map(|p| p.j).max().unwrap();
    let min_i = map.keys().map(|p| p.i).min().unwrap();
    let max_i = map.keys().map(|p| p.i).max().unwrap();

    for i in min_i - 1..max_i + 2 {
        let mut line = "".to_owned();
        for j in min_j - 1..max_j + 2 {
            let p = Position{i, j};
            if map.get(&p).is_some() {
                line.push(*map.get(&p).unwrap());
            }
            else {
                line.push('#');
            }
        }
        println!("{}", line);
    }
}

fn move_position(dir: char, pos: &mut Position, map: &mut HashMap<Position, char>) {
    match dir {
        'N' => {
            map.insert(Position{i: pos.i - 1, j: pos.j}, '-');
            map.insert(Position{i: pos.i - 2, j: pos.j}, '.');
            pos.i -= 2;
        },
        'E' => {
            map.insert(Position{i: pos.i, j: pos.j + 1}, '|');
            map.insert(Position{i: pos.i, j: pos.j + 2}, '.');
            pos.j += 2;
        },
        'S' => {
            map.insert(Position{i: pos.i + 1, j: pos.j}, '-');
            map.insert(Position{i: pos.i + 2, j: pos.j}, '.');
            pos.i += 2;
        },
        'W' => {
            map.insert(Position{i: pos.i, j: pos.j - 1}, '|');
            map.insert(Position{i: pos.i, j: pos.j - 2}, '.');
            pos.j -= 2;
        },
        _ => panic!("Unknown direction: {}", dir),
    }
}

fn dfs(map: &HashMap<Position, char>) -> usize {
    let start: Position = Position{i: 0, j: 0};
    let steps: usize = 0;
    let mut visited: HashSet<Position> = HashSet::new();
    let mut stack: Vec<(Position, usize)> = Vec::new();
    let mut max_steps = usize::MIN;

    stack.push((start, steps));

    while stack.len() != 0 {
        let (current_position, current_steps) = stack.pop().unwrap();

        if !visited.contains(&current_position) {
            visited.insert(current_position);
            max_steps = cmp::max(max_steps, current_steps);

            let neighbours = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
            for n in neighbours {
                let mut next_pos = Position{i: current_position.i + n.0, j: current_position.j + n.1};
                let v = map.get(&next_pos);
                if v.is_none() {
                    continue;
                }
    
                if *v.unwrap() == '|' || *v.unwrap() == '-' {
                    next_pos.i = current_position.i + (2 * n.0);
                    next_pos.j = current_position.j + (2 * n.1);
    
                    if *map.get(&next_pos).unwrap() != '.' {
                        panic!("Found door without open position at: {:?}", next_pos);
                    }

                    stack.push((next_pos, current_steps + 1));
                }
            }
        }
    }

    return max_steps;
}

fn explore_iterative(regex: &String) -> HashMap<Position, char> {
    let mut map: HashMap<Position, char> = HashMap::new();
    let mut position = Position{i:0, j:0};
    let mut stack: Vec<Position> = Vec::new();

    for c in regex.chars() {
        match c {
            '(' => {
                stack.push(position.clone());
            },
            '|' => {
                position = stack.get(stack.len() - 1).unwrap().clone();
            },
            ')' => {
                position = stack.pop().unwrap();
            },
            _ => {
                move_position(c, &mut position, &mut map);
            }
        }
    }

    map.insert(Position{i: 0, j: 0}, '.');
    return map;
}

fn find_largest_number_of_door_to_reach_a_room() -> usize {
    let mut regex: String = "".to_string();

    for line in BufReader::new(File::open("src/data/day_20_input.txt").unwrap()).lines() {
        regex = line.unwrap();
        regex = regex[1..regex.len()-1].to_string();
    }

    let map: HashMap<Position, char> = explore_iterative(&regex);
    return dfs(&map);
}

fn main() -> (){
    println!("Part 1: {}", find_largest_number_of_door_to_reach_a_room());
}