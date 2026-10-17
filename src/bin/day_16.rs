//! `cargo run --bin day_16`

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::collections::HashSet;

pub(crate) type Operation = fn(Vec<usize>, usize, usize, usize) -> Vec<usize>;

pub(crate) fn addr(mut register: Vec<usize>, a: usize, b: usize, c: usize) -> Vec<usize> {
    register[c] = register[a]  + register[b];
    register
}

pub(crate) fn addi(mut register: Vec<usize>, a: usize, b: usize, c: usize) -> Vec<usize> {
    register[c] = register[a] + b;
    register
}

pub(crate) fn mulr(mut register: Vec<usize>, a: usize, b: usize, c: usize) -> Vec<usize> {
    register[c] = register[a] * register[b];
    register
}

pub(crate) fn muli(mut register: Vec<usize>, a: usize, b: usize, c: usize) -> Vec<usize> {
    register[c] = register[a] * b;
    register
}

pub(crate) fn banr(mut register: Vec<usize>, a: usize, b: usize, c: usize) -> Vec<usize> {
    register[c] = register[a] & register[b];
    register
}

pub(crate) fn bani(mut register: Vec<usize>, a: usize, b: usize, c: usize) -> Vec<usize> {
    register[c] = register[a] & b;
    register
}

pub(crate) fn borr(mut register: Vec<usize>, a: usize, b: usize, c: usize) -> Vec<usize> {
    register[c] = register[a] | register[b];
    register
}

pub(crate) fn bori(mut register: Vec<usize>, a: usize, b: usize, c: usize) -> Vec<usize> {
    register[c] = register[a] | b;
    register
}

pub(crate) fn setr(mut register: Vec<usize>, a: usize, _: usize, c: usize) -> Vec<usize> {
    register[c] = register[a];
    register
}

pub(crate) fn seti(mut register: Vec<usize>, a: usize, _: usize, c: usize) -> Vec<usize> {
    register[c] = a;
    register
}

pub(crate) fn gtir(mut register: Vec<usize>, a: usize, b: usize, c: usize) -> Vec<usize> {
    if a > register[b] {
        register[c] = 1;
        return register;
    }

    register[c] = 0;
    register
}

pub(crate) fn gtri(mut register: Vec<usize>, a: usize, b: usize, c: usize) -> Vec<usize> {
    if register[a] > b {
        register[c] = 1;
        return register;
    }

    register[c] = 0;
    register
}

pub(crate) fn gtrr(mut register: Vec<usize>, a: usize, b: usize, c: usize) -> Vec<usize> {
    if register[a] > register[b] {
        register[c] = 1;
        return register;
    }

    register[c] = 0;
    register
}

pub(crate) fn eqir(mut register: Vec<usize>, a: usize, b: usize, c: usize) -> Vec<usize> {
    if a == register[b] {
        register[c] = 1;
        return register;
    }

    register[c] = 0;
    register
}

pub(crate) fn eqri(mut register: Vec<usize>, a: usize, b: usize, c: usize) -> Vec<usize> {
    if register[a] == b {
        register[c] = 1;
        return register;
    }

    register[c] = 0;
    register
}

pub(crate) fn eqrr(mut register: Vec<usize>, a: usize, b: usize, c: usize) -> Vec<usize> {
    if register[a] == register[b] {
        register[c] = 1;
        return register;
    }

    register[c] = 0;
    register
}

#[allow(dead_code)]
fn main(){

    let mut count = 0;
    let lines: Vec<String> = BufReader::new(File::open("src/data/day_16_input.txt").unwrap())
                             .lines().map(|l| l.unwrap()).collect();

    let functions: Vec<Operation> = vec![
        addr,  // 0
        addi,  // 1
        mulr,  // 2
        muli,  // 3
        banr,  // 4
        bani,  // 5
        borr,  // 6
        bori,  // 7
        setr,  // 8
        seti,  // 9
        gtir,  // 10
        gtri,  // 11
        gtrr,  // 12
        eqir,  // 13
        eqri,  // 14
        eqrr,  // 15
    ];

    let mut possible_opcodes: HashMap<usize, HashSet<usize>> = HashMap::new();
    for i in 0..functions.len() {
        possible_opcodes.insert(i, HashSet::new());
    }
    let mut opcodes: HashMap<usize, usize> = HashMap::new();

    let mut blank_line_count = 0;
    let mut program_start_index = 0;

    for (i, line) in lines.iter().enumerate() {

        if line.is_empty() {
            blank_line_count += 1;
            // if we see 3 blank lines in a row
            // break as we're entering part 2
            if blank_line_count == 3 {
                // keep index as we'll need it for
                // later
                program_start_index = i + 1;
                break;
            }

            continue;
        }

        if i % 4 == 0 {
            let line1 = &line.replace("Before: [", "").replace("]", "").replace(" ", "");
            let registers_before: Vec<usize> = line1.split(",").map(|v| v.parse::<usize>().unwrap()).collect();

            let operation: Vec<usize> = lines[i+1].split_whitespace().map(|v| v.parse::<usize>().unwrap()).collect();

            let line3 = &lines[i+2].replace("After:  [", "").replace("]", "").replace(" ", "");
            let registers_after: Vec<usize> = line3.split(",").map(|v| v.parse::<usize>().unwrap()).collect();

            let mut total = 0;
            let mut vec: Vec<usize> = Vec::new();

            for (j, function) in functions.iter().enumerate() {
                let eval = function(registers_before.clone(), operation[1], operation[2], operation[3]);

                if eval == registers_after {
                    if let Some(set) = possible_opcodes.get_mut(&operation[0]) {
                        set.insert(j);
                    }
                    total += 1;
                    vec.push(j);
                }
            }

            if total == 1 {
                opcodes.insert(operation[0], vec[0]);
            }

            if total >= 3 {
                count += 1;
            }

            blank_line_count = 0;
        }
    }

    println!("Part 1: {}", count);

    let mut determined_opcodes: HashSet<usize> = HashSet::new();

    for v in possible_opcodes.values() {
        if v.len() == 1 {
            for x in v.iter() {
                determined_opcodes.insert(*x);
            }
        }
    }

    let mut all_sets_empty = false;

    while !all_sets_empty {
        for (k, v) in &mut possible_opcodes {
            for code in &determined_opcodes {
                let _ = &v.remove(code);
            }

            if v.len() == 1 {
                for x in v.iter() {
                    determined_opcodes.insert(*x);
                    opcodes.insert(*k, *x);
                }
            }
        }

        all_sets_empty = true;
        for v in possible_opcodes.values_mut() {
            if !v.is_empty() {
                all_sets_empty = false;
            }
        }
    }

    let mut part2_register = vec![0; 4];

    for line in lines.iter().skip(program_start_index) {
        let operation: Vec<usize> = line.split_whitespace().map(|v| v.parse::<usize>().unwrap()).collect();
        let function_index: usize = *opcodes.get(&operation[0]).unwrap();
        part2_register = functions.get(function_index).unwrap()(part2_register.clone(), operation[1], operation[2], operation[3]);
    }

    println!("Part 2: {:?}", part2_register[0]);
}