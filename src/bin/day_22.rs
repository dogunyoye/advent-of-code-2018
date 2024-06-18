//! `cargo run --bin day_22`

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
enum RegionType {
    UNKNOWN,
    ROCKY,
    NARROW,
    WET
}

#[derive(Debug, Clone, Copy)]
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

#[allow(unused)]
fn print_map(grid: &Vec<Vec<Region>>, depth: usize, width: usize) {
    for i in 0..depth {
        let mut line = "".to_owned();
        for j in 0..width {
            line.push(grid[i][j].region_char);
        }
        println!("{}", line);
    }
}

fn normalise_map(grid: &Vec<Vec<Region>>, depth: usize, width: usize) -> Vec<Vec<Region>> {
    let default = Region { region_type: RegionType::UNKNOWN, geologic_index: -1, erosion_level: -1, region_char: '?' };
    let mut normalised_grid: Vec<Vec<Region>> = vec![vec![default; width]; depth];
    for i in 0..depth {
        for j in 0..width {
            normalised_grid[i][j] = grid[j][i].clone();
        }
    }
    return normalised_grid;
}

fn build_map(expanded_depth: Option<usize>, expanded_width: Option<usize>) -> (Vec<Vec<Region>>, usize, usize) {
    let lines: Vec<String> = BufReader::new(File::open("src/data/day_22_input.txt").unwrap()).lines()
    .map(|l| l.unwrap()).collect();

    let depth_level: i32 = lines[0][7..lines[0].len()].parse::<i32>().unwrap();
    let target: String = lines[1][8..lines[1].len()].to_string();

    let target_depth = target.split(",").collect::<Vec<&str>>()[0].parse::<usize>().unwrap() + 1;
    let target_width = target.split(",").collect::<Vec<&str>>()[1].parse::<usize>().unwrap() + 1;

    let mut grid_depth: usize = target_depth;
    let mut grid_width: usize = target_width;
    if expanded_depth.is_some() && expanded_width.is_some() {
        grid_depth += expanded_depth.unwrap();
        grid_width += expanded_width.unwrap();
    }

    let default = Region { region_type: RegionType::UNKNOWN, geologic_index: -1, erosion_level: -1, region_char: '?' };
    let mut grid: Vec<Vec<Region>> = vec![vec![default; grid_width]; grid_depth];

    grid[0][0] = Region {
        region_type: RegionType::UNKNOWN,
        geologic_index: 0,
        erosion_level: depth_level % 20183,
        region_char: 'M'
    };

    grid[target_depth-1][target_width-1] = Region {
        region_type: RegionType::UNKNOWN,
        geologic_index: 0,
        erosion_level: depth_level % 20183,
        region_char: 'T'
    };

    for x in 0..grid_depth {
        for y in 0..grid_width {
            if x == 0 && y == 0 {
                grid[x][y].determine_region_type();
                continue;
            }

            if x == target_depth-1 && y == target_width-1 {
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

    return (grid, grid_depth, grid_width);
}

fn calculate_total_risk_level() -> usize {
    let (grid, depth, width) = build_map(None, None);
    let mut total_risk_level: usize = 0;

    for x in 0..depth {
        for y in 0..width {
            match grid[x][y].region_type {
                RegionType::ROCKY => {},
                RegionType::WET => total_risk_level += 1,
                RegionType::NARROW => total_risk_level += 2,
                _ => panic!("Unknown region type: {:?}", grid[x][y].region_type)
            }
        }
    }

    return total_risk_level;
}

fn find_fewest_number_of_minutes_to_reach_target() -> usize {
    let (grid, grid_depth, grid_width) = build_map(Some(5), Some(5));
    let normalised_grid = normalise_map(&grid, grid_depth, grid_width);
    print_map(&normalised_grid, grid_depth, grid_width);
    return 0;
}

fn main() -> (){
    println!("Part one: {}", calculate_total_risk_level());
    println!("Part two: {}", find_fewest_number_of_minutes_to_reach_target());
}