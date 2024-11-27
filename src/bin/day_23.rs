//! `cargo run --bin day_23`

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Nanobot {
    x: i32,
    y: i32,
    z: i32,
    r: usize
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct CubeState {
    cube: Cube,
    bots_in_range: usize,
    distance_to_start: usize
}

impl Ord for CubeState {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the comparison order
        // we compare self with other to get the max heap (most bots in range)
        // we compare other with self to get the min heap (closest distance to start)
        self.bots_in_range.cmp(&other.bots_in_range)
            .then_with(|| other.distance_to_start.cmp(&self.distance_to_start))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for CubeState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Cube {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    min_z: i32,
    max_z: i32,
    centre: (i32, i32, i32)
}

impl Cube {

    fn is_unit_cube(&mut self) -> bool {
        return ((self.max_x - self.min_x) <= 1) && ((self.max_y - self.min_y) <= 1) && ((self.max_z - self.min_z) <= 1)
    }

    fn split(&mut self) -> Vec<Cube> {
        let middle_x = (self.min_x + self.max_x)/2;
        let middle_y = (self.min_y + self.max_y)/2;
        let middle_z = (self.min_z + self.max_z)/2;
        
        let c0 = build_cube(vec![self.min_x, middle_x, self.min_y, middle_y, self.min_z, middle_z]);
        let c1 = build_cube(vec![self.min_x, middle_x, middle_y, self.max_y, self.min_z, middle_z]);
        let c2 = build_cube(vec![self.min_x, middle_x, self.min_y, middle_y, middle_z, self.max_z]);
        let c3 = build_cube(vec![self.min_x, middle_x, middle_y, self.max_y, middle_z, self.max_z]);
        let c4 = build_cube(vec![middle_x, self.max_x, self.min_y, middle_y, self.min_z, middle_z]);
        let c5 = build_cube(vec![middle_x, self.max_x, middle_y, self.max_y, self.min_z, middle_z]);
        let c6 = build_cube(vec![middle_x, self.max_x, self.min_y, middle_y, middle_z, self.max_z]);
        let c7 = build_cube(vec![middle_x, self.max_x, middle_y, self.max_y, middle_z, self.max_z]);

        return vec![c0, c1, c2, c3, c4, c5, c6, c7];
    }

    fn corners(&mut self) -> Vec<(i32, i32, i32)> {
        let mut corners: Vec<(i32, i32, i32)> = Vec::new();
        for x in vec![self.min_x, self.max_x] {
            for y in vec![self.min_y, self.max_y] {
                for z in vec![self.min_z, self.max_z] {
                    corners.push((x, y, z));
                }
            }
        }
        return corners;
    }
}

fn manhattan_distance(a: (i32, i32, i32), b: (i32, i32, i32)) -> usize {
    return ((a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs()) as usize;
}

fn nanobots_in_range_of_cube(nanobots: &Vec<Nanobot>, mut cube: Cube) -> usize {
    let mut bots_in_range: HashSet<Nanobot> = HashSet::new();
    for nanobot in nanobots {
        let nanobot_position = (nanobot.x, nanobot.y, nanobot.z);
        for c in &cube.corners() {
            if nanobot.r >= manhattan_distance(nanobot_position, *c) {
                bots_in_range.insert(nanobot.clone());
                break; 
            }
        }
    }
    return bots_in_range.len();
}

fn nanobots_in_range_of_point(nanobots: &Vec<Nanobot>, point: (i32, i32, i32)) -> usize {
    let mut bots_in_range: HashSet<Nanobot> = HashSet::new();
    for nanobot in nanobots {
        let nanobot_position = (nanobot.x, nanobot.y, nanobot.z);
        if nanobot.r >= manhattan_distance(nanobot_position, point) {
            bots_in_range.insert(nanobot.clone());
        }
    }
    return bots_in_range.len();
}

fn build_cube(bounds: Vec<i32>) -> Cube {
    let min_x = bounds[0];
    let max_x = bounds[1];
    let min_y = bounds[2];
    let max_y = bounds[3];
    let min_z = bounds[4];
    let max_z = bounds[5];

    let mut corners: Vec<(i32, i32, i32)> = Vec::new();
    for x in vec![min_x, max_x] {
        for y in vec![min_y, max_y] {
            for z in vec![min_z, max_z] {
                corners.push((x, y, z));
            }
        }
    }

    let centre_x = (max_x + min_x)/2;
    let centre_y = (max_y + min_y)/2;
    let centre_z = (max_z + min_z)/2;

    let centre = (centre_x, centre_y, centre_z);

    return Cube{min_x, max_x, min_y, max_y, min_z, max_z, centre};

}

fn build_bounding_box(nanobots: &Vec<Nanobot>) -> Cube {
    let mut bounds: Vec<i32> = Vec::new();
    bounds.push(nanobots.iter().map(|n| n.x).min().unwrap());
    bounds.push(nanobots.iter().map(|n| n.x).max().unwrap());
    bounds.push(nanobots.iter().map(|n| n.y).min().unwrap());
    bounds.push(nanobots.iter().map(|n| n.y).max().unwrap());
    bounds.push(nanobots.iter().map(|n| n.z).min().unwrap());
    bounds.push(nanobots.iter().map(|n| n.z).max().unwrap());

    return build_cube(bounds);
}

fn build_nanobots() -> (Vec<Nanobot>, Nanobot) {
    let lines: Vec<String> = BufReader::new(File::open("src/data/day_23_input.txt").unwrap()).lines()
        .map(|l| l.unwrap()).collect();

    let mut nanobots: Vec<Nanobot> = Vec::new();

    let mut largest_radius: usize = 0;
    let mut largest_radius_nanobot = Nanobot {x: -1, y: -1, z: -1, r: 0};

    for line in lines {
        let vec = line.split_whitespace().collect::<Vec<&str>>();
        let replaced = vec[0].replace("pos=<", "").replace(">,", "");
        let radius = vec[1].replace("r=", "").parse::<usize>().unwrap();

        let pos_vec = replaced.split(",").collect::<Vec<&str>>();
        let nanobot = Nanobot {
                                x: pos_vec[0].parse::<i32>().unwrap(),
                                y: pos_vec[1].parse::<i32>().unwrap(),
                                z: pos_vec[2].parse::<i32>().unwrap(),
                                r: radius};

        nanobots.push(nanobot.clone());

        if radius > largest_radius {
            largest_radius = radius;
            largest_radius_nanobot = nanobot;
        }
    }

    return (nanobots, largest_radius_nanobot);
}

fn calculate_nanobots_in_range_of_largest() -> i32 {
    let (nanobots, largest_radius_nanobot) = build_nanobots();
    let mut in_range = 0;
    for nanobot in nanobots {
        let nanobot_position = (nanobot.x, nanobot.y, nanobot.z);
        let largest_nanonbot_position = (largest_radius_nanobot.x, largest_radius_nanobot.y, largest_radius_nanobot.z);
        if manhattan_distance(nanobot_position, largest_nanonbot_position) <= largest_radius_nanobot.r {
            in_range += 1;
        }
    }

    return in_range; 
}

fn octree_scan(cube: Cube, nanobots: &Vec<Nanobot>) -> usize {
    let mut most_nanobots_in_range = usize::MIN;
    let mut shortest_distance_from_origin = usize::MAX;

    let mut pq: BinaryHeap<CubeState> = BinaryHeap::new();
    pq.push(CubeState{cube, bots_in_range: nanobots.len(), distance_to_start: manhattan_distance(cube.centre, (0, 0, 0))});

    while pq.len() != 0 {
        let current_cube_state = pq.pop().unwrap();
        let mut current_cube = current_cube_state.cube;
        
        if current_cube_state.bots_in_range < most_nanobots_in_range {
            continue;
        }

        if current_cube.is_unit_cube() {
            let bots = nanobots_in_range_of_point(nanobots, current_cube.centre);
            if bots >= most_nanobots_in_range {
                if bots == most_nanobots_in_range {
                    shortest_distance_from_origin = std::cmp::min(shortest_distance_from_origin, manhattan_distance((0, 0, 0), current_cube.centre));
                }
                else {
                    most_nanobots_in_range = bots;
                    shortest_distance_from_origin = manhattan_distance((0, 0, 0), current_cube.centre);
                }
            }
        } else {
            let cubes = current_cube.split();
            for c in cubes {
                let in_range = nanobots_in_range_of_cube(nanobots, c);
                if in_range > 0 {
                    let distance = manhattan_distance(c.centre, (0, 0, 0));
                    pq.push(CubeState{cube: c, bots_in_range: in_range, distance_to_start: distance});
                }
            }
        }
    }

    return shortest_distance_from_origin;
}

fn find_shortest_manhattan_distance_between_point_in_range_of_most_nanobots() -> usize {
    let (nanobots, _) = build_nanobots();
    let bounding_box = build_bounding_box(&nanobots);
    return octree_scan(bounding_box, &nanobots);
}

fn main() -> (){
    println!("Part one: {}", calculate_nanobots_in_range_of_largest());
    println!("Part two: {}", find_shortest_manhattan_distance_between_point_in_range_of_most_nanobots());
}