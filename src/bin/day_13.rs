//! `cargo run --bin day_13`

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct Position {
    i: usize,
    j: usize
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct Cart {
    id: usize,
    pos: Position,
    facing: char,
    intersections: usize
}

impl Cart {

    fn turn_left(&mut self) {
        match self.facing {
            '>' => {
                self.facing = '^';
            },
            'v' => {
                self.facing = '>';
            },
            '<' => {
                self.facing = 'v';
            },
            '^' => {
                self.facing = '<';
            },
            _ => {
                panic!("invalid cart character: {}", self.facing);
            }
        }
    }

    fn turn_right(&mut self) {
        match self.facing {
            '>' => {
                self.facing = 'v';
            },
            'v' => {
                self.facing = '<';
            },
            '<' => {
                self.facing = '^';
            },
            '^' => {
                self.facing = '>';
            },
            _ => {
                panic!("invalid cart character: {}", self.facing);
            }
        }
    }
}

fn build_track_data() -> (HashMap<Position, char>, HashMap<Position, char>, Vec<Cart>) {
    let mut track_map: HashMap<Position, char> = HashMap::new();
    let mut initial_map: HashMap<Position, char> = HashMap::new();
    let mut carts: Vec<Cart> = Vec::new();

    let mut i = 0;
    let mut cart_id: usize = 0;
    for line in BufReader::new(File::open("src/data/day_13_input.txt").unwrap()).lines() {
        let row = line.unwrap();
        let mut j = 0;
        for b in row.bytes() {
            let c = b as char;
            let position = Position{i, j};
            match c {
                '>' => {
                    initial_map.insert(position, '-');
                    carts.push(Cart{id: cart_id, pos: position, facing: '>', intersections: 0});
                    cart_id += 1;
                },
                'v' => {
                    initial_map.insert(position, '|');
                    carts.push(Cart{id: cart_id, pos: position, facing: 'v', intersections: 0});
                    cart_id += 1;
                },
                '<' => {
                    initial_map.insert(position, '-');
                    carts.push(Cart{id: cart_id, pos: position, facing: '<', intersections: 0});
                    cart_id += 1;
                },
                '^' => {
                    initial_map.insert(position, '|');
                    carts.push(Cart{id: cart_id, pos: position, facing: '^', intersections: 0});
                    cart_id += 1;
                },
                _ => {
                    initial_map.insert(position, c);
                }
            }
            track_map.insert(position, b as char);
            j += 1;
        }
        i += 1
    }

    (track_map, initial_map, carts)
}

#[allow(unused)]
fn print_map(track_map: HashMap<Position, char>) {
    let max_depth = track_map.keys().map(|p| p.i).max().unwrap();
    let max_width = track_map.keys().map(|p| p.j).max().unwrap();

    for i in 0..max_depth + 1 {
        let mut line = "".to_owned();
        for j in 0..max_width + 1 {
            let p = Position{i, j};
            if track_map.contains_key(&p) {
                line.push(*track_map.get(&p).unwrap());
            }
        }
        println!("{}", line);
    }
}

fn cart_sort_fn(a: &Cart, b: &Cart) -> Ordering {
    if a.pos.i > b.pos.i { return Ordering::Greater; }
    if a.pos.i < b.pos.i { return Ordering::Less; }

    if a.pos.j > b.pos.j { return Ordering::Greater; }
    if a.pos.j < b.pos.j { return Ordering::Less; }

    return Ordering::Equal;
}

fn simulate_carts(track_data: (HashMap<Position, char>, HashMap<Position, char>, Vec<Cart>), is_part_two: bool) -> Position {

    let mut track_map = track_data.0;
    let initial_map = track_data.1;
    let mut carts = track_data.2;
    let mut cart_positions: HashMap<usize, Position> = HashMap::new();

    for c in &carts {
        cart_positions.insert(c.id, c.pos);
    }

    loop {
        carts.sort_by(|a: &Cart, b: &Cart| cart_sort_fn(a, b));
        let mut to_remove: HashSet<usize> = HashSet::new();

        for cart in &mut carts {

            if is_part_two && to_remove.contains(&cart.id) {
                continue;
            }

            let next;
            match cart.facing {
                '>' => {
                    next = Position{i: cart.pos.i, j: cart.pos.j + 1};
                },
                'v' => {
                    next = Position{i: cart.pos.i + 1, j: cart.pos.j};
                },
                '<' => {
                    next = Position{i: cart.pos.i, j: cart.pos.j - 1};
                },
                '^' => {
                    next = Position{i: cart.pos.i - 1, j:cart.pos.j};
                },
                _ => {
                    panic!("Invalid cart character: {}", cart.facing)
                }
            }

            if track_map.contains_key(&next) {
                let neighbour = *track_map.get(&next).unwrap();
                if neighbour != ' ' && (neighbour == '>' || neighbour == '<' || neighbour == 'v' || neighbour == '^') {
                    if !is_part_two {
                        return Position{i: next.j, j: next.i};
                    }

                    track_map.insert(next, *initial_map.get(&next).unwrap());
                    track_map.insert(cart.pos, *initial_map.get(&cart.pos).unwrap());
                    cart.pos = Position{i: next.i, j: next.j};

                    to_remove.insert(cart.id);
                    cart_positions.remove(&cart.id);
                    for (k, v) in &cart_positions {
                        if *v == cart.pos {
                            to_remove.insert(*k);
                        }
                    }

                    for r in &to_remove {
                        cart_positions.remove(&r);
                    }

                    continue;
                }

                match neighbour {
                    '\\' => {
                        match cart.facing {
                            '>' => {
                                cart.facing = 'v';
                            },
                            'v' => {
                                cart.facing = '>';
                            },
                            '<' => {
                                cart.facing = '^';
                            },
                            '^' => {
                                cart.facing = '<';
                            },
                            _ => {
                                panic!("Invalid cart character: {}", cart.facing)
                            }
                        }
                    },
                    '/' => {
                        match cart.facing {
                            '>' => {
                                cart.facing = '^';
                            },
                            'v' => {
                                cart.facing = '<';
                            },
                            '<' => {
                                cart.facing = 'v';
                            },
                            '^' => {
                                cart.facing = '>';
                            },
                            _ => {
                                panic!("Invalid cart character: {}", cart.facing)
                            }
                        }
                    },
                    '+' => {
                        cart.intersections += 1;
                        if cart.intersections % 3 == 1 {
                            cart.turn_left();
                        }
                        else if cart.intersections % 3 == 0 {
                            cart.turn_right();
                        }
                    },
                    _ => {
                        // no-op
                        // keep facing the same direction
                    }
                }

                track_map.insert(next, cart.facing);
                track_map.insert(cart.pos, *initial_map.get(&cart.pos).unwrap());
                cart.pos = Position{i: next.i, j: next.j};

                cart_positions.insert(cart.id, cart.pos.clone());
            }
        }

        if is_part_two {
            carts.retain(|&c| !to_remove.contains(&c.id));
            if carts.len() == 1 {
                let last_cart = carts.get(0).unwrap();
                return Position{i: last_cart.pos.j, j: last_cart.pos.i};
            }
        }
    }
}

fn main() -> (){
    let crash_location = simulate_carts(build_track_data(), false);
    println!("Part 1: {},{}", crash_location.i, crash_location.j);

    let last_cart_position = simulate_carts(build_track_data(), true);
    println!("Part 2: {},{}", last_cart_position.i, last_cart_position.j);
}