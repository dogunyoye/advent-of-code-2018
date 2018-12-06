//! `cargo run --bin day_6`

use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashMap;
use std::cmp;

#[derive(Clone)]
struct Point {
    name: String,
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Occupation {
    point: Point,
    distance_from_point: i32,
    can_be_occupied: bool
}

fn main() -> Result<()>{

    let mut largest_y = 0;
    let mut largest_x = 0;
    let mut i = 1;

    let mut coords_map: HashMap<String, Point> = HashMap::new();
    let mut coords_area: HashMap<String, i32> = HashMap::new();

    for line in BufReader::new(File::open("src/bin/day_6_input.txt")?).lines() {
        let coord_vec = line.unwrap().split(",").collect::<Vec<&str>>().iter().map(|&x| x.to_owned()).collect::<Vec<String>>();
        let name_of_point = format!("Point{}", i);

        let point = Point {
                name: name_of_point.to_string(),
                x: coord_vec.get(0).unwrap().trim().parse().unwrap(),
                y: coord_vec.get(1).unwrap().trim().parse().unwrap()
        };

        coords_map.insert(name_of_point.to_string(), point.clone());
        coords_area.insert(name_of_point.to_string(), 0);

        largest_y = cmp::max(largest_y, point.y);
        largest_x = cmp::max(largest_x, point.x);

        i += 1;
    }

    let unoccupied_point = Point { name: "unoccupied".to_string(), x: -1, y: -1};

    // initialise all points as unoccupied
    let unoccupied = Occupation { point: unoccupied_point, distance_from_point: -1, can_be_occupied: true};

    let size: usize = (cmp::max(largest_y, largest_x) + 1) as usize;

    let mut grid = vec![vec![unoccupied; size]; size];

    for (_point_name, point) in coords_map.iter() {

        let coord_occupation = Occupation { point: point.clone(), distance_from_point: 0, can_be_occupied: false };
        grid[point.x as usize][point.y as usize] = coord_occupation;

        for x in 0..size {
            for y in 0..size {

                if !grid[x][y].can_be_occupied {
                    // skip
                    continue;
                }

                let manhattan_distance = (point.x - x as i32).abs() + (point.y - y as i32).abs();

                if grid[x][y].point.name == "unoccupied" || grid[x][y].distance_from_point > manhattan_distance {
                    grid[x][y] = Occupation { point: point.clone(), distance_from_point: manhattan_distance, can_be_occupied: true};
                }
                else if grid[x][y].distance_from_point == manhattan_distance {
                    let equal_point = Point { name: "equal".to_string(), x: x as i32, y: y as i32};
                    grid[x][y] = Occupation { point: equal_point.clone(), distance_from_point: manhattan_distance, can_be_occupied: true};
                }
            }
        }
    }

    for x in 0..size {
        for y in 0..size {
            if let Some(total_area) = coords_area.get_mut(&grid[x][y].point.name) {
                *total_area += 1;
            }

            if x == 0 || y == 0 || x == size as usize - 1 || y == size as usize - 1 {
                // on the border so infinite => remove
                coords_area.remove(&grid[x][y].point.name);
            }
        }
    }

    let mut max_area = 0;

    for (_key, val) in coords_area.iter() {
        max_area = cmp::max(max_area, *val);
    }

    println!("Part 1: {}", max_area);

    let mut region_total = 0;

    for x in 0..size {
        for y in 0..size {
            let mut total_manhattan_distance = 0;
            for (_point_name, point) in coords_map.iter() {
                total_manhattan_distance += (point.x - x as i32).abs() + (point.y - y as i32).abs();
            }

            if total_manhattan_distance < 10000 {
                region_total += 1;
            }
        }
    }

    println!("Part 2: {}", region_total);

    Ok(())
}