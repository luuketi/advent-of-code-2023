#![allow(dead_code)]

use crate::inputs::day5::{INPUT1, INPUT2};

#[derive(Clone, Debug)]
struct Mapping {
    source_start: u64,
    source_end: u64,
    destination_start: u64,
    destination_end: u64,
}

#[derive(Clone, Debug)]
struct Map {
    mappings: Vec<Mapping>
}

impl Map {
    fn new() -> Self {
        Map{ mappings: Vec::new() }
    }

    fn add_mapping(&mut self, destination: u64, source: u64, length: u64) {
        self.mappings.push(
            Mapping {
                source_start: source,
                source_end: source + length,
                destination_start: destination,
                destination_end: destination + length
            }
        )
    }

    fn destination(self, source: u64) -> u64 {
        for mapping in self.mappings {
            if source >= mapping.source_start && source <= mapping.source_end {
                return source - mapping.source_start + mapping.destination_start;
            }
        }
        source
    }
}

#[derive(Clone, Debug)]
struct Almanac {
    maps: Vec<Map>,
}

impl Almanac {
    fn new(input :&str) -> Self {
        let maps :Vec<Map> = input
            .split("\n\n")
            .map(|m| Self::build_map(m.split(":\n").last().unwrap()))
            .collect();
        Almanac{ maps }
    }

    fn build_map(mappings: &str) -> Map {
        let mut map = Map::new();
        for mapping in mappings.split("\n") {
            let mut splitted_line = mapping.split_whitespace();
            let destination: u64 = splitted_line.next().unwrap().parse().unwrap();
            let source: u64 = splitted_line.next().unwrap().parse().unwrap();
            let length: u64 = splitted_line.next().unwrap().parse().unwrap();
            map.add_mapping(destination, source, length);
        }
        map
    }

    fn location(&self, seed: u64) -> u64 {
        let mut source = seed;
        for map in &self.maps {
            source = map.clone().destination(source);
        }
        source
    }
}


fn lowest_location_number(input : &str) -> u64 {
    let seeds : Vec<u64> = input
        .split("\n")
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let mappings = &input[input.find("\n\n").unwrap()+2..];
    let almanac = Almanac::new(mappings);
    let locations : Vec<u64> = seeds.iter().map(|s| almanac.location(*s)).collect();
    *locations.iter().min().unwrap()
}

fn lowest_location_number_from_range_of_seeds(input : &str) -> u64 {
    let mut seeds = input
        .split("\n")
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap());
    let mappings = &input[input.find("\n\n").unwrap()+2..];
    let almanac = Almanac::new(mappings);
    let mut all_seeds : Vec<(u64, u64)> = Vec::new();
    let mut lowest_location = u64::MAX;
    while let Some(start_seed) = seeds.next() {
        let end_seed = start_seed + seeds.next().unwrap();
        all_seeds.push((start_seed, end_seed));
    }
    for (start_seed, end_seed) in all_seeds {
        for s in start_seed..end_seed {
            let location = almanac.location(s);
            if location < lowest_location {
                lowest_location = location;
            }
        }
    }
    lowest_location
}

pub fn day5() {
    let result1 = lowest_location_number(INPUT1);
    println!("Result 1: {result1}");
    let result2 = lowest_location_number(INPUT2);
    println!("Result 2: {result2}");
    let result3 = lowest_location_number_from_range_of_seeds(INPUT1);
    println!("Result 3: {result3}");
    let result4 = lowest_location_number_from_range_of_seeds(INPUT2);
    println!("Result 2: {result4}");
}

#[test]
fn test_day5() {
    assert_eq!(35, lowest_location_number(INPUT1));
    assert_eq!(46, lowest_location_number_from_range_of_seeds(INPUT1));
}