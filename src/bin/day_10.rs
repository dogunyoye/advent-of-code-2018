//! `cargo run --bin day_10`

use std::fs::File;
use std::io::{BufRead, BufReader, Result};

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
    x_velocity: i32,
    y_velocity: i32
}

fn main() -> Result<()>{

    let mut points: Vec<Point> = Vec::new();

    for line in BufReader::new(File::open("src/data/day_10_input.txt")?).lines() {
        let data = line.unwrap().replace("position=", "").replace("velocity=", "")
            .replace(">", ""). replace("<", "").replace(",", "");

        let vec: Vec<i32> = data.trim().split_whitespace().map(|d| d.trim().parse::<i32>().unwrap()).collect();
        points.push(Point { x: vec[0], y: vec[1], x_velocity: vec[2], y_velocity: vec[3] });
    }

    let len = points.len();
    let mut count = 0;

    let grid_size = 80;

    loop {
        let mut grid = vec![vec!['.'; grid_size]; grid_size];
        let mut all_points_in_sight = false;

        let min_x = points.iter().map(|point| point.x).min().unwrap();
        let min_y = points.iter().map(|point| point.y).min().unwrap();

        for i in 0..len {
            let mut p: &mut Point = &mut points[i];
            let x = p.x - min_x;
            let y = p.y - min_y;

            if x < grid_size as i32  && y < grid_size as i32 {
                grid[x as usize][y as usize] = '#';
                all_points_in_sight = true;
            }
            else {
                all_points_in_sight = false;
            }
        }

        for i in 0..len {
            points[i].x += points[i].x_velocity;
            points[i].y += points[i].y_velocity;
        }

        if all_points_in_sight {

            let mut break_condition = 0;
            for x in 0..10 {
                if grid[x][0] == '#' {
                    break_condition += 1;
                }

                print!("{}|", count);
                for y in 0..80 {
                    print!("{}", grid[y][x]);
                }
                println!();
            }

            if break_condition == 6 {
                println!("Part one");
                break;
            }

            println!();
        }

        count += 1;
    }

    println!();
    println!("Part two: {} secs", count);

    Ok(())
}