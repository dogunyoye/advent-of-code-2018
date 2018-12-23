//! `cargo run --bin day_22`

use std::fs::File;
use std::io::{BufRead, BufReader, Result};

#[derive(Debug, Clone)]
enum RegionType {
    UNKNOWN,
    ROCKY,
    NARROW,
    WET
}

#[derive(Debug, Clone)]
struct Region {
    region_type: RegionType,
    geologic_index: i32,
    erosion_level: i32,
    region_char: char
}

impl Region {

    fn determine_region_type(&mut self) -> () {
        match self.erosion_level % 3 {
            0 => {
                self.region_char = '.';
                self.region_type = RegionType::ROCKY;
            },
            1 => {
                self.region_char = '=';
                self.region_type = RegionType::WET;
            },
            2 => {
                self.region_char = '|';
                self.region_type = RegionType::NARROW;
            },
            _ => panic!("Unknown erosion level: {}", self.erosion_level % 3)
        };
    }
}

fn main() -> Result<()> {

    let lines: Vec<String> = BufReader::new(File::open("src/data/day_22_input.txt")?).lines()
        .map(|l| l.unwrap()).collect();

    let depth_level: i32 = lines[0][7..lines[0].len()].parse::<i32>().unwrap();
    let target: String = lines[1][8..lines[1].len()].to_string();

    let depth: usize = target.split(",").collect::<Vec<&str>>()[0].parse::<usize>().unwrap() + 1;
    let width: usize = target.split(",").collect::<Vec<&str>>()[1].parse::<usize>().unwrap() + 1;

    let default = Region { region_type: RegionType::UNKNOWN, geologic_index: -1, erosion_level: -1, region_char: '?' };
    let mut grid: Vec<Vec<Region>> = vec![vec![default; width]; depth];

    grid[0][0] = Region {
        region_type: RegionType::UNKNOWN,
        geologic_index: 0,
        erosion_level: depth_level % 20183,
        region_char: 'M'
    };

    grid[depth-1][width-1] = Region {
        region_type: RegionType::UNKNOWN,
        geologic_index: 0,
        erosion_level: depth_level % 20183,
        region_char: 'T'
    };

    for x in 0..depth {
        for y in 0..width {
            if x == 0 && y == 0 {
                grid[x][y].determine_region_type();
                continue;
            }

            if x == depth-1 && y == width-1 {
                grid[x][y].determine_region_type();
                continue;
            }

            if x == 0 {
                grid[x][y].geologic_index = (y * 48271) as i32;
            }
            else if y == 0 {
                grid[x][y].geologic_index = (x * 16807) as i32;
            }
            else {
                grid[x][y].geologic_index = grid[x-1][y].erosion_level * grid[x][y-1].erosion_level;
            }

            grid[x][y].erosion_level = (grid[x][y].geologic_index + depth_level) % 20183;
            grid[x][y].determine_region_type();
        }
    }

    let mut total = 0;

    for x in 0..depth {
        for y in 0..width {
            match grid[x][y].region_type {
                RegionType::ROCKY => {},
                RegionType::WET => total += 1,
                RegionType::NARROW => total += 2,
                _ => panic!("Unknown region type: {:?}", grid[x][y].region_type)
            }
        }
    }

    println!("Part one: {}", total);

    Ok(())
}