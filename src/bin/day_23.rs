//! `cargo run --bin day_23`

use std::fs::File;
use std::io::{BufRead, BufReader, Result};

#[derive(Debug, Clone)]
struct Nanobot {
    x: i32,
    y: i32,
    z: i32,
    r: usize
}

fn main() -> Result<()> {

    let lines: Vec<String> = BufReader::new(File::open("src/data/day_23_input.txt")?).lines()
        .map(|l| l.unwrap()).collect();

    let mut nanobots: Vec<Nanobot> = Vec::new();

    let mut largest_radius: usize = 0;
    let mut largest_radius_nanobot = Nanobot {x: -1, y: -1, z: -1, r: 0};

    for line in lines {
        let vec = line.split_whitespace().collect::<Vec<&str>>();
        let replaced = vec[0].replace("pos=<", "").replace(">,", "");
        let radius = vec[1].replace("r=", "").parse::<usize>().unwrap();

        let pos_vec = replaced.split(",").collect::<Vec<&str>>();
        let nanobot = Nanobot { x: pos_vec[0].parse::<i32>().unwrap(),
                                y: pos_vec[1].parse::<i32>().unwrap(),
                                z: pos_vec[2].parse::<i32>().unwrap(),
                                r: radius};

        nanobots.push(nanobot.clone());

        if radius > largest_radius {
            largest_radius = radius;
            largest_radius_nanobot = nanobot;
        }
    }

    let mut in_range = 0;

    for nanobot in nanobots {
        let manhattan_distance = (nanobot.x - largest_radius_nanobot.x as i32).abs() +
                                 (nanobot.y - largest_radius_nanobot.y as i32).abs() +
                                 (nanobot.z - largest_radius_nanobot.z as i32).abs();

        if manhattan_distance <= largest_radius_nanobot.r as i32 {
            in_range += 1;
        }
    }

    println!("Part one: {}", in_range);

    Ok(())
}