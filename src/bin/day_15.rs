//! `cargo run --bin day_15`

use std::{cmp::Ordering, collections::HashMap, fs::File, io::{BufRead, BufReader}};

#[derive(Debug, Clone, Copy, PartialEq)]
enum OccupantType {
    OpenField(char),
    Wall(char),
    Elf(char),
    Goblin(char)
}

impl OccupantType {
    fn unwrap_occupant(self) -> char {
        match self {
            OccupantType::OpenField(r) => r,
            OccupantType::Wall(r) => r,
            OccupantType::Elf(r) => r,
            OccupantType::Goblin(r) => r
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Occupant {
    position: Position,
    occ_type: OccupantType,
    attack_power: usize,
    hp: i32
}

impl Occupant {

    fn enemy(&mut self) -> OccupantType {
        return match self.occ_type {
            OccupantType::Elf(_) => OccupantType::Goblin('G'),
            OccupantType::Goblin(_) => OccupantType::Elf('E'),
            _ => panic!("No corresponding enemy for: {:?}", self.occ_type)
        };
    }

    fn adjacent_enemies(&mut self, arena: &HashMap<Position, Occupant>) -> Vec<Position> {
        let current_position = self.position;
        let mut neighbours: Vec<Position> = Vec::new();
        neighbours.push(Position{i: current_position.i - 1, j: current_position.j}); // north
        neighbours.push(Position{i: current_position.i, j: current_position.j + 1}); // east
        neighbours.push(Position{i: current_position.i + 1, j: current_position.j}); // south
        neighbours.push(Position{i: current_position.i, j: current_position.j - 1}); // west

        let enemy = self.enemy();
        neighbours = neighbours.into_iter().filter(|p| arena.get(p).unwrap().occ_type == enemy).collect();

        return neighbours;
    }

    fn move_or_attack(&mut self, arena: &HashMap<Position, Occupant>) {
        let adjacent_enemies = self.adjacent_enemies(arena);
        if adjacent_enemies.len() == 0 {
            // no enemies ajacent, move to the closest one
            let enemies: Vec<(&Position, &Occupant)> = arena.into_iter().filter(|(_, v)| v.occ_type == self.enemy()).collect();
            println!("{:?}", enemies);
            if enemies.len() == 0 {
                // no enemies, so can't move or attack
                return;
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    i: usize,
    j: usize
}

struct Node {
    parent: Position,
    current: Position
}

#[allow(unused)]
fn print_map(arena: &HashMap<Position, Occupant>) {
    let max_depth = arena.keys().map(|p| p.i).max().unwrap();
    let max_width = arena.keys().map(|p| p.j).max().unwrap();

    for i in 0..max_depth + 1 {
        let mut line = "".to_owned();
        for j in 0..max_width + 1 {
            let p = Position{i, j};
            if arena.contains_key(&p) {
                line.push(arena.get(&p).unwrap().occ_type.unwrap_occupant());
            }
        }
        println!("{}", line);
    }
}

fn position_sort_fn(a: &Position, b: &Position) -> Ordering {
    if a.i > b.i { return Ordering::Greater; }
    if a.i < b.i { return Ordering::Less; }

    if a.j > b.j { return Ordering::Greater; }
    if a.j < b.j { return Ordering::Less; }

    return Ordering::Equal;
}

fn find_elves_and_goblins(arena: &HashMap<Position, Occupant>) -> Vec<(Position, Occupant)> {
    let mut occupants: Vec<(Position, Occupant)> = Vec::new();
    for (key, value) in arena {
        match value.occ_type {
            OccupantType::Elf(_) | OccupantType::Goblin(_) => occupants.push((key.clone(), value.clone())),
            OccupantType::OpenField(_) | OccupantType::Wall(_) => {
                // no-op
            },
        }
    }
    occupants.sort_by(|a, b| position_sort_fn(&a.0, &b.0));
    return occupants;
}


fn build_arena() -> HashMap<Position, Occupant> {
    let mut arena: HashMap<Position, Occupant> = HashMap::new();
    let mut i: usize = 0;

    for line in BufReader::new(File::open("src/data/day_15_input.txt").unwrap()).lines() {
        let row = line.unwrap();
        for j in 0..row.len() {
            let pos = Position{i, j};
            let c =  row.chars().nth(j).unwrap();

            match c {
                '#' => {
                    arena.insert(pos, Occupant{position: pos, occ_type: OccupantType::Wall('#'), attack_power: 0, hp: -1});
                },
                '.' => {
                    arena.insert(pos, Occupant{position: pos, occ_type: OccupantType::OpenField('.'), attack_power: 0, hp: -1});
                },
                'E' => {
                    arena.insert(pos, Occupant{position: pos, occ_type: OccupantType::Elf('E'), attack_power: 3, hp: 200});
                },
                'G' => {
                    arena.insert(pos, Occupant{position: pos, occ_type: OccupantType::Goblin('G'), attack_power: 3, hp: 200});
                },
                _ => {
                    panic!("Unknown character: {}", c)
                }
            }
        }

        i += 1;
    }

    return arena;
}

fn find_outcome_of_battle() -> usize {
    let arena = build_arena();
    print_map(&arena);
    //println!("{:?}", find_elves_and_goblins(arena));
    let combatants = find_elves_and_goblins(&arena);
    for mut c in combatants {
        c.1.move_or_attack(&arena);
    }
    return 0;
}

fn main() -> (){
    println!("Part 1: {}", find_outcome_of_battle());
}