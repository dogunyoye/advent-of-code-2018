//! `cargo run --bin day_19`

mod day_16;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn run_process(instructions: &HashMap<String, fn(Vec<usize>, usize, usize, usize) -> Vec<usize>>, register: &mut Vec<usize>, mut program: Vec<String>, part_two: bool) {
    let mut instruction_pointer: usize = 0;
    let instruction_pointer_index: usize = program[0][4..5].parse::<usize>().unwrap();
    let mut cycles = 0;

    program.remove(0);

    loop {

        if instruction_pointer > program.len() - 1 {
            break;
        }

        register[instruction_pointer_index] = instruction_pointer;

        let operation: Vec<String> = program[instruction_pointer].split_whitespace()
            .map(|x| x.to_string()).collect();

        let operation_fn: fn(Vec<usize>, usize, usize, usize) -> Vec<usize> = *instructions.get(&operation[0].to_string()).unwrap();
        *register = operation_fn(register.to_vec(), operation[1].parse::<usize>().unwrap(), operation[2].parse::<usize>().unwrap(), operation[3].parse::<usize>().unwrap()).to_vec();
        instruction_pointer = register[instruction_pointer_index] + 1;
        cycles += 1;

        if part_two && cycles == 1000 {
            return;
        }
    }
}

fn main() -> (){

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

    let program: Vec<String> = BufReader::new(File::open("src/data/day_19_input.txt").unwrap())
        .lines().map(|l| l.unwrap()).collect();

    let mut registers_1: Vec<usize> = vec![0, 0, 0, 0, 0, 0];
    run_process(&instructions, &mut registers_1, program.clone(), false);
    let part_one = *registers_1.get(0).unwrap();

    println!("Part one: {}", part_one);

    // https://www.michaelfogleman.com/aoc18/#19
    let mut registers_2: Vec<usize> = vec![1, 0, 0, 0, 0, 0];
    run_process(&instructions, &mut registers_2, program.clone(), true);
    let mut part_two = 0;
    let n = registers_2.into_iter().max().unwrap();

    for i in 1..n + 1 {
        if n % i == 0 {
            part_two += i;
        }
    }

    println!("Part two: {}", part_two);
}