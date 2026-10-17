//! `cargo run --bin day_06`

use std::fs::File;
use std::io::{BufRead, BufReader};
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

fn main(){

    let mut largest_y = 0;
    let mut largest_x = 0;

    let mut coords_map: HashMap<String, Point> = HashMap::new();
    let mut coords_area: HashMap<String, i32> = HashMap::new();

    for (i, line) in (1..).zip(BufReader::new(File::open("src/data/day_6_input.txt").unwrap()).lines()) {
        let coord_vec = line.unwrap().split(",").collect::<Vec<&str>>().iter().map(|&x| x.to_owned()).collect::<Vec<String>>();
        let name_of_point = format!("Point{}", i);

        let point = Point {
                name: name_of_point.to_string(),
                x: coord_vec.first().unwrap().trim().parse().unwrap(),
                y: coord_vec.get(1).unwrap().trim().parse().unwrap()
        };

        coords_map.insert(name_of_point.to_string(), point.clone());
        coords_area.insert(name_of_point.to_string(), 0);

        largest_y = cmp::max(largest_y, point.y);
        largest_x = cmp::max(largest_x, point.x);
    }

    let unoccupied_point = Point { name: "unoccupied".to_string(), x: -1, y: -1};

    // initialise all points as unoccupied
    let unoccupied = Occupation { point: unoccupied_point, distance_from_point: -1, can_be_occupied: true};

    let size: usize = (cmp::max(largest_y, largest_x) + 1) as usize;

    let mut grid = vec![vec![unoccupied; size]; size];

    for point in coords_map.values() {

        let coord_occupation = Occupation { point: point.clone(), distance_from_point: 0, can_be_occupied: false };
        grid[point.x as usize][point.y as usize] = coord_occupation;

        for (x, row) in grid.iter_mut().enumerate() {
            for (y, cell) in row.iter_mut().enumerate() {

                if !cell.can_be_occupied {
                    // skip
                    continue;
                }

                let manhattan_distance = (point.x - x as i32).abs() + (point.y - y as i32).abs();

                if cell.point.name == "unoccupied" || cell.distance_from_point > manhattan_distance {
                    *cell = Occupation { point: point.clone(), distance_from_point: manhattan_distance, can_be_occupied: true};
                }
                else if cell.distance_from_point == manhattan_distance {
                    let equal_point = Point { name: "equal".to_string(), x: x as i32, y: y as i32};
                    *cell = Occupation { point: equal_point.clone(), distance_from_point: manhattan_distance, can_be_occupied: true};
                }
            }
        }
    }

    for (x, row) in grid.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            if let Some(total_area) = coords_area.get_mut(&cell.point.name) {
                *total_area += 1;
            }

            if x == 0 || y == 0 || x == size - 1 || y == size - 1 {
                // on the border so infinite => remove
                coords_area.remove(&cell.point.name);
            }
        }
    }

    let mut max_area = 0;

    for val in coords_area.values() {
        max_area = cmp::max(max_area, *val);
    }

    println!("Part 1: {}", max_area);

    let mut region_total = 0;

    for x in 0..size {
        for y in 0..size {
            let mut total_manhattan_distance = 0;
            for point in coords_map.values() {
                total_manhattan_distance += (point.x - x as i32).abs() + (point.y - y as i32).abs();
            }

            if total_manhattan_distance < 10000 {
                region_total += 1;
            }
        }
    }

    println!("Part 2: {}", region_total);
}