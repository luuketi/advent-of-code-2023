#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use crate::inputs;
use inputs::day3::{INPUT1, INPUT2};

const NUMBERS : &str = "0123456789";

struct Engine {
    input: String,
    positions_and_symbols: HashMap<(usize, usize), char>,
    positions_and_numbers: HashMap<(usize, usize), String>,
    sum: u32,
    number: String,
    char_positions : Vec<(usize, usize)>,
    multipliers : HashMap<(usize, usize), u32>,
}

impl Engine {
    pub fn new(input: &str) -> Self {
        Engine{
            input: input.into(),
            positions_and_symbols: Default::default(),
            positions_and_numbers: Default::default(),
            sum: 0,
            number: String::new(),
            char_positions: vec![],
            multipliers: HashMap::new(),
        }
    }

    pub fn sum_part_numbers(&mut self) -> u32 {
        self.build_symbols_map();
        self.sum = 0;
        for (row, line) in self.input.clone().split("\n").enumerate() {
            self.clear_number();
            for (col, char) in line.chars().enumerate() {
                if self.is_end_of_number(char) {
                    self.sum_number();
                } else if Self::is_end_of_line(line, col) {
                    self.append_character(row, col, char);
                    self.sum_number();
                } else {
                    if is_a_number(char) {
                        self.append_character(row, col, char);
                    }
                }
            }
        }
        self.sum
    }

    fn sum_gear_ratios(&mut self) -> u32 {
        self.build_symbols_map();
        self.build_numbers_map();
        self.clear_number();
        self.sum = 0;

        for (position, symbol) in self.positions_and_symbols.clone().iter() {
            if symbol != &'*' {
                continue;
            }
            self.multipliers = HashMap::new();
            self.add_multipliers(position);
            self.filter_multipler_if_its_row_is_duplicated();
            if self.multipliers.len() == 2 {
                if self.are_multipliers_the_same_number(position) {
                    continue;
                }
                self.sum_product();
            }
        }
        self.sum
    }

    fn are_multipliers_the_same_number(&mut self, position: &(usize, usize)) -> bool {
        let column_of_asterisk = position.1;
        let are_multipliers_in_the_same_row = HashSet::<usize>::from_iter(self.multipliers.iter().map(|((r, _), _)| *r)).len() == 1;
        let mut columns_of_multipliers = self.multipliers.iter().map(|((_, c), _)| *c);
        let are_multipliers_in_the_same_column_than_asterisk = columns_of_multipliers.next().unwrap() == column_of_asterisk || columns_of_multipliers.next().unwrap() == column_of_asterisk;
        return are_multipliers_in_the_same_row && are_multipliers_in_the_same_column_than_asterisk
    }

    fn sum_product(&mut self) {
        let product: u32 = self.multipliers.iter().map(|(_, number)| number).product();
        self.sum += product;
    }

    fn filter_multipler_if_its_row_is_duplicated(&mut self) {
        if self.multipliers.len() > 2 {
            let mut set = HashSet::new();
            for ((row, column), _) in &self.multipliers {
                if set.iter().all(|&(r, _)| r != row) {
                    set.insert((row, column));
                }
            }
            self.multipliers = set.iter().map(|(r, c)|
                ((**r, **c), self.positions_and_numbers.get(&(**r, **c)).unwrap().parse::<u32>().unwrap())
            ).collect();
        }
    }

    fn add_multipliers(&mut self, position: &(usize, usize)) {
        for position in Self::generate_all_possible_positions(*position) {
            match self.positions_and_numbers.get(&position) {
                Some(number) => { self.multipliers.insert(position, number.parse::<u32>().unwrap()); },
                _ => {}
            }
        }
    }

    fn append_character(&mut self, row: usize, col: usize, char: char) {
        self.char_positions.push((row + 1, col + 1));
        self.number = format!("{}{char}", self.number);
    }

    fn sum_number(&mut self) {
        self.sum += self.number_to_be_sum();
        self.clear_number();
    }

    fn insert_number_into_positions_and_numbers(&mut self) {
        for (row, col) in self.char_positions.clone() {
            self.positions_and_numbers.insert((row, col), self.number.clone());
        }
        self.clear_number();
    }

    fn clear_number(&mut self) {
        self.number = String::new();
        self.char_positions = vec!();
    }

    fn build_symbols_map(&mut self) {
        self.positions_and_symbols = HashMap::new();
        for (row, line) in self.input.split("\n").enumerate() {
            for (col, char) in line.chars().enumerate() {
                let a = char.clone();
                if is_a_symbol(a) {
                    self.positions_and_symbols.insert((row+1, col+1), char);
                }
            }
        }
    }

    fn build_numbers_map(&mut self) {
        for (row, line) in self.input.clone().split("\n").enumerate() {
            self.clear_number();
            for (col, char) in line.chars().enumerate() {
                if self.is_end_of_number(char) {
                    self.insert_number_into_positions_and_numbers();
                } else if Self::is_end_of_line(line, col) {
                    self.append_character(row, col, char);
                    self.insert_number_into_positions_and_numbers();
                } else {
                    if is_a_number(char) {
                        self.append_character(row, col, char);
                    }
                }
            }
        }
    }

    fn is_end_of_number(&self, char: char) -> bool {
        (char == '.' && self.number != "") || is_a_symbol(char)
    }

    fn is_end_of_line(line: &str, column: usize) -> bool {
        line.len() == column + 1
    }

    fn number_to_be_sum(&self) -> u32 {
        let positions: Vec<(usize, usize)> = self.char_positions
            .iter()
            .flat_map(|p| Self::generate_all_possible_positions(*p))
            .collect();
        let is_number_next_to_symbol = positions
            .iter()
            .any(|p| self.positions_and_symbols.contains_key(p));
        if is_number_next_to_symbol {
            return self.number.parse::<u32>().unwrap();
        }
        0
    }

    fn generate_all_possible_positions((row, col): (usize, usize)) -> Vec<(usize, usize)> {
        vec![(row-1, col-1), (row-1, col), (row-1, col+1), (row  , col-1), (row  , col+1), (row+1, col-1), (row+1, col), (row+1, col+1)]
    }
}

fn is_a_number(char: char) -> bool {
    NUMBERS.find(char) != None
}

fn is_a_symbol(c: char) -> bool {
    NUMBERS.find(c) == None && c != '.'
}

pub fn day3() {
    let result1 = Engine::new(INPUT1).sum_part_numbers();
    println!("Result 1: {result1}");
    let result2 = Engine::new(INPUT2).sum_part_numbers();
    println!("Result 2: {result2}");
    let result3 = Engine::new(INPUT1).sum_gear_ratios();
    println!("Result 3: {result3}");
    let result4 = Engine::new(INPUT2).sum_gear_ratios();
    println!("Result 4: {result4}");
}


#[test]
fn test_day1() {
    assert_eq!(4361, Engine::new(INPUT1).sum_part_numbers());
    assert_eq!(467835, Engine::new(INPUT1).sum_gear_ratios());
}