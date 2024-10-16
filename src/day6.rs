#![allow(dead_code)]

use std::str::Split;
use regex::Regex;
use crate::inputs::day6::{INPUT1, INPUT2};

struct Boat {}

impl Boat {
    pub fn hold_button_for(time :u64, total_time :u64) -> u64 {
        (total_time - time) * time
    }
}

fn read_line(line: &mut Split<&str>) -> Vec<u64> {
    line.next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split_whitespace().map(|s| s.parse::<u64>().unwrap())
        .collect()
}

fn number_of_ways_to_beat_the_record(input : &str) -> u64 {
    let mut splitted_input = input.split("\n");
    let times = read_line(&mut splitted_input);
    let distances = read_line(&mut splitted_input);
    times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)|
            (0..(time+1))
                .filter(|t| Boat::hold_button_for(*t, *time) > *distance)
                .count() as u64
        ).product()
}

fn read_line_without_spaces(line: &mut Split<&str>) -> u64 {
    line.next().unwrap().split(":").last().unwrap().parse().unwrap()
}

fn number_of_ways_to_beat_the_record_without_spaces(input : &str) -> u64 {
    let input_without_spaces = Regex::new(r" +").unwrap().replace_all(input, "");
    let mut splitted_input = input_without_spaces.split("\n");
    let time = read_line_without_spaces(&mut splitted_input);
    let distance = read_line_without_spaces(&mut splitted_input);
    (0..(time+1)).filter(|t| Boat::hold_button_for(*t, time) > distance).count() as u64
}

pub fn day6() {
    let result1 = number_of_ways_to_beat_the_record(INPUT1);
    println!("Result 1: {result1}");
    let result2 = number_of_ways_to_beat_the_record(INPUT2);
    println!("Result 2: {result2}");
    let result3 = number_of_ways_to_beat_the_record_without_spaces(INPUT1);
    println!("Result 3: {result3}");
    let result4 = number_of_ways_to_beat_the_record_without_spaces(INPUT2);
    println!("Result 4: {result4}");
}

#[test]
fn test_day6() {
    assert_eq!(288, number_of_ways_to_beat_the_record(INPUT1));
    assert_eq!(71503, number_of_ways_to_beat_the_record_without_spaces(INPUT1));
}