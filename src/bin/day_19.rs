//! `cargo run --bin day_19`

mod day_16;

use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashMap;

fn main() -> Result<()>{

    let mut instructions: HashMap<String, fn(Vec<usize>, usize, usize, usize) -> Vec<usize>> =
        HashMap::new();

    instructions.insert("addr".to_string(), day_16::addr);
    instructions.insert("addi".to_string(), day_16::addi);
    instructions.insert("mulr".to_string(), day_16::mulr);
    instructions.insert("muli".to_string(), day_16::muli);
    instructions.insert("banr".to_string(), day_16::banr);
    instructions.insert("bani".to_string(), day_16::bani);
    instructions.insert("borr".to_string(), day_16::borr);
    instructions.insert("bori".to_string(), day_16::bori);
    instructions.insert("setr".to_string(), day_16::setr);
    instructions.insert("seti".to_string(), day_16::seti);
    instructions.insert("gtir".to_string(), day_16::gtir);
    instructions.insert("gtri".to_string(), day_16::gtri);
    instructions.insert("gtrr".to_string(), day_16::gtrr);
    instructions.insert("eqir".to_string(), day_16::eqir);
    instructions.insert("eqri".to_string(), day_16::eqri);
    instructions.insert("eqrr".to_string(), day_16::eqrr);

    let mut lines: Vec<String> = BufReader::new(File::open("src/data/day_19_input.txt")?)
        .lines().map(|l| l.unwrap()).collect();

    let mut register: Vec<usize> = vec![0, 0, 0, 0, 0, 0];

    let mut instruction_pointer: usize = 0;
    let instruction_pointer_index: usize = lines[0][4..5].parse::<usize>().unwrap();

    lines.remove(0);

    loop {

        if instruction_pointer > lines.len() - 1 {
            break;
        }

        register[instruction_pointer_index] = instruction_pointer;

        let operation: Vec<String> = lines[instruction_pointer].split_whitespace()
            .map(|x| x.to_string()).collect();

        let operation_fn: fn(Vec<usize>, usize, usize, usize) -> Vec<usize> = *instructions.get(&operation[0].to_string()).unwrap();
        register = operation_fn(register, operation[1].parse::<usize>().unwrap(), operation[2].parse::<usize>().unwrap(), operation[3].parse::<usize>().unwrap());
        instruction_pointer = register[instruction_pointer_index];
        instruction_pointer += 1;
    }

    println!("Part one: {}", register[0]);
    Ok(())
}