#![allow(dead_code)]

use crate::inputs;
use inputs::day2::{INPUT1, INPUT2};
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct SetOfCubes {
    red : u32,
    green : u32,
    blue : u32,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseSetOfCubesError;

impl FromStr for SetOfCubes {
    type Err = ParseSetOfCubesError;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut new = SetOfCubes{ red: 0, green: 0, blue: 0 };
        let mut cubes = HashMap::new();
        for cube in line.split(", ") {
            let splitted_cube : Vec<&str> = cube.split(" ").collect();
            cubes.insert(splitted_cube[1], splitted_cube[0].parse::<u32>());
        }
        for (color, quantity) in cubes {
            let quantity = quantity.unwrap().clone();
            match color {
                "red" => new.red = quantity,
                "green" => new.green = quantity,
                "blue" => new.blue = quantity,
                _ => {}
            }
        }
       Ok(new)
    }
}

impl SetOfCubes {
    fn power(self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Clone, Debug)]
struct Game {
    id : u32,
    sets: Vec<SetOfCubes>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;

impl FromStr for Game {
    type Err = ParseGameError;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let splitted_line : Vec<&str> = line.split(": ").collect();
        let id = splitted_line[0]
            .split(" ")
            .collect::<Vec<&str>>()[1]
            .parse::<u32>()
            .unwrap();
        let sets = splitted_line[1]
            .split("; ")
            .flat_map(|g| SetOfCubes::from_str(g))
            .collect();
        Ok(Game{ id, sets })
    }
}

impl Game {
    fn is_possible_with_bag(self, bag : SetOfCubes) -> bool {
        for cube in self.sets {
            if bag.red < cube.red || bag.green < cube.green || bag.blue < cube.blue {
                return false
            }
        }
        true
    }

    fn fewest_number_of_cubes(self) -> SetOfCubes {
        let mut cubes = SetOfCubes{ red: 0, green: 0, blue: 0 };
        for set in self.sets {
            if set.red > cubes.red {
                cubes.red = set.red;
            }
            if set.green > cubes.green {
                cubes.green = set.green;
            }
            if set.blue > cubes.blue {
                cubes.blue = set.blue;
            }
        }
        cubes
    }
}

fn sum_of_game_ids(input : String) -> u32 {
    let bag = SetOfCubes{ red: 12, green: 13, blue: 14 };
    build_games(input)
        .into_iter()
        .map(|g|
            if g.clone().is_possible_with_bag(bag) { g.clone().id }
            else { 0 }
        )
        .collect::<Vec<u32>>()
        .iter()
        .sum()
}

fn build_games(input: String) -> Vec<Game> {
    input.split("\n")
        .flat_map(|g| Game::from_str(g))
        .collect()
}

fn sum_of_the_power_of_sets(input : String) -> u32 {
    build_games(input)
        .into_iter()
        .map(|g| g.fewest_number_of_cubes().power())
        .collect::<Vec<u32>>()
        .iter()
        .sum()
}

pub fn day2() {
    let result1 = sum_of_game_ids(INPUT1.into());
    println!("Result 1: {result1}");
    let result2 = sum_of_game_ids(INPUT2.into());
    println!("Result 2: {result2}");
    let result3 = sum_of_the_power_of_sets(INPUT1.into());
    println!("Result 3: {result3}");
    let result4 = sum_of_the_power_of_sets(INPUT2.into());
    println!("Result 4: {result4}");
}


#[test]
fn test_day1() {
    assert_eq!(8, sum_of_game_ids(INPUT1.into()));
    assert_eq!(2286, sum_of_the_power_of_sets(INPUT1.into()));
}