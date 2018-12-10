//! `cargo run --bin day_9`

use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::cmp;

fn play_game(players: i32, number_of_marbles: i64) -> i64 {
    let mut game: VecDeque<i64> = VecDeque::new();
    game.push_back(0);

    let mut num_to_play: i64 = 1;
    let mut players_score: HashMap<i32, i64> = HashMap::new(); // <player_number, score>

    for i in 1..players + 1 {
        players_score.insert(i, 0);
    }

    let mut in_play = true;

    while in_play {
        for i in 1..players + 1 {
            if game.len() == 1 {
                game.push_back(num_to_play);
                num_to_play += 1;
                continue;
            }

            if num_to_play == number_of_marbles {
                in_play = false;
                break;
            }

            if num_to_play % 23 == 0 {

                for _ in 0..7 {
                    let popped = game.pop_front().unwrap();
                    game.push_back(popped);
                }

                let removed_marble = game.pop_front().unwrap();
                let popped = game.pop_back().unwrap();
                game.push_front(popped);

                if let Some(score) = players_score.get_mut(&i) {
                    *score += removed_marble + num_to_play;
                }
            }
            else {
                let popped = game.pop_back().unwrap();
                game.push_front(popped);
                game.push_front(num_to_play);
            }

            num_to_play += 1;
        }
    }

    let mut winning_score = 0;

    for (_, val) in players_score.iter() {
        winning_score = cmp::max(winning_score, *val);
    }

    winning_score
}

fn main() -> Result<()>{

    let mut players: i32 = 0;
    let mut last_marble_points: i64 = 0;

    for line in BufReader::new(File::open("src/bin/day_9_input.txt")?).lines() {
        let line = line.unwrap();
        let data = line.split(" ").collect::<Vec<&str>>();

        players = data.get(0).unwrap().parse().unwrap();
        last_marble_points = data.get(data.len() - 2).unwrap().parse().unwrap();
    }

    println!("Part 1: {}", play_game(players, last_marble_points));
    println!("Part 2: {}", play_game(players, last_marble_points * 100));

    Ok(())
}