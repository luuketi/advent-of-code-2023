#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use pathfinding::prelude::dijkstra_all;
use crate::inputs::day10::{INPUT1, INPUT2, INPUT3};

const VERTICAL_PIPE : char = '|';
const HORIZONTAL_PIPE : char = '-';
const NE_PIPE : char = 'L';
const NW_PIPE : char = 'J';
const SW_PIPE : char = '7';
const SE_PIPE : char = 'F';
const GROUND : char = '.';
const START : char = 'S';

#[derive(Eq, PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Hash)]
struct Point {
    x : u32,
    y : u32,
}

struct Map {
    map : HashMap<Point, HashSet<Point>>,
    start : Point,
}

impl Map {

    pub fn new(input: &str) -> Self {
        let mut map = HashMap::default();
        let mut start = Point{x:0, y:0};
        let max_row = input.split("\n").count() as u32;
        let max_column = input.split("\n").next().unwrap().len() as u32;

        for (row, line) in input.split("\n").enumerate() {
            let row = row as u32;
            for (column, c) in line.chars().enumerate() {
                let column = column as u32;
                match c {
                    VERTICAL_PIPE => Self::vertical_pipe(&mut map, &max_row, &column, &row),
                    HORIZONTAL_PIPE => Self::horizontal_pipe(&mut map, &max_column, &column, &row),
                    NE_PIPE => Self::ne_pipe(&mut map, &max_column, &column, &row),
                    NW_PIPE => Self::nw_pipe(&mut map, &column, &row),
                    SW_PIPE => Self::sw_pipe(&mut map, &max_row, &column, &row),
                    SE_PIPE => Self::se_pipe(&mut map, &max_column, &max_row, &column, &row),
                    _ => {}
                }
            }
        }
        Self::add_start_point(input, &mut start, &mut map, &max_row, &max_column);
        Map { map, start }
    }

    fn add_start_point(input: &str, start: &mut Point, mut map: &mut HashMap<Point, HashSet<Point>>, max_row: &u32, max_column: &u32) {
        for (row, line) in input.split("\n").enumerate() {
            for (column, c) in line.chars().enumerate() {
                match c {
                    START => {
                        *start = Point { x: column as u32, y: row as u32 };
                        Self::start_point(&mut map, &max_column, &max_row, &(column as u32), &(row as u32));
                        return;
                    },
                    _ => {}
                }
            }
        }
    }

    fn start_point(map: &mut HashMap<Point, HashSet<Point>>, max_column: &u32, max_row: &u32, x: &u32, y: &u32) {
        let start_point = Point{x:*x, y:*y};
        let mut connected_points: HashSet<Point> = HashSet::new();
        if *y > 0 {
            Self::add_connected_points(map, &start_point, &Point { x: *x, y: *y - 1 }, &mut connected_points);
        }
        if *x > 0 {
            Self::add_connected_points(map, &start_point, &Point { x: *x - 1, y: *y }, &mut connected_points);
        }
        if *y < *max_row {
            Self::add_connected_points(map, &start_point, &Point { x: *x, y: *y + 1 }, &mut connected_points);
        }
        if *x < *max_column {
            Self::add_connected_points(map, &start_point, &Point { x: *x + 1, y: *y }, &mut connected_points);
        }
        map.insert(start_point, connected_points);
    }

    fn add_connected_points(map: &mut HashMap<Point, HashSet<Point>>, start_point: &Point, adjacent_point :&Point, connected_points: &mut HashSet<Point>) {
        if let Some(points) = map.get(&adjacent_point) {
            if points.contains(&start_point) {
                connected_points.insert(adjacent_point.clone());
            }
        }
    }

    fn vertical_pipe(map: &mut HashMap<Point, HashSet<Point>>, max_row: &u32, x: &u32, y: &u32) {
        let point = Point{x:*x, y:*y};

         if *y < *max_row {
            Self::add_to_map(map, &point, &HashSet::from_iter(vec![Point{x:*x, y:*y+1}]));
        }
        if *y > 0 {
            Self::add_to_map(map, &point, &HashSet::from_iter(vec![Point{x:*x, y:*y-1}]));
        }
    }

    fn horizontal_pipe(map: &mut HashMap<Point, HashSet<Point>>, max_column: &u32, x: &u32, y: &u32) {
        let point = Point{x:*x, y:*y};

        if  *x < *max_column {
            Self::add_to_map(map, &point, &HashSet::from_iter(vec![Point{x:*x+1, y:*y}]));
        }
        if *x > 0 {
            Self::add_to_map(map, &point, &HashSet::from_iter(vec![Point{x:*x-1, y:*y}]));
        }
    }

    fn ne_pipe(map: &mut HashMap<Point, HashSet<Point>>, max_column: &u32, x: &u32, y: &u32) {
        let point = Point{x:*x, y:*y};

        if *x < *max_column {
            Self::add_to_map(map, &point, &HashSet::from_iter(vec![Point{x:*x+1, y:*y}]));
        }
        if *y > 0 {
            Self::add_to_map(map, &point, &HashSet::from_iter(vec![Point{x:*x, y:*y-1}]));
        }
    }

    fn nw_pipe(map: &mut HashMap<Point, HashSet<Point>>, x: &u32, y: &u32) {
        let point = Point{x:*x, y:*y};

        if *x > 0 {
            Self::add_to_map(map, &point, &HashSet::from_iter(vec![Point{x:*x-1, y:*y}]));
        }
        if *y > 0 {
            Self::add_to_map(map, &point, &HashSet::from_iter(vec![Point{x:*x, y:*y-1}]));
        }
    }

    fn sw_pipe(map: &mut HashMap<Point, HashSet<Point>>, max_row: &u32, x: &u32, y: &u32) {
        let point = Point{x:*x, y:*y};

        if *y < *max_row {
            Self::add_to_map(map, &point, &HashSet::from_iter(vec![Point{x:*x, y:*y+1}]));
        }
        if *x > 0 {
            Self::add_to_map(map, &point, &HashSet::from_iter(vec![Point{x:*x-1, y:*y}]));
        }
    }

    fn se_pipe(map: &mut HashMap<Point, HashSet<Point>>, max_column: &u32, max_row: &u32, x: &u32, y: &u32) {
        let point = Point{x:*x, y:*y};

        if *x < *max_row {
            Self::add_to_map(map, &point, &HashSet::from_iter(vec![Point{x:*x+1, y:*y}]));
        }
        if *y < *max_column {
            Self::add_to_map(map, &point, &HashSet::from_iter(vec![Point{x:*x, y:*y+1}]));
        }
    }

    fn add_to_map(map: &mut HashMap<Point, HashSet<Point>>, key: &Point, value: &HashSet<Point>) {
        if let Some(ref mut connected_points) = map.get_mut(&key) {
            connected_points.extend(value.clone());
        } else {
            map.insert(key.clone(), value.clone());
        }
    }

    fn get(&self, point : Point) -> HashSet<Point> {
        match self.map.get(&point) {
            Some(x) => x.clone(),
            None => HashSet::new(),
        }

    }
}


fn steps_to_farthest_point(input : &str) -> u32 {
    let map = Map::new(input);
    let path = dijkstra_all(&map.start, |n| {
        map.get(n.clone()).iter().map(|n| (n.clone(), 1)).collect::<Vec<_>>()
    });
    path.values().map(|(_, distance)| *distance as u32).max().unwrap()
}

pub fn day10() {
    let result1 = steps_to_farthest_point(INPUT1);
    println!("Result 1: {result1}");
    let result2 = steps_to_farthest_point(INPUT2);
    println!("Result 2: {result2}");
    let result3 = steps_to_farthest_point(INPUT3);
    println!("Result 3: {result3}");
}

#[test]
fn test_day10() {
    assert_eq!(4, steps_to_farthest_point(INPUT1));
    assert_eq!(8, steps_to_farthest_point(INPUT2));
}

