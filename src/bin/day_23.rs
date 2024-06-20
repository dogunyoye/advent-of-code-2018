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

#[derive(Debug, Clone)]
struct Cube {
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
    min_z: f32,
    max_z: f32,
    corners: Vec<(f32, f32, f32)>
}

impl Cube {
    fn split(&mut self) -> Vec<Cube> {
        let middle_x = (self.min_x + self.max_x)/2.0;
        let middle_y = (self.min_y + self.max_y)/2.0;
        let middle_z = (self.min_z + self.max_z)/2.0;
        
        let c0 = build_cube(vec![self.min_x, middle_x, self.min_y, middle_y, self.min_z, middle_z]);
        let c1 = build_cube(vec![self.min_x, middle_x, middle_y, self.max_y, self.min_z, middle_z]);
        let c2 = build_cube(vec![self.min_x, middle_x, self.min_y, middle_y, middle_z, self.max_z]);
        let c3 = build_cube(vec![self.min_x, middle_x, middle_y, self.max_y, middle_z, self.max_z]);
        let c4 = build_cube(vec![middle_x, self.max_x, self.min_y, middle_y, self.min_z, middle_z]);
        let c5 = build_cube(vec![middle_x, self.max_x, middle_y, self.max_y, self.min_z, middle_z]);
        let c6 = build_cube(vec![middle_x, self.max_x, self.min_y, middle_y, middle_z, self.max_z]);
        let c7 = build_cube(vec![middle_x, self.max_x, middle_y, self.max_y, middle_z, self.max_z]);

        return vec![c0, c1, c2, c3, c4, c5, c6, c7];
    }
}

fn manhattan_distance(first: &Nanobot, second: &Nanobot) -> i32 {
    return (first.x - second.x as i32).abs() + (first.y - second.y as i32).abs() + (first.z - second.z as i32).abs();
}

fn build_cube(bounds: Vec<f32>) -> Cube {
    let min_x = bounds[0];
    let max_x = bounds[1];
    let min_y = bounds[2];
    let max_y = bounds[3];
    let min_z = bounds[4];
    let max_z = bounds[5];

    let mut corners: Vec<(f32, f32, f32)> = Vec::new();
    for x in vec![min_x, max_x] {
        for y in vec![min_y, max_y] {
            for z in vec![min_z, max_z] {
                corners.push((x, y, z));
            }
        }
    }

    return Cube{min_x, max_x, min_y, max_y, min_z, max_z, corners};

}

fn build_bounding_box(nanobots: Vec<Nanobot>) -> Cube {
    let mut bounds: Vec<f32> = Vec::new();
    bounds.push(nanobots.iter().map(|n| n.x).min().unwrap() as f32);
    bounds.push(nanobots.iter().map(|n| n.x).max().unwrap() as f32);
    bounds.push(nanobots.iter().map(|n| n.y).min().unwrap() as f32);
    bounds.push(nanobots.iter().map(|n| n.y).max().unwrap() as f32);
    bounds.push(nanobots.iter().map(|n| n.z).min().unwrap() as f32);
    bounds.push(nanobots.iter().map(|n| n.z).max().unwrap() as f32);

    return build_cube(bounds);
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
    let (nanobots, _) = build_nanobots();
    let bounding_box = build_bounding_box(nanobots);
    println!("cube: {:?}", bounding_box);
    return 0;
}

fn main() -> (){
    println!("Part one: {}", calculate_nanobots_in_range_of_largest());
    println!("Part two: {}", find_shortest_manhattan_distance_between_point_in_range_of_most_nanobots());
}