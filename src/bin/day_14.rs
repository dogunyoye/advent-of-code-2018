//! `cargo run --bin day_14`

use std::fs::{self};

#[derive(Debug)]
struct Elf {
    score: u32,
    index: u32
}

impl Elf {
    fn set(&mut self, score:u32, index:u32) -> () {
        self.score = score;
        self.index = index;
    }
}

fn part1() -> () {
    let mut elf1 = Elf { score: 3, index: 0};
    let mut elf2 = Elf { score: 7, index: 1};

    let mut recipes: Vec<u32> = Vec::new();
    recipes.push(elf1.score);
    recipes.push(elf2.score);

    let input = fs::read_to_string("src/data/day_14_input.txt").unwrap();
    let count = input.parse::<usize>().unwrap();

    while recipes.len() < count + 10 {
        let sum = elf1.score + elf2.score;

        let sum_string = sum.to_string();
        for c in sum_string.chars() {
            let digit = c.to_digit(10).unwrap();
            recipes.push(digit);
        }

        let elf1_index = (elf1.index + elf1.score + 1) % recipes.len() as u32;
        elf1.set(*recipes.get(elf1_index as usize).unwrap(), elf1_index);

        let elf2_index = (elf2.index + elf2.score + 1) % recipes.len() as u32;
        elf2.set(*recipes.get(elf2_index as usize).unwrap(), elf2_index);
    }

    print!("Part 1: ");
    for i in count..count+10 {
        print!("{}", recipes.get(i).unwrap());
    }
    println!();
}

fn part2() -> () {
    let mut elf1 = Elf { score: 3, index: 0};
    let mut elf2 = Elf { score: 7, index: 1};

    let mut recipes: Vec<u32> = Vec::new();
    recipes.push(elf1.score);
    recipes.push(elf2.score);

    let sequence = fs::read_to_string("src/data/day_14_input.txt").unwrap();
    let sequence_vec = &[5, 4, 0, 3, 9, 1];
    let mut searching = true;

    while searching {
        let sum = elf1.score + elf2.score;

        let sum_string = sum.to_string();
        for c in sum_string.chars() {
            let digit = c.to_digit(10).unwrap();
            recipes.push(digit);
        }

        if recipes.len() > sequence.len() {

            if sum_string.len() == 2 {

                let start = recipes.len() - sequence.len() - 1;
                let end = recipes.len()-1;

                if &recipes[start..end] == sequence_vec {
                    println!("Part 2: {}", recipes.len() - sequence.to_string().len() - 1);
                    break;
                }

                if !searching {
                    break;
                }
            }

            let start = recipes.len() - sequence.len();
            let end = recipes.len();

            if &recipes[start..end] == sequence_vec {
                searching = false;
                println!("Part 2: {}", recipes.len() - sequence.to_string().len());
            }
        }

        let elf1_index = (elf1.index + elf1.score + 1) % recipes.len() as u32;
        elf1.set(*recipes.get(elf1_index as usize).unwrap(), elf1_index);

        let elf2_index = (elf2.index + elf2.score + 1) % recipes.len() as u32;
        elf2.set(*recipes.get(elf2_index as usize).unwrap(), elf2_index);
    }
}

fn main() {
    part1();
    part2();
}