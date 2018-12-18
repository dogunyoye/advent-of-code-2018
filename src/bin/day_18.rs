//! `cargo run --bin day_18`

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug)]
struct Acre {
    current: char,
    next: char
}

struct Point {
    x: i32,
    y: i32
}

fn build_initial_grid() -> Vec<Vec<Acre>> {
    let default = Acre { current: '?', next: '?' };
    let grid_size = 50;
    let mut grid = vec![vec![default; grid_size]; grid_size];

    let mut x = 0;
    for line in BufReader::new(File::open("src/data/day_18_input.txt").unwrap()).lines() {
        let line = line.unwrap();
        let mut y = 0;
        for c in line.chars() {
            grid[x][y] = Acre { current: c, next: '?' };
            y += 1;
        }
        x += 1;
    }

    grid
}

fn part1() -> () {

    let grid_size = 50;
    let mut grid = build_initial_grid();
    let mut minutes = 10;

    while minutes != 0 {
        for x in 0..grid_size {
            let x_coord = x as i32;

            for y in 0..grid_size {
                let current = grid[x][y].current;
                let mut adjacent_points: Vec<Point> = Vec::new();
                let y_coord = y as i32;

                let top_left = Point { x: x_coord - 1, y: y_coord - 1 };
                let top = Point { x: x_coord - 1, y: y_coord };
                let top_right = Point { x: x_coord - 1, y: y_coord + 1 };
                let left = Point { x: x_coord, y: y_coord - 1 };
                let right = Point { x: x_coord, y: y_coord + 1 };
                let bottom_left = Point { x: x_coord + 1, y: y_coord - 1 };
                let bottom = Point { x: x_coord + 1, y: y_coord };
                let bottom_right = Point { x: x_coord + 1, y: y_coord + 1 };

                adjacent_points.push(top_left);
                adjacent_points.push(top);
                adjacent_points.push(top_right);
                adjacent_points.push(left);
                adjacent_points.push(right);
                adjacent_points.push(bottom_left);
                adjacent_points.push(bottom);
                adjacent_points.push(bottom_right);

                let mut tree_count = 0;
                let mut lumberyard_count = 0;

                for p in adjacent_points {
                    if p.x >= 0 && p.x < grid_size as i32 && p.y >= 0 && p.y < grid_size as i32 {
                        let acre: &Acre = &grid[p.x as usize][p.y as usize];
                        if acre.current == '|' {
                            tree_count += 1;
                        }
                        else if acre.current == '#' {
                            lumberyard_count += 1;
                        }
                    }
                }

                match current {
                    '.' => {
                        if tree_count >= 3 {
                            grid[x][y].next = '|';
                        }
                        else {
                            grid[x][y].next = current;
                        }
                    },
                    '|' => {
                        if lumberyard_count >= 3 {
                            grid[x][y].next = '#';
                        }
                        else {
                            grid[x][y].next = current;
                        }
                    },
                    '#' => {
                        if lumberyard_count >= 1 && tree_count >= 1 {
                            grid[x][y].next = current;
                        }
                        else {
                            grid[x][y].next = '.'
                        }
                    },
                    _ => {}
                }
            }
        }

        for x in 0..grid_size {
            for y in 0..grid_size {
                grid[x][y].current = grid[x][y].next;
            }
        }

        minutes -= 1;
    }

    let mut lumberyard_count = 0;
    let mut wooded_acre_count = 0;

    for x in 0..grid_size {
        for y in 0..grid_size {
            if grid[x][y].current == '#' {
                lumberyard_count += 1;
            }
            else if grid[x][y].current == '|' {
                wooded_acre_count += 1
            }
        }
    }

    println!("Part 1: {}", lumberyard_count * wooded_acre_count);
}

fn part2() -> () {

    let grid_size = 50;
    let mut grid = build_initial_grid();
    let mut minutes = 1000000000;

    let mut len = 0;
    let mut sequence_started = false;
    let mut sequence_vec: Vec<i32> = Vec::new();

    while minutes != 0 {
        for x in 0..grid_size {

            let x_coord = x as i32;
            for y in 0..grid_size {
                let current = grid[x][y].current;
                let mut adjacent_points: Vec<Point> = Vec::new();
                let y_coord = y as i32;

                let top_left = Point { x: x_coord - 1, y: y_coord - 1 };
                let top = Point { x: x_coord - 1, y: y_coord };
                let top_right = Point { x: x_coord - 1, y: y_coord + 1 };
                let left = Point { x: x_coord, y: y_coord - 1 };
                let right = Point { x: x_coord, y: y_coord + 1 };
                let bottom_left = Point { x: x_coord + 1, y: y_coord - 1 };
                let bottom = Point { x: x_coord + 1, y: y_coord };
                let bottom_right = Point { x: x_coord + 1, y: y_coord + 1 };

                adjacent_points.push(top_left);
                adjacent_points.push(top);
                adjacent_points.push(top_right);
                adjacent_points.push(left);
                adjacent_points.push(right);
                adjacent_points.push(bottom_left);
                adjacent_points.push(bottom);
                adjacent_points.push(bottom_right);

                let mut tree_count = 0;
                let mut lumberyard_count = 0;

                for p in adjacent_points {
                    if p.x >= 0 && p.x < grid_size as i32 && p.y >= 0 && p.y < grid_size as i32 {
                        let acre: &Acre = &grid[p.x as usize][p.y as usize];
                        if acre.current == '|' {
                            tree_count += 1;
                        }
                        else if acre.current == '#' {
                            lumberyard_count += 1;
                        }
                    }
                }

                match current {
                    '.' => {
                        if tree_count >= 3 {
                            grid[x][y].next = '|';
                        }
                        else {
                            grid[x][y].next = current;
                        }
                    },
                    '|' => {
                        if lumberyard_count >= 3 {
                            grid[x][y].next = '#';
                        }
                        else {
                            grid[x][y].next = current;
                        }
                    },
                    '#' => {
                        if lumberyard_count >= 1 && tree_count >= 1 {
                            grid[x][y].next = current;
                        }
                        else {
                            grid[x][y].next = '.'
                        }
                    },
                    _ => {}
                }
            }
        }

        let mut lumberyard_count = 0;
        let mut wooded_acre_count = 0;

        for x in 0..grid_size {
            for y in 0..grid_size {
                grid[x][y].current = grid[x][y].next;
                if grid[x][y].current == '#' {
                    lumberyard_count += 1;
                }
                else if grid[x][y].current == '|' {
                    wooded_acre_count += 1
                }
            }
        }

        let total = lumberyard_count * wooded_acre_count;

        // Some "magic number" programming.. I noticed that there is
        // a 28 number sequence after the 400th minute (iteration) by
        // printing out totals to stdout. There is probably a general
        // way of determining this
        if total == 172765 {
            if !sequence_started {
                len = 1000000000 - minutes - 1;
                sequence_started = true;
            }
            else {
                break;
            }
        }

        if sequence_started {
            sequence_vec.push(total);
        }

        minutes -= 1;
    }

    let index = ((1000000000 - len) % sequence_vec.len()) - 2;
    println!("Part 2: {}", sequence_vec.get(index).unwrap());
}

fn main() -> (){
    part1();
    part2();
}