//! `cargo run --bin day_8`

use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::slice::Iter;

#[derive(Clone, Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>
}

fn build_tree(data: &Vec<usize>, iterator: &mut Iter<usize>, sum: &mut usize) -> Node {
    let children_count = iterator.next().unwrap();
    let metadata_count = iterator.next().unwrap();

    let mut children: Vec<Node> = Vec::new();
    let mut metadata: Vec<usize> = Vec::new();

    for _ in 0..*children_count {
        children.push(build_tree(data, iterator, sum));
    }

    for _ in 0..*metadata_count {
        let value = *iterator.next().unwrap();
        metadata.push(value);
        *sum += value;
    }

    Node { children: children, metadata: metadata }
}

fn main() -> Result<()>{

    let mut tree_data: Vec<usize> = Vec::new();

    for line in BufReader::new(File::open("src/data/day_8_input.txt")?).lines() {
        let line = line.unwrap();
        let data = line.split_whitespace().collect::<Vec<&str>>();
        for x in data {
            tree_data.push(x.parse::<usize>().unwrap());
        }
    }

    let mut iterator = tree_data.iter();
    let mut sum: usize = 0;

    build_tree(&tree_data, &mut iterator, &mut sum);

    println!("Part 1: {}", sum);

    Ok(())
}