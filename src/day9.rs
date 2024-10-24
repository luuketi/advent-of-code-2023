#![allow(dead_code)]

use std::collections::VecDeque;
use std::string::ParseError;
use std::str::FromStr;
use crate::inputs::day9::{INPUT1, INPUT2};


struct Steps {
    steps : Vec<i32>
}

impl FromStr for Steps {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let steps = line.split(" ").map(|v| v.parse::<i32>().unwrap()).collect();
        Ok(Steps{ steps })
    }
}

impl Steps {
    pub fn predict(&self) -> i64 {
        let mut new_steps : Vec<i32> = self.steps.clone();
        let mut last_steps : Vec<i32> = Vec::new();
        last_steps.push(*new_steps.last().unwrap());
        while !self.check_zeroes(new_steps.clone()) {
            new_steps = self.create_new_steps(new_steps);
            last_steps.push(*new_steps.last().unwrap());
        }
        self.predict_last_step(last_steps)
    }

    fn check_zeroes(&self, steps: Vec<i32>) -> bool {
        steps.iter().all(|s| *s == 0 )
    }

    fn create_new_steps(&self, steps: Vec<i32>) -> Vec<i32> {
        let mut new_steps : Vec<i32> = Vec::new();
        let mut steps = VecDeque::from(steps);
        let mut last_value = steps.pop_front().unwrap();
        while let Some(s) = steps.pop_front() {
            new_steps.push(s - last_value);
            last_value = s;
        }
        new_steps
    }

    fn predict_last_step(&self, mut last_steps: Vec<i32>) -> i64 {
        let mut last_step = last_steps.pop().unwrap() ;
        while let Some(s) = last_steps.pop() {
            last_step += s;
        }
        last_step as i64
    }

    pub fn predict_first_value(&self) -> i64 {
        let mut new_steps : Vec<i32> = self.steps.clone();
        let mut first_steps : Vec<i32> = Vec::new();
        first_steps.push(*new_steps.first().unwrap());
        while !self.check_zeroes(new_steps.clone()) {
            new_steps = self.create_new_steps(new_steps);
            first_steps.push(*new_steps.first().unwrap());
        }
        self.predict_first_step(first_steps)
    }

    fn predict_first_step(&self, mut last_steps: Vec<i32>) -> i64 {
        let mut last_step = last_steps.pop().unwrap() ;
        while let Some(s) = last_steps.pop() {
            last_step = s - last_step;
        }
        last_step as i64
    }
}


fn sum_of_predicted_values(input : &str) -> i64 {
    input.split("\n").map(|line| Steps::from_str(line).unwrap().predict()).sum()
}

fn sum_of_predicted_beginning_values(input : &str) -> i64 {
    input.split("\n").map(|line| Steps::from_str(line).unwrap().predict_first_value()).sum()
}


pub fn day9() {
    let result1 = sum_of_predicted_values(INPUT1);
    println!("Result 1: {result1}");
    let result2 = sum_of_predicted_values(INPUT2);
    println!("Result 2: {result2}");
    let result3 = sum_of_predicted_beginning_values(INPUT1);
    println!("Result 3: {result3}");
    let result4 = sum_of_predicted_beginning_values(INPUT2);
    println!("Result 4: {result4}");
}

#[test]
fn test_day9() {
    assert_eq!(114, sum_of_predicted_values(INPUT1));
    assert_eq!(2, sum_of_predicted_beginning_values(INPUT1));
}

