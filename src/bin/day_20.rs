//! `cargo run --bin day_20`

use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

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

fn explore(regex: &String, mut pos: &mut Position, pointer: &mut usize, map: &mut HashMap<Position, char>) {
    let mut c: char;

    while *pointer != regex.len() - 1 {
        c = regex.chars().nth(*pointer).unwrap();

        if c == '(' {
            *pointer += 1;
            explore(regex, &mut pos.clone(), pointer, map);
            c = regex.chars().nth(*pointer).unwrap();
        }

        if c == '|' || c == ')' {
            *pointer += 1;
            return;
        }

        move_position(c, &mut pos, map);
        *pointer += 1;
    }
}

fn find_largest_number_of_door_to_reach_a_room() -> i32 {
    let mut regex: String = "".to_string();
    let mut map: HashMap<Position, char> = HashMap::new();
    let mut pointer: usize = 1;
    let mut position = Position{i:0, j:0};

    map.insert(position, 'X');

    for line in BufReader::new(File::open("src/data/day_20_input.txt").unwrap()).lines() {
        regex = line.unwrap();
    }

    println!("{}", regex);
    explore(&regex, &mut position, &mut pointer, &mut map);
    print_map(&map);

    return 0;
}

fn main() -> (){
    println!("Part 1: {}", find_largest_number_of_door_to_reach_a_room());
}