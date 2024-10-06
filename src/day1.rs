use regex::Regex;
use crate::inputs;
use inputs::day1::{INPUT1, INPUT2, INPUT3};

const NUMBERS_AS_LETTERS: [&'static str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
const NUMBERS: [&'static str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

fn sum_of_all_the_calibration_values(input : String) -> i32 {
    let mut result = 0;
    let re = Regex::new(r"[a-z]").unwrap();
    let lines = input.split("\n");
    for line in lines {
        let only_numbers_line = re.replace_all(line, "");
        let first_digit = only_numbers_line.chars().next().unwrap();
        let last_digit = only_numbers_line.chars().last().unwrap();
        let digits = format!("{first_digit}{last_digit}");
        let number = digits.parse::<i32>().unwrap();
        result += number;
    }
    result
}

fn find_first_number(line : String) -> i32 {
    let mut first_index = usize::MAX;
    let mut first_number : usize = 0;
    for (i, number) in NUMBERS_AS_LETTERS.iter().enumerate() {
        match line.find(number) {
            None => {},
            Some(idx) => if idx < first_index {
                first_index = idx;
                first_number = i;
            },
        }
    }
    for (i, number) in NUMBERS.iter().enumerate() {
        match line.find(number) {
            None => {},
            Some(idx) => if idx < first_index {
                first_index = idx;
                first_number = i;
            },
        }
    }
    (first_number+1) as i32
}

fn find_last_number(line : String) -> i32 {
    let mut first_index = 0;
    let mut first_number : usize = 0;
    for (i, number) in NUMBERS_AS_LETTERS.iter().enumerate() {
        match line.rfind(number) {
            None => {},
            Some(idx) => if idx >= first_index {
                first_index = idx;
                first_number = i;
            },
        }
    }
    for (i, number) in NUMBERS.iter().enumerate() {
        match line.rfind(number) {
            None => {},
            Some(idx) => if idx >= first_index {
                first_index = idx;
                first_number = i;
            },
        }
    }
    (first_number+1) as i32
}


fn sum_of_all_the_calibration_values_for_letters(input : String) -> i32 {
    let mut result = 0;
    let lines = input.split("\n");
    for line in lines {
        let first_digit = find_first_number(line.into());
        let last_digit = find_last_number(line.into());
        let digits = format!("{first_digit}{last_digit}");
        let number = digits.parse::<i32>().unwrap();
        result += number;
    }
    result
}

pub fn day1() {
    let result1 = sum_of_all_the_calibration_values(INPUT1.into());
    println!("Result 1a is {result1}");
    let result2 = sum_of_all_the_calibration_values(INPUT2.into());
    println!("Result 1b is {result2}");
    let result3 = sum_of_all_the_calibration_values_for_letters(INPUT3.into());
    println!("Result 2a is {result3}");
    let result4 = sum_of_all_the_calibration_values_for_letters(INPUT2.into());
    println!("Result 2b is {result4}");
}


#[test]
fn test_day1() {
    assert_eq!(142, sum_of_all_the_calibration_values(INPUT1.into()));
    assert_eq!(281, sum_of_all_the_calibration_values_for_letters(INPUT3.into()));
}