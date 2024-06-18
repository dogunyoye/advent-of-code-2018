//! `cargo run --bin day_22`

use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
enum RegionType {
    Unknown,
    Rocky,
    Narrow,
    Wet
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
enum Equipment {
    Torch,
    ClimbingGear,
    Empty
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct Climber {
    equipment: Equipment,
    position: (i32, i32),
    cost: usize
}

impl Ord for Climber {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return other.cost.cmp(&self.cost);
    }
}

impl PartialOrd for Climber {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
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
                self.region_type = RegionType::Rocky;
            },
            1 => {
                self.region_char = '=';
                self.region_type = RegionType::Wet;
            },
            2 => {
                self.region_char = '|';
                self.region_type = RegionType::Narrow;
            },
            _ => panic!("Unknown erosion level: {}", self.erosion_level % 3)
        };
    }
}

#[allow(unused)]
fn print_map(grid: &Vec<Vec<Region>>, depth: usize, width: usize) {
    for i in 0..width {
        let mut line = "".to_owned();
        for j in 0..depth {
            line.push(grid[j][i].region_char);
        }
        println!("{}", line);
    }
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

    let default = Region { region_type: RegionType::Unknown, geologic_index: -1, erosion_level: -1, region_char: '?' };
    let mut grid: Vec<Vec<Region>> = vec![vec![default; grid_width]; grid_depth];

    grid[0][0] = Region {
        region_type: RegionType::Unknown,
        geologic_index: 0,
        erosion_level: depth_level % 20183,
        region_char: 'M'
    };

    grid[target_depth-1][target_width-1] = Region {
        region_type: RegionType::Unknown,
        geologic_index: 0,
        erosion_level: depth_level % 20183,
        region_char: 'T'
    };

    for x in 0..grid_width {
        for y in 0..grid_depth {
            if x == 0 && y == 0 {
                grid[y][x].determine_region_type();
                continue;
            }

            if y == target_depth-1 && x == target_width-1 {
                grid[y][x].determine_region_type();
                continue;
            }

            if y == 0 {
                grid[y][x].geologic_index = (x * 48271) as i32;
            }
            else if x == 0 {
                grid[y][x].geologic_index = (y * 16807) as i32;
            }
            else {
                grid[y][x].geologic_index = grid[y-1][x].erosion_level * grid[y][x-1].erosion_level;
            }

            grid[y][x].erosion_level = (grid[y][x].geologic_index + depth_level) % 20183;
            grid[y][x].determine_region_type();
        }
    }

    return (grid, grid_depth, grid_width);
}

fn djikstra(grid: &Vec<Vec<Region>>, target: (i32, i32)) -> usize {
    let mut frontier = BinaryHeap::new();
    let mut cost_so_far: HashMap<(i32, i32), usize> = HashMap::new();
    frontier.push(Climber{equipment: Equipment::Torch, position: (0, 0), cost: 0});

    while frontier.len() != 0 {
        let current_climber = frontier.pop().unwrap();
        if current_climber.position == target {
            return *cost_so_far.get(&target).unwrap();
        }

        let neighbours = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
        for n in neighbours {
            let next_pos = (current_climber.position.0 + n.0, current_climber.position.1 + n.1);

        }
    }

    return 0;
}

fn calculate_total_risk_level() -> usize {
    let (grid, depth, width) = build_map(None, None);
    let mut total_risk_level: usize = 0;

    for x in 0..width {
        for y in 0..depth {
            match grid[y][x].region_type {
                RegionType::Rocky => {},
                RegionType::Wet => total_risk_level += 1,
                RegionType::Narrow => total_risk_level += 2,
                _ => panic!("Unknown region type: {:?}", grid[x][y].region_type)
            }
        }
    }

    return total_risk_level;
}

fn find_fewest_number_of_minutes_to_reach_target() -> usize {
    let (grid, grid_depth, grid_width) = build_map(Some(5), Some(5));
    print_map(&grid, grid_depth, grid_width);
    return 0;
}

fn main() -> (){
    println!("Part one: {}", calculate_total_risk_level());
    println!("Part two: {}", find_fewest_number_of_minutes_to_reach_target());
}