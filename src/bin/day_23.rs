//! `cargo run --bin day_23`

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Nanobot {
    x: i32,
    y: i32,
    z: i32,
    r: usize
}

fn manhattan_distance(first: &Nanobot, second: &Nanobot) -> i32 {
    return (first.x - second.x as i32).abs() + (first.y - second.y as i32).abs() + (first.z - second.z as i32).abs();
}

fn build_nanobots() -> (Vec<Nanobot>, Nanobot) {
    let lines: Vec<String> = BufReader::new(File::open("src/data/day_23_input.txt").unwrap()).lines()
        .map(|l| l.unwrap()).collect();

    let mut nanobots: Vec<Nanobot> = Vec::new();

    let mut largest_radius: usize = 0;
    let mut largest_radius_nanobot = Nanobot {x: -1, y: -1, z: -1, r: 0};

    for line in lines {
        let vec = line.split_whitespace().collect::<Vec<&str>>();
        let replaced = vec[0].replace("pos=<", "").replace(">,", "");
        let radius = vec[1].replace("r=", "").parse::<usize>().unwrap();

        let pos_vec = replaced.split(",").collect::<Vec<&str>>();
        let nanobot = Nanobot {
                                x: pos_vec[0].parse::<i32>().unwrap(),
                                y: pos_vec[1].parse::<i32>().unwrap(),
                                z: pos_vec[2].parse::<i32>().unwrap(),
                                r: radius};

        nanobots.push(nanobot.clone());

        if radius > largest_radius {
            largest_radius = radius;
            largest_radius_nanobot = nanobot;
        }
    }

    return (nanobots, largest_radius_nanobot);
}

fn calculate_nanobots_in_range_of_largest() -> i32 {
    let (nanobots, largest_radius_nanobot) = build_nanobots();
    let mut in_range = 0;
    for nanobot in nanobots {
        if manhattan_distance(&nanobot, &largest_radius_nanobot) <= largest_radius_nanobot.r as i32 {
            in_range += 1;
        }
    }

    return in_range; 
}

fn find_shortest_manhattan_distance_between_point_in_range_of_most_nanobots() -> i32 {
    return 0;
}

fn main() -> (){
    println!("Part one: {}", calculate_nanobots_in_range_of_largest());
    println!("Part two: {}", find_shortest_manhattan_distance_between_point_in_range_of_most_nanobots());
}