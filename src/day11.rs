#![allow(dead_code)]

use std::collections::{HashMap};
use crate::inputs::day11::{INPUT1, INPUT2};

const EMPTY : char = '.';
const GALAXY : char = '#';


#[derive(Eq, PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Hash)]
struct Point {
    x : u64,
    y : u64,
}

#[derive(Clone)]
struct Universe {
    map : HashMap<Point, char>,
    size : u64,
    galaxy_positions : Vec<Point>,
}

impl Universe {
    pub fn new(input: &str) -> Self {
        let mut map: HashMap<Point, char> = HashMap::default();
        let size = input.split("\n").count() as u64;
        for (row, line) in input.split("\n").enumerate() {
            for (column, c) in line.chars().enumerate() {
                map.insert(Point { x: row as u64, y: column as u64 }, c);
            }
        }
        Self { map, size, galaxy_positions: vec![] }
    }

    pub fn expand(&mut self, times: u32) {
        let rows_to_expand: Vec<u64> = (0..self.size).filter(|x| (0..self.size).all(|y| self.map[&Point { x: *x, y }] == EMPTY)).collect();
        let columns_to_expand: Vec<u64> = (0..self.size).filter(|y| (0..self.size).all(|x| self.map[&Point { x, y: *y }] == EMPTY)).collect();
        self.compute_galaxy_positions();
        self.row_expand_map(&rows_to_expand, times);
        self.column_expand_map(&columns_to_expand, times);
    }

    pub fn sum_of_shortest_paths(self) -> u64 {
        let mut path_lengths : u64 = 0;
        for (i, g) in self.galaxy_positions.clone().iter().enumerate() {
            let other_galaxies = &self.galaxy_positions[i+1..];
            for o in other_galaxies {
                path_lengths += g.x.abs_diff(o.x) as u64 + g.y.abs_diff(o.y) as u64
            }
        }
        path_lengths
    }

    fn compute_galaxy_positions(&mut self) {
        self.galaxy_positions = self.clone().map.into_iter().filter_map(|(key, value)| {
            if value == GALAXY {
                Some(key)
            } else { None }
        }).collect()
    }

    fn row_expand_map(&mut self, rows_to_expand: &Vec<u64>, times: u32) {
        for r in rows_to_expand.iter().rev() {
            let mut new_galaxy_positions : Vec<Point> = Vec::new();
            for g in &self.galaxy_positions {
                if g.x > *r {
                    new_galaxy_positions.push(Point{x: g.x + times  as u64 - 1 , y: g.y})
                } else {
                    new_galaxy_positions.push(g.clone())
                }
            }
            self.galaxy_positions = new_galaxy_positions;
        }
    }

    fn column_expand_map(&mut self, columns_to_expand: &Vec<u64>, times: u32) {
        for c in columns_to_expand.iter().rev() {
            let mut new_galaxy_positions : Vec<Point> = Vec::new();
            for g in &self.galaxy_positions {
                if g.y > *c {
                    new_galaxy_positions.push(Point{x: g.x, y: g.y + times  as u64 - 1 })
                } else {
                    new_galaxy_positions.push(g.clone())
                }
            }
            self.galaxy_positions = new_galaxy_positions;
        }
    }
}

fn sum_of_shortest_paths(input : &str) -> u64 {
    let mut universe = Universe::new(input);
    universe.expand(2);
    universe.sum_of_shortest_paths()
}

fn sum_of_shortest_paths_expanding_universe(input : &str, times: u32) -> u64 {
    let mut universe = Universe::new(input);
    universe.expand(times);
    universe.sum_of_shortest_paths()
}


pub fn day11() {
    let result1 = sum_of_shortest_paths(INPUT1);
    println!("Result 1: {result1}");
    let result2 = sum_of_shortest_paths(INPUT2);
    println!("Result 2: {result2}");
    let result3 = sum_of_shortest_paths_expanding_universe(INPUT1, 10);
    println!("Result 3: {result3}");
    let result4 = sum_of_shortest_paths_expanding_universe(INPUT2, 1000000);
    println!("Result 4: {result4}");
}

#[test]
fn test_day11() {
    assert_eq!(374, sum_of_shortest_paths(INPUT1));
    assert_eq!(1030, sum_of_shortest_paths_expanding_universe(INPUT1, 10));
    assert_eq!(8410, sum_of_shortest_paths_expanding_universe(INPUT1, 100));
}
