//! `cargo run --bin day_18`

use std::collections::HashMap;
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

fn grid_to_string(grid: &Vec<Vec<Acre>>) -> String {
    return grid
        .iter()
        .flatten()
        .map(|x|
            if x.current == '#' {
                return "#"
            } else if x.current == '|' {
                return "|"
            } else {
                return "?"
            }
        )
        .collect::<Vec<_>>()
        .join(" ");
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

fn simulate(grid: &mut Vec<Vec<Acre>>, grid_size: usize) {
    for x in 0..grid_size {
        let x_coord = x as i32;

        for y in 0..grid_size {
            let current = grid[x][y].current;
            let mut adjacent_points: Vec<Point> = Vec::new();
            let y_coord = y as i32;

            for dx in [-1, 0, 1] {
                for dy in [-1, 0, 1] {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    adjacent_points.push(Point { x: x_coord + dx, y: y_coord + dy })
                }
            }

            let mut tree_count = 0;
            let mut lumberyard_count = 0;

            for p in adjacent_points {
                if p.x >= 0 && p.x < grid_size as i32 && p.y >= 0 && p.y < grid_size as i32 {
                    let acre: &Acre = &grid[p.x as usize][p.y as usize];
                    match acre.current {
                        '|' => {
                            tree_count += 1
                        },
                        '#' => {
                            lumberyard_count += 1
                        },
                        _ => {}
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
}

fn calculate_total(grid: &Vec<Vec<Acre>>, grid_size: usize) -> usize {
    let mut lumberyard_count = 0;
    let mut wooded_acre_count = 0;

    for x in 0..grid_size {
        for y in 0..grid_size {
            match grid[x][y].current {
                '#' => {
                    lumberyard_count += 1
                },
                '|' => {
                    wooded_acre_count += 1
                },
                _ => {}
            }
        }
    }

    lumberyard_count * wooded_acre_count
}

fn part1() -> () {
    let grid_size = 50;
    let mut grid = build_initial_grid();
    let mut minutes = 10;

    while minutes != 0 {
        simulate(&mut grid, grid_size);
        minutes -= 1;
    }

    println!("Part 1: {}", calculate_total(&grid, grid_size));
}

fn part2() -> () {
    let grid_size = 50;
    let mut grid = build_initial_grid();
    let mut minutes = 1000000000;

    let mut sequence_vec: Vec<usize> = Vec::new();
    let mut seen: HashMap<String, i32> = HashMap::new();

    while minutes != 0 {
        minutes -= 1;

        simulate(&mut grid, grid_size);
        let grid_string = grid_to_string(&grid);

        if seen.contains_key(&grid_string) {
            let cycle_start = seen.get(&grid_string).unwrap();
            let cycle_length = (1000000000 - minutes) - cycle_start;
            let remaining = (1000000000 - cycle_start) % cycle_length;
            let result = sequence_vec.get((cycle_start + remaining - 1) as usize).unwrap();
            println!("Part 2: {}", result);
            return;
        }

        seen.insert(grid_string, 1000000000 - minutes);
        sequence_vec.push(calculate_total(&grid, grid_size))
    }
}

fn main() -> (){
    part1();
    part2();
}