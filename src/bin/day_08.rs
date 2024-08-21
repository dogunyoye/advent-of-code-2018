//! `cargo run --bin day_08`

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::slice::Iter;

#[derive(Clone, Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
    value: usize
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

    Node { children: children, metadata: metadata, value: 0 }
}

fn calculate_value(node: &mut Node, sum: &mut usize) -> () {
    if node.children.is_empty() {
        node.value = node.metadata.iter().sum();
        return;
    }

    for i in node.metadata.clone() {
        if i == 0 {
            continue;
        }

        if let Some(child) = node.children.get_mut(i-1) {
            calculate_value(child, sum);
            *sum += child.value;
        }
    }
}

fn main() -> (){

    let mut tree_data: Vec<usize> = Vec::new();

    for line in BufReader::new(File::open("src/data/day_8_input.txt").unwrap()).lines() {
        let line = line.unwrap();
        let data = line.split_whitespace().collect::<Vec<&str>>();
        for x in data {
            tree_data.push(x.parse::<usize>().unwrap());
        }
    }

    let mut iterator = tree_data.iter();
    let mut sum: usize = 0;

    let mut root = build_tree(&tree_data, &mut iterator, &mut sum);

    println!("Part 1: {}", sum);

    let mut part2_sum: usize = 0;
    calculate_value(&mut root, &mut part2_sum);

    println!("Part 2: {}", part2_sum);
}
