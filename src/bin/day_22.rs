//! `cargo run --bin day_22`

use std::collections::{BinaryHeap, HashMap, HashSet};
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
    equipped: Equipment,
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
    for i in 0..depth {
        let mut line = "".to_owned();
        for j in 0..width {
            line.push(grid[i][j].region_char);
        }
        println!("{}", line);
    }
}

// reorientate the map in accordance with the example
fn translate_map(grid: &Vec<Vec<Region>>, depth: usize, width: usize) -> Vec<Vec<Region>> {
    let default = Region { region_type: RegionType::Unknown, geologic_index: -1, erosion_level: -1, region_char: '?' };
    let mut translated_grid: Vec<Vec<Region>> = vec![vec![default; depth]; width];

    for i in 0..width {
        let mut v: Vec<Region> = Vec::new();
        for j in 0..depth {
            v.push(grid[j][i].clone());
        }
        translated_grid[i] = v;
    }

    return translated_grid;
}

fn build_map(expanded_depth: Option<usize>, expanded_width: Option<usize>) -> (Vec<Vec<Region>>, usize, usize, (i32, i32)) {
    let lines: Vec<String> = BufReader::new(File::open("src/data/day_22_input.txt").unwrap()).lines()
    .map(|l| l.unwrap()).collect();

    let depth_level: i32 = lines[0][7..lines[0].len()].parse::<i32>().unwrap();
    let target: String = lines[1][8..lines[1].len()].to_string();

    let target_depth: usize = target.split(",").collect::<Vec<&str>>()[0].parse::<usize>().unwrap() + 1;
    let target_width: usize = target.split(",").collect::<Vec<&str>>()[1].parse::<usize>().unwrap() + 1;

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

    return (grid, grid_depth, grid_width, (target_width as i32 - 1, target_depth as i32 - 1));
}

fn is_position_valid(position: (i32, i32), depth: i32, width: i32) -> bool {
    return !(position.0 < 0 || position.1 < 0 || position.0 >= depth || position.1 >= width);
}

fn inventory_options(terrain: char) -> HashSet<Equipment> {
    match terrain {
        '.' => return HashSet::from([Equipment::ClimbingGear, Equipment::Torch]),
        '=' => return HashSet::from([Equipment::ClimbingGear, Equipment::Empty]),
        '|' => return HashSet::from([Equipment::Torch, Equipment::Empty]),
        _ => panic!("Unknown terrain: {}", terrain)
    }
}

fn djikstra(grid: &Vec<Vec<Region>>, target: (i32, i32), depth: i32, width: i32) -> usize {
    let mut frontier: BinaryHeap<Climber> = BinaryHeap::new();
    let mut cost_so_far: HashMap<(Equipment, (i32, i32)), usize> = HashMap::new();
    let mut result = usize::MAX;

    let initial_climber = Climber{equipped: Equipment::Torch, position: (0, 0), cost: 0};
    frontier.push(initial_climber);
    cost_so_far.insert((initial_climber.equipped, initial_climber.position), 0);

    while frontier.len() != 0 {
        let current_climber = frontier.pop().unwrap();
        let current_state = (current_climber.equipped, current_climber.position);

        if current_climber.position == target {
            if current_climber.equipped != Equipment::Torch {
                result = std::cmp::min(result, *cost_so_far.get(&current_state).unwrap() + 7);
            }
            else {
                result = std::cmp::min(result, *cost_so_far.get(&current_state).unwrap());
            }
        }

        if current_climber.cost <= *cost_so_far.get(&current_state).unwrap() {
            let current_terrain = grid[current_climber.position.0 as usize][current_climber.position.1 as usize].region_char;
            let neighbours = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

            for n in neighbours {
                let next_pos = (current_climber.position.0 + n.0, current_climber.position.1 + n.1);
                if is_position_valid(next_pos, depth, width) {
                    let next_terrain = grid[next_pos.0 as usize][next_pos.1 as usize].region_char;
                    let current_position_options = inventory_options(current_terrain);
                    let next_position_options = inventory_options(next_terrain);
    
                    let equippable: Vec<&Equipment> = current_position_options.intersection(&next_position_options).collect();
                    for e in equippable {
                        let mut climber_clone = current_climber.clone();
                        climber_clone.position = next_pos;

                        let mut new_cost = climber_clone.cost + 1;
                        if *e != climber_clone.equipped {
                            new_cost += 7;
                            climber_clone.equipped = *e
                        }

                        let next_state = (climber_clone.equipped, climber_clone.position);
                        if !cost_so_far.contains_key(&next_state) || new_cost < *cost_so_far.get(&next_state).unwrap() {
                            climber_clone.cost = new_cost;
                            cost_so_far.insert(next_state, new_cost);
                            frontier.push(climber_clone);
                        }
                    }
                }
            }
        }
    }

    return result;
}

fn calculate_total_risk_level() -> usize {
    let (grid, depth, width, _) = build_map(None, None);
    let mut total_risk_level: usize = 0;

    for x in 0..depth {
        for y in 0..width {
            match grid[x][y].region_type {
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
    let (grid, grid_depth, grid_width, target) = build_map(Some(100), Some(100));
    let translated_map: Vec<Vec<Region>> = translate_map(&grid, grid_depth, grid_width);
    return djikstra(&translated_map, target, grid_width as i32, grid_depth as i32);
}

fn main() -> (){
    println!("Part one: {}", calculate_total_risk_level());
    println!("Part two: {}", find_fewest_number_of_minutes_to_reach_target());
}