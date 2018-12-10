//! `cargo run --bin day_4`

extern crate chrono;

use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashMap;

use chrono::{NaiveTime, NaiveDateTime};

fn main() -> Result<()>{

    let mut guard_log = Vec::new();
    let mut guards_sleep_map: HashMap<String, i64> = HashMap::new();
    let mut minutes_map: HashMap<i32, i32> = HashMap::new();

    for i in 0..59 {
        minutes_map.insert(i, 0);
    }

    for line in BufReader::new(File::open("src/data/day_4_input.txt")?).lines() {
        guard_log.push(line?.to_string());
    }

    guard_log.sort_by_key(|a|
        NaiveDateTime::parse_from_str(&a[1..17], "%Y-%m-%d %H:%M").unwrap().timestamp());

    let mut current_guard_id = String::new();
    let mut current_sleep_time = String::new();

    for log_line in guard_log.iter() {
        //println!("{}", log_line);
        let log_data_vec = log_line.splitn(3, " ").collect::<Vec<&str>>().iter().map(|&x| x.to_owned()).collect::<Vec<String>>();

        let time = log_data_vec.get(1).unwrap().to_string();
        let event = log_data_vec.get(2).unwrap().to_string();

        if event.ends_with("begins shift") {
            let guard_id = event.trim_end_matches("begins shift").trim();
            if !guards_sleep_map.contains_key(guard_id) {
                // new guard
                guards_sleep_map.insert(guard_id.to_string(), 0);
            }

            current_guard_id = guard_id.to_string();
        }
        else if log_line.ends_with("falls asleep") {
            let sleep_time = time.trim_end_matches("]").trim();
            current_sleep_time = sleep_time.to_string();
        }
        else if log_line.ends_with("wakes up") {
            let wake_up_time = time.trim_end_matches("]").trim();

            let total_sleep = NaiveTime::parse_from_str(&wake_up_time, "%H:%M").unwrap() - NaiveTime::parse_from_str(&current_sleep_time, "%H:%M").unwrap();

            let cloned_map = guards_sleep_map.clone();
            let total_guard_sleep_time = cloned_map.get(&current_guard_id);
            guards_sleep_map.insert(current_guard_id.to_string(), total_guard_sleep_time.unwrap() + total_sleep.num_minutes());
        }
    }

    let mut sleepiest_guard = "";
    let mut max_sleep: i64 = 0;

    for (key, val) in guards_sleep_map.iter() {
        if val > &max_sleep {
            sleepiest_guard = key;
            max_sleep = *val;
        }
    }

    for log_line in guard_log.iter() {
        let log_data_vec = log_line.splitn(3, " ").collect::<Vec<&str>>().iter().map(|&x| x.to_owned()).collect::<Vec<String>>();

        let time = log_data_vec.get(1).unwrap().to_string();
        let event = log_data_vec.get(2).unwrap().to_string();

        if event.ends_with("begins shift") {
            let guard_id = event.trim_end_matches("begins shift").trim();
            current_guard_id = guard_id.to_string();
        }
        else if log_line.ends_with("falls asleep") && current_guard_id == sleepiest_guard {
            let sleep_time = time.trim_end_matches("]").trim();
            current_sleep_time = sleep_time.to_string();
        }
        else if log_line.ends_with("wakes up") && current_guard_id == sleepiest_guard {

            let wake_up_time = time.trim_end_matches("]").trim();
            let guard_sleep = &current_sleep_time[3..5].parse::<i32>().unwrap();
            let guard_wakeup = &wake_up_time[3..5].parse::<i32>().unwrap();

            for i in *guard_sleep..*guard_wakeup {
                let cloned_map = minutes_map.clone();
                let count = cloned_map.get(&i);
                let incremented = count.unwrap() + 1;
                minutes_map.insert(i, incremented);
            }
        }
    }

    let mut most_frequent = 0;
    let mut minute_val = 0;

    for (key, val) in minutes_map.iter() {
        if val > &most_frequent {
            minute_val = *key;
            most_frequent = *val;
        }
    }

    println!("Part 1 - Most frequent count: {} times, on minute {} from {}", most_frequent, minute_val, sleepiest_guard);

    for val in minutes_map.values_mut() {
        *val = 0;
    }

    let mut guard = String::new();
    let mut max = 0;
    let mut minute = 0;

    for (key, _val) in guards_sleep_map.iter() {

        for log_line in guard_log.iter() {
            let log_data_vec = log_line.splitn(3, " ").collect::<Vec<&str>>().iter().map(|&x| x.to_owned()).collect::<Vec<String>>();

            let time = log_data_vec.get(1).unwrap().to_string();
            let event = log_data_vec.get(2).unwrap().to_string();

            if event.ends_with("begins shift") {
                let guard_id = event.trim_end_matches("begins shift").trim();
                current_guard_id = guard_id.to_string();
            }
            else if log_line.ends_with("falls asleep") && current_guard_id == *key {
                let sleep_time = time.trim_end_matches("]").trim();
                current_sleep_time = sleep_time.to_string();
            }
            else if log_line.ends_with("wakes up") && current_guard_id == *key {

                let wake_up_time = time.trim_end_matches("]").trim();
                let guard_sleep: &i32 = &current_sleep_time[3..5].parse::<i32>().unwrap();
                let guard_wakeup: &i32 = &wake_up_time[3..5].parse::<i32>().unwrap();

                for i in *guard_sleep..*guard_wakeup {
                    let cloned_map = minutes_map.clone();
                    let count = cloned_map.get(&i);
                    let incremented = count.unwrap() + 1;
                    minutes_map.insert(i, incremented);
                }
            }
        }

        for (k, v) in minutes_map.iter() {
            if v > &max {
                minute = *k;
                max = *v;
                guard = key.to_string();
            }
        }

        for count in minutes_map.values_mut() {
            *count = 0;
        }
    }

    println!("Part 2 - Most frequent sleeping minute: {}, from {}", minute, guard);

    Ok(())
}