//! `cargo run --bin day_03`

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn main() -> (){

    let mut grid = vec![vec!["#".to_string(); 1000]; 1000];

    let mut claim_ids_set = HashSet::new();

    for line in BufReader::new(File::open("src/data/day_3_input.txt").unwrap()).lines() {
        let split_vec = line.unwrap().split("@").collect::<Vec<&str>>().iter().map(|&x| x.to_owned()).collect::<Vec<String>>();

        claim_ids_set.insert(split_vec.get(0).unwrap().trim().to_string());
        let claim_id = split_vec.get(0).unwrap().trim().to_string();

        let fabric_vec = split_vec.get(1).unwrap().trim().split(":").collect::<Vec<&str>>();

        let distances_vec = fabric_vec.get(0).unwrap().split(",").collect::<Vec<&str>>();
        let left_distance: usize = distances_vec.get(0).unwrap().parse().unwrap();
        let top_distance: usize = distances_vec.get(1).unwrap().parse().unwrap();

        let dimensions_vec = fabric_vec.get(1).unwrap().trim().split("x").collect::<Vec<&str>>();
        let rectangle_width: usize = dimensions_vec.get(0).unwrap().parse().unwrap();
        let rectangle_height: usize = dimensions_vec.get(1).unwrap().parse().unwrap();

        for i in 0..rectangle_width {
            for j in 0..rectangle_height {
                let current_value = grid[top_distance+j][left_distance+i].clone();
                if current_value == "#" {
                    // unoccupied
                    grid[top_distance+j][left_distance+i] = claim_id.clone();
                }
                else if current_value == "X" {
                    // already overlapping, remove from set
                    claim_ids_set.remove(&claim_id.to_string());
                }
                else {
                    // occupied by a claim id, therefore an overlap will occur
                    grid[top_distance+j][left_distance+i] = "X".to_string();
                    claim_ids_set.remove(&current_value);
                    claim_ids_set.remove(&claim_id.to_string());
                }
            }
        }
    }

    let mut x_count = 0;

    for (_i, row) in grid.iter_mut().enumerate() {
        for (_y, col) in row.iter_mut().enumerate() {
            if col == "X" {
                x_count += 1;
            }
        }
    }

    println!("Part 1: {}", x_count);

    for i in claim_ids_set.drain() {
        println!("Part 2: {}", i);
    }
}