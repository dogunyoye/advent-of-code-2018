//! `cargo run --bin day_15`

use std::{borrow::BorrowMut, cmp::Ordering, collections::{HashMap, HashSet, VecDeque}, fs::File, io::{BufRead, BufReader}};

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
    id: usize,
    position: Position,
    occ_type: OccupantType,
    attack_power: i32,
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

    fn move_or_attack(&mut self, arena: &mut HashMap<Position, Occupant>) -> (bool, Option<Occupant>) {
        let mut adjacent_enemies = self.adjacent_enemies(arena);

        if adjacent_enemies.len() == 0 {
            // no enemies ajacent, move to the closest one
            let enemies: Vec<Position> = arena.iter_mut().filter(|(_, v)| v.occ_type.unwrap_occupant() == self.enemy().unwrap_occupant()).map(|(k, _)| k).copied().collect();
            if enemies.len() == 0 {
                return (true, None);
            }

            let paths: Vec<Vec<(Vec<Position>, Position)>> =
                enemies.into_iter()
                .map(|e| {
                    let start_open_adjacent_positions = open_adjacent_positions(self.position, arena);
                    let open_adjacent_positions = open_adjacent_positions(e, arena);
                    let mut paths_to_adjacent: Vec<(Vec<Position>, Position)> = Vec::new();
                    for s in start_open_adjacent_positions {
                        for o in &open_adjacent_positions {
                            let path = bfs(s, *o, self.enemy(), arena);
                            if path.len() != 0 {
                                paths_to_adjacent.push((path, o.clone()));
                            }
                        }
                    }
                    return paths_to_adjacent;
                })
                .collect();

            let mut paths_to_enemies: Vec<(Position, usize, Position)> = Vec::new();

            for a in &paths {
                for b in &*a {
                    paths_to_enemies.push((*b.0.get(0).unwrap(), b.0.len() + 1, b.1.clone()));
                }
            }

            if paths_to_enemies.len() == 0 {
                // no path to enemy, so can't move or attack
                return (false, None);
            }

            paths_to_enemies.sort_by(|a, b| movement_sort_fn(a, b));

            let next_pos = paths_to_enemies.get(0).unwrap().0;
            let mut current_occupant = arena.get(&self.position).unwrap().clone();
            let field_occupant = Occupant{id: 0, position: self.position.clone(), occ_type: OccupantType::OpenField('.'), attack_power: 0, hp: 0};

            current_occupant.position = next_pos;
            arena.insert(next_pos, current_occupant);
            arena.insert(self.position, field_occupant);

            self.position = next_pos;
        }

        adjacent_enemies = self.adjacent_enemies(arena);
        if adjacent_enemies.len() == 0 {
            // moved but still not a position
            // adjacent to an enemy.
            return (false, None);
        }

        let mut enemies: Vec<Occupant> = arena.iter().filter(|(k, _)| adjacent_enemies.contains(k)).map(|(_, v)| v).copied().collect();
        enemies.sort_by(|a, b| attack_sort_fn(a, b));
        let mut occupant: Occupant = *enemies.get(0).unwrap();
        occupant.hp -= self.attack_power;

        arena.insert(occupant.position, occupant);

        if occupant.hp <= 0 {
            let defeated_occupant = occupant.clone();
            let field_occupant = Occupant{id: 0, position: occupant.position.clone(), occ_type: OccupantType::OpenField('.'), attack_power: 0, hp: 0};
            arena.insert(occupant.position.clone(), field_occupant);
            return (false, Some(defeated_occupant));
        }

        return (false, None);

    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    i: i32,
    j: i32
}

#[allow(unused)]
fn print_arena(arena: &HashMap<Position, Occupant>) {
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

#[allow(unused)]
fn print_combatants(arena: &HashMap<Position, Occupant>) {
    for (_, v) in arena {
        let occ_type = v.occ_type.unwrap_occupant();
        if occ_type == 'G' || occ_type == 'E' {
            println!("{:?}", v);
        }
    }
}

fn open_adjacent_positions(enemy_position: Position, arena: &mut HashMap<Position, Occupant>) -> Vec<Position> {
    let neighbours: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut free: Vec<Position> = Vec::new();
    for n in neighbours {
        let next_pos = Position{i: enemy_position.i + n.0, j: enemy_position.j + n.1};
        let occ_type = arena.get(&next_pos).unwrap().occ_type.unwrap_occupant();
        if occ_type == '.' {
            free.push(next_pos)
        }
    }

    return free;
}

#[allow(unused)]
// Breadth First Search to find all paths from start to end
// unused as it works for the part 1 example cases, but not the actual input (memory blows up)
fn bfs_all_paths(start: Position, end: Position, enemy: OccupantType, arena: &mut HashMap<Position, Occupant>) -> Vec<Vec<Position>> {
    if open_adjacent_positions(end, arena).len() == 0 {
        // if there is no free space next
        // to the enemy, there is no path
        return vec![];
    }

    let mut queue: VecDeque<(Position, Vec<Position>)> = VecDeque::new();
    let mut paths: Vec<Vec<Position>> = Vec::new();
    let mut current_min: usize = usize::MAX;

    queue.push_back((start, vec![start]));

    while queue.len() != 0 {
        let node = queue.pop_front().unwrap();
        let current_position = node.0;
        let current_path = node.1;

        if current_position == end {
            current_min = current_path.len().min(current_min);
            paths.push(current_path);
            continue;
        }

        let neighbours: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
        for n in neighbours {
            let next_pos = Position{i: current_position.i + n.0, j: current_position.j + n.1};
            if current_path.contains(&next_pos) {
                continue;
            }

            let occ_type = arena.get(&next_pos).unwrap().occ_type.unwrap_occupant();
            if occ_type == '.' || (occ_type == enemy.unwrap_occupant() && next_pos == end) {
                let mut path_copy = current_path.clone();
                path_copy.push(next_pos);

                if path_copy.len() >= current_min {
                    continue;
                }

                queue.push_back((next_pos, path_copy));
            }
        }
    }

    return paths;
}

fn bfs(start: Position, end: Position, enemy: OccupantType, arena: &mut HashMap<Position, Occupant>) -> Vec<Position> {
    let mut queue: VecDeque<(Position, Vec<Position>)> = VecDeque::new();
    let mut visited: HashSet<Position> = HashSet::new();

    queue.push_back((start, vec![start]));
    visited.insert(start);

    while queue.len() != 0 {
        let (current_position, current_path) = queue.pop_front().unwrap();

        if current_position == end {
            return current_path;
        }

        let neighbours: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
        for n in neighbours {
            let next_pos = Position{i: current_position.i + n.0, j: current_position.j + n.1};
            if visited.contains(&next_pos) {
                continue;
            }

            let occ_type = arena.get(&next_pos).unwrap().occ_type.unwrap_occupant();
            if occ_type == '.' || (occ_type == enemy.unwrap_occupant()) {
                let mut path_copy: Vec<Position> = current_path.clone();
                path_copy.push(next_pos);
                queue.push_back((next_pos, path_copy));
                visited.insert(next_pos);
            }
        }
    }

    // no path to end position
    return vec![];
}

fn position_sort_fn(a: &Position, b: &Position) -> Ordering {
    if a.i > b.i { return Ordering::Greater; }
    if a.i < b.i { return Ordering::Less; }

    if a.j > b.j { return Ordering::Greater; }
    if a.j < b.j { return Ordering::Less; }

    return Ordering::Equal;
}

fn movement_sort_fn(a: &(Position, usize, Position), b: &(Position, usize, Position)) -> Ordering {
    if a.1 > b.1 { return Ordering::Greater; }
    if a.1 < b.1 { return Ordering::Less; }

    let ordering = position_sort_fn(&a.2, &b.2);
    if ordering == Ordering::Equal {
        return position_sort_fn(&a.0, &b.0);
    }

    return ordering;
}

fn attack_sort_fn(a: &Occupant, b: &Occupant) -> Ordering {
    if a.hp > b.hp { return Ordering::Greater }
    if a.hp < b.hp { return Ordering::Less }

    return position_sort_fn(&a.position, &b.position);
}

fn find_elves_and_goblins(arena: &HashMap<Position, Occupant>) -> Vec<(Position, Occupant)> {
    let mut occupants: Vec<(Position, Occupant)> = Vec::new();
    for (key, value) in arena {
        match value.occ_type {
            OccupantType::Elf(_) | OccupantType::Goblin(_) => occupants.push((key.clone(), value.clone())),
            OccupantType::OpenField(_) | OccupantType::Wall(_) => { /* no-op */ },
        }
    }
    occupants.sort_by(|a, b| position_sort_fn(&a.0, &b.0));
    return occupants;
}

fn build_arena() -> HashMap<Position, Occupant> {
    let mut arena: HashMap<Position, Occupant> = HashMap::new();
    let mut i: usize = 0;
    let mut id: usize = 0;

    for line in BufReader::new(File::open("src/data/day_15_input.txt").unwrap()).lines() {
        let row = line.unwrap();
        for j in 0..row.len() {
            let pos = Position{i: i as i32, j: j as i32};
            let c =  row.chars().nth(j).unwrap();

            match c {
                '#' => {
                    arena.insert(pos, Occupant{id: 0, position: pos, occ_type: OccupantType::Wall('#'), attack_power: 0, hp: 0});
                },
                '.' => {
                    arena.insert(pos, Occupant{id: 0, position: pos, occ_type: OccupantType::OpenField('.'), attack_power: 0, hp: 0});
                },
                'E' => {
                    arena.insert(pos, Occupant{id, position: pos, occ_type: OccupantType::Elf('E'), attack_power: 3, hp: 200});
                    id += 1;
                },
                'G' => {
                    arena.insert(pos, Occupant{id, position: pos, occ_type: OccupantType::Goblin('G'), attack_power: 3, hp: 200});
                    id += 1;
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

fn combat_terminated(arena: &HashMap<Position, Occupant>) -> bool {
    let elves: Vec<Occupant> = arena.values().into_iter().filter(|o| o.occ_type.unwrap_occupant() == 'E').copied().collect();
    let goblins: Vec<Occupant> = arena.values().into_iter().filter(|o| o.occ_type.unwrap_occupant() == 'G').copied().collect();
    return elves.len() == 0 || goblins.len() == 0;
}

fn enhance_elf_attack_power(attack_power: i32, mut arena: HashMap<Position, Occupant>) -> HashMap<Position, Occupant> {
    arena.values_mut().for_each(|o| { if o.occ_type.unwrap_occupant() == 'E' { o.attack_power = attack_power; }});
    return arena;
}

// TODO - Profile and optimise
fn find_outcome_of_battle(mut arena: HashMap<Position, Occupant>, part_2: bool) -> Option<i32> {
    let mut rounds = 0;
    let mut killed: HashSet<usize> = HashSet::new();

    loop {
        let mut combatants = find_elves_and_goblins(&arena);
        let mut terminate_count = 0;

        for (_, combatant) in combatants.iter_mut() {
            if !killed.contains(&combatant.id) {
                let (combat_ended, defeated_combatant) = combatant.move_or_attack(arena.borrow_mut());

                if defeated_combatant.is_some() {
                    let defeated = defeated_combatant.unwrap();
                    if part_2 {
                        if defeated.occ_type.unwrap_occupant() == 'E' {
                            return None;
                        }
                    }

                    killed.insert(defeated.id);
                }

                if combat_ended {
                    terminate_count += 1;
                }
            }
        }

        rounds += 1;

        if combat_terminated(&arena) {
            if terminate_count != 0 {
                rounds -= 1;
            }
            break;
        }
    }

    let remaining_hp: i32 = arena.values()
        .filter(|o| o.occ_type.unwrap_occupant() == 'G' || o.occ_type.unwrap_occupant() == 'E')
        .map(|o| o.hp)
        .sum();

    return Some(remaining_hp * rounds);
}

fn find_outcome_of_battle_with_enhanced_elves(arena: HashMap<Position, Occupant>, part_2: bool) -> i32 {
    let mut attack_power = 4;
    loop {
        let arena_copy = enhance_elf_attack_power(attack_power, arena.clone());
        let outcome = find_outcome_of_battle(arena_copy, part_2);
        if outcome.is_some() {
            return outcome.unwrap();
        }

        attack_power += 1;
    }
}

fn main() -> (){
    println!("Part 1: {}", find_outcome_of_battle(build_arena(), false).unwrap());
    println!("Part 2: {}", find_outcome_of_battle_with_enhanced_elves(build_arena(), true));
}