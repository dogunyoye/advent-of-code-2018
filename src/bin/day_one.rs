//! `cargo run --bin day_one`

use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()>{
    let file = File::open("src/bin/day_one_input.txt")?;
    let mut frequency = 0;

    for line in BufReader::new(file).lines() {
        frequency = frequency + line?.parse::<i32>().unwrap();
    }
    
    println!("{}", frequency);
    Ok(())
}