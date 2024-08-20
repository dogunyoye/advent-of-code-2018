//! `cargo run --bin day_25`

extern crate petgraph;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use petgraph::unionfind::UnionFind;

#[derive(Clone, Debug)]
struct Point {
    w: i32,
    x: i32,
    y: i32,
    z: i32
}

fn main() -> (){
    let mut points: Vec<Point> = Vec::new();

    for line in BufReader::new(File::open("src/data/day_25_input.txt").unwrap()).lines() {
        let data = line.unwrap();
        let coords = data.split(",").collect::<Vec<&str>>();

        let point = Point {
            w: coords[0].parse::<i32>().unwrap(),
            x: coords[1].parse::<i32>().unwrap(),
            y: coords[2].parse::<i32>().unwrap(),
            z: coords[3].parse::<i32>().unwrap()
        };

        points.push(point);
    }

    let mut u: UnionFind<usize> = UnionFind::new(points.len());

    for i in 0..points.len() {
        for j in i+1..points.len() {

            let p1: Point = points[i].clone();
            let p2: Point = points[j].clone();

            let manhattan_distance = (p1.w - p2.w).abs() + (p1.x - p2.x).abs() +
                                          (p1.y - p2.y).abs() + (p1.z - p2.z).abs();

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
}