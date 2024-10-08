//! `cargo run --bin day_21`

mod day_16;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Instruction {
    operation: String,
    a: usize,
    b: usize,
    c: usize
}

fn part1(operations: &HashMap<String, fn(Vec<usize>, usize, usize, usize) -> Vec<usize>>, program: &Vec<Instruction>, index: usize) {
    let mut registers: Vec<usize> = vec![0, 0, 0, 0, 0, 0];
    let mut instruction_pointer: usize = 0;
    let instruction_pointer_index: usize = index;

    loop {

        if instruction_pointer > program.len() - 1 {
            break;
        }

        registers[instruction_pointer_index] = instruction_pointer;

        // Inspecting my instructions input showed line 30 to be instruction
        // that would trigger the program halt: "eqrr 1 0 4", so the "hacky"
        // solution would be to set register 1's value to register 0 - so when
        // the program reaches it, it will terminate.
        // Perhaps the more general solution would be to search the program input for all
        // register instructions involving register 0 and pre-emptively invoke them
        registers[0] = registers[1];
        let instruction: &Instruction = &program[instruction_pointer];

        let operation_fn: fn(Vec<usize>, usize, usize, usize) -> Vec<usize> = *operations.get(&instruction.operation).unwrap();
        registers = operation_fn(registers, instruction.a, instruction.b, instruction.c);

        instruction_pointer = registers[instruction_pointer_index];
        instruction_pointer += 1;
    }

    println!("Part 1: {}", registers[0]);
}

// This takes a long time to return! Very likely can be optimised to not
// iterate until we find a repeating number. Perhaps if we inspect the
// program, we can find the general pattern and generate the values until
// find a repeater
fn part2(operations: &HashMap<String, fn(Vec<usize>, usize, usize, usize) -> Vec<usize>>, program: &Vec<Instruction>, index: usize) {
    let mut registers: Vec<usize> = vec![0, 0, 0, 0, 0, 0];
    let mut instruction_pointer: usize = 0;
    let instruction_pointer_index: usize = index;
    let mut last: usize = 0;

    let mut values: HashSet<usize> = HashSet::new();

    loop {

        if instruction_pointer > program.len() - 1 {
            break;
        }

        registers[instruction_pointer_index] = instruction_pointer;
        let instruction: &Instruction = &program[instruction_pointer];

        let operation_fn: fn(Vec<usize>, usize, usize, usize) -> Vec<usize> = *operations.get(&instruction.operation).unwrap();
        registers = operation_fn(registers, instruction.a, instruction.b, instruction.c);

        instruction_pointer = registers[instruction_pointer_index];
        instruction_pointer += 1;

        if instruction_pointer == 28 {
            if values.get(&registers[1]).is_some() {
                break;
            }

            values.insert(registers[1]);
            last = registers[1];
        }
    }

    println!("Part 2: {}", last);
}

fn main() -> (){

    let mut lines: Vec<String> = BufReader::new(File::open("src/data/day_21_input.txt").unwrap())
        .lines().map(|l| l.unwrap()).collect();

    let instruction_pointer_index: usize = lines[0][4..5].parse::<usize>().unwrap();
    lines.remove(0);

    let mut program: Vec<Instruction> = Vec::new();

    for line in lines {
        let vec: Vec<String> = line.split_whitespace().map(|x| x.to_string()).collect();
        program.push(Instruction {
            operation: vec[0].to_string(),
            a: vec[1].parse::<usize>().unwrap(),
            b: vec[2].parse::<usize>().unwrap(),
            c: vec[3].parse::<usize>().unwrap()});
    }

    let mut operations: HashMap<String, fn(Vec<usize>, usize, usize, usize) -> Vec<usize>> =
        HashMap::new();

    operations.insert("addr".to_string(), day_16::addr);
    operations.insert("addi".to_string(), day_16::addi);
    operations.insert("mulr".to_string(), day_16::mulr);
    operations.insert("muli".to_string(), day_16::muli);
    operations.insert("banr".to_string(), day_16::banr);
    operations.insert("bani".to_string(), day_16::bani);
    operations.insert("borr".to_string(), day_16::borr);
    operations.insert("bori".to_string(), day_16::bori);
    operations.insert("setr".to_string(), day_16::setr);
    operations.insert("seti".to_string(), day_16::seti);
    operations.insert("gtir".to_string(), day_16::gtir);
    operations.insert("gtri".to_string(), day_16::gtri);
    operations.insert("gtrr".to_string(), day_16::gtrr);
    operations.insert("eqir".to_string(), day_16::eqir);
    operations.insert("eqri".to_string(), day_16::eqri);
    operations.insert("eqrr".to_string(), day_16::eqrr);

    part1(&operations, &program, instruction_pointer_index);
    part2(&operations, &program, instruction_pointer_index);
}