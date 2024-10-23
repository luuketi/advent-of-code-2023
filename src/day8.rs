#![allow(dead_code)]

use std::collections::HashMap;
use regex::Regex;
use crate::inputs::day8::{INPUT1, INPUT2, INPUT3};


struct Map {
    nodes : HashMap<String, (String, String)>,
    position : String,
}

impl Map {
    pub fn new(input : &str) -> Self {
        let nodes = Regex::new(r"(?m)^(.*?) = \((.*), (.*)\)$")
            .unwrap()
            .captures_iter(input)
            .map(|c| c.extract())
            .map(|(_, [node, left, right])| (node.into(), (left.into(), right.into())))
            .collect::<HashMap<String, (String, String)>>();
        Map{nodes, position: "AAA".into()}
    }

    pub fn move_to(&mut self, direction : char) -> String {
        let new_position : String;
        if direction == 'L' {
            new_position = self.move_to_left();
        } else {
            new_position = self.move_to_right();
        }
        self.position = new_position.clone();
        new_position
    }

    fn move_to_right(&self) -> String {
        self.nodes.get(&self.position).unwrap().1.clone()
    }

    fn move_to_left(&self) -> String {
        self.nodes.get(&self.position).unwrap().0.clone()
    }
}

fn count_steps(input :&str) -> u32 {
    let mut splitted_input = input.split("\n\n");
    let instructions = splitted_input.next().unwrap();
    let mut map = Map::new(splitted_input.next().unwrap());
    let mut node = "AAA".into();
    let mut steps = 0;

    while node != "ZZZ" {
        for i in instructions.chars() {
            node = map.move_to(i);
            steps += 1;
            if node == "ZZZ" {
                return steps;
            }
        }
    }
    0
}

pub fn day8() {
    let result1 = count_steps(INPUT1);
    println!("Result 1: {result1}");
    let result2 = count_steps(INPUT2);
    println!("Result 2: {result2}");
    let result3 = count_steps(INPUT3);
    println!("Result 3: {result3}");
}

#[test]
fn test_day8() {
    assert_eq!(2, count_steps(INPUT1));
    assert_eq!(6, count_steps(INPUT2));
}

