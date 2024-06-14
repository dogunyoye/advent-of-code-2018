//! `cargo run --bin day_17`

use std::{borrow::BorrowMut, collections::{HashMap, VecDeque}, fs::File, io::{BufRead, BufReader, LineWriter, Write}};


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

    let file = File::create("day17_debug_part1.txt").unwrap();
    let mut file = LineWriter::new(file);

    for i in min_i..max_i + 1 {
        let mut line = "".to_owned();
        for j in min_j..max_j + 1 {
            let p = Position{i, j};
            line.push(*map.get(&p).unwrap())
        }

        file.write_all(line.as_bytes());
        file.write_all(b"\n");
    }
}

fn build_map() -> (HashMap<Position, char>, i32, i32) {
    let mut map: HashMap<Position, char> = HashMap::new();

    for line in BufReader::new(File::open("src/data/day_17_input.txt").unwrap()).lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(", ").collect();

        let mut xs: Vec<i32> = Vec::new();
        let mut ys: Vec<i32> = Vec::new();

        for i in 0..2 {
            let p: Vec<&str> = parts.get(i).unwrap().split("=").collect();
            let axis = p.get(0).unwrap();
            let axis_val = p.get(1).unwrap();

            let axis_vec: &mut Vec<i32>;
            if *axis == "x" {
                axis_vec = xs.borrow_mut()
            }
            else {
                axis_vec = ys.borrow_mut()
            }

            if axis_val.contains("..") {
                let range: Vec<&str> = axis_val.split("..").collect();
                let lower = range.get(0).unwrap().parse::<i32>().unwrap();
                let higher = range.get(1).unwrap().parse::<i32>().unwrap() + 1;
                for v in lower..higher {
                    axis_vec.push(v)
                }
            }
            else {
                axis_vec.push(axis_val.parse::<i32>().unwrap());
            }
        }

        for i in &ys {
            for j in &xs {
                map.insert(Position{i: *i, j: *j}, '#');
            }
        }
    }

    let min_j = map.keys().map(|p| p.j).min().unwrap();
    let max_j = map.keys().map(|p| p.j).max().unwrap();
    let min_i = map.keys().map(|p| p.i).min().unwrap();
    let max_i = map.keys().map(|p| p.i).max().unwrap();

    for i in 0..min_i {
        for j in min_j-1..max_j+2 {
            map.insert(Position{i, j}, '.');
        }
    }

    for i in min_i..max_i+1 {
        for j in min_j-1..max_j+2 {
            let p = Position{i, j};
            if !map.contains_key(&p) {
                map.insert(p, '.');
            }
        }
    }

    return (map, min_i, max_i);
}

fn drop(drop_point: Position, map: &mut HashMap<Position, char>) -> Option<Position> {
    let mut p: Position = Position{i: drop_point.i + 1, j: drop_point.j};
    loop {
        let s: Option<&char> = map.get(&p);
        if s.is_none() {
            return None;
        }
        // if landed on position known to be unsettled
        if *s.unwrap() == '|' {
            return None;
        }

        if *s.unwrap() == '#' {
            p.i -= 1;
            return Some(p);
        }

        map.insert(p, '|');
        p.i += 1;
    }
}

fn fill_row(pos: Position, map: &mut HashMap<Position, char>) -> Vec<Position> {
    let mut left: Vec<Position> = Vec::new();
    let mut right: Vec<Position> = Vec::new();
    let mut drop_points: Vec<Position> = Vec::new();

    let i = pos.i;
    let mut left_j = pos.j - 1;
    let mut right_j = pos.j + 1;
    let mut overflow: bool = false;

    // move left
    while *map.get(&Position{i, j: left_j}).unwrap() != '#' {
        let south = *map.get(&Position{i: i + 1, j: left_j}).unwrap();
        left.push(Position{i, j: left_j});
        if south == '.' || south == '|' {
            drop_points.push(Position{i, j: left_j});
            overflow = true;
            break;
        }
        left.push(Position{i, j: left_j});
        left_j -= 1;
    }

    // move right
    while *map.get(&Position{i, j: right_j}).unwrap() != '#' {
        let south = *map.get(&Position{i: i + 1, j: right_j}).unwrap();
        right.push(Position{i, j: right_j});
        if south == '.' || south == '|' {
            drop_points.push(Position{i, j: right_j});
            overflow = true;
            break;
        }
        right_j += 1;
    }

    let mut fill = '~';
    if overflow {
        fill = '|'
    }

    map.insert(pos, fill);

    for lp in left {
        map.insert(lp, fill);
    }

    for rp in right {
        map.insert(rp, fill);
    }

    return drop_points;
}

fn find_number_of_water_reaching_tiles() -> (i32, i32) {
    let (mut map, min_i, max_i) = build_map();
    let origin: Position = Position{i: 0, j: 500};
    let mut drop_points: VecDeque<Position> = VecDeque::new();
    drop_points.push_back(origin);

    while drop_points.len() != 0 {
        let dp = drop_points.pop_front().unwrap();
        let sp = drop(dp, &mut map);
        if sp.is_some() {
            let mut p = sp.unwrap();
            loop {
                let next_drop_points = fill_row(p, &mut map);
                if next_drop_points.len() != 0 {
                    for ndp in next_drop_points {
                        drop_points.push_back(ndp)
                    }
                    break;
                }
                p.i -= 1;
            }
        }
    }

    let mut part_one = 0;
    let mut part_two = 0;
    let positions: Vec<Position> = map.keys().filter(|p| p.i >= min_i && p.i <= max_i).copied().collect();
    for p in positions {
        let v = *map.get(&p).unwrap();
        if v == '|' {
            part_one += 1;
        }

        if v == '~' {
            part_two += 1;
        }
    }

    return (part_one + part_two, part_two);
}

fn main() -> (){
    let (part_one, part_two) = find_number_of_water_reaching_tiles();
    println!("Part 1: {}", part_one);
    println!("Part 2: {}", part_two);
}