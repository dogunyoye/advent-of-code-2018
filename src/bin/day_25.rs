//! `cargo run --bin day_25`

extern crate petgraph;

use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashSet;
use petgraph::unionfind::UnionFind;

#[derive(Clone, Debug)]
struct Point {
    w: i32,
    x: i32,
    y: i32,
    z: i32
}

fn main() -> Result<()>{
    let mut points: Vec<(Point, HashSet<usize>)> = Vec::new();

    for line in BufReader::new(File::open("src/data/day_25_input.txt")?).lines() {
        let data = line.unwrap();
        let coords = data.split(",").collect::<Vec<&str>>();

        let point = Point { w: coords[0].parse::<i32>().unwrap(),
                            x: coords[1].parse::<i32>().unwrap(),
                            y: coords[2].parse::<i32>().unwrap(),
                            z: coords[3].parse::<i32>().unwrap()};

        points.push((point, HashSet::new()));
    }

    let mut u: UnionFind<usize> = UnionFind::new(points.len());

    for i in 0..points.len() {
        for j in i+1..points.len() {

            let p1: (Point, HashSet<usize>) = points[i].clone();
            let p2: (Point, HashSet<usize>) = points[j].clone();

            let manhattan_distance = (p1.0.w - p2.0.w).abs() + (p1.0.x - p2.0.x).abs() +
                                          (p1.0.y - p2.0.y as i32).abs() + (p1.0.z - p2.0.z).abs();

            if manhattan_distance <= 3 {
                u.union(i, j);
            }
        }
    }

    let mut set = HashSet::new();
    let result = u.into_labeling();

    for i in 0..result.len() {
        set.insert(result[i]);
    }

    println!("Part one: {}", set.len());

    Ok(())
}