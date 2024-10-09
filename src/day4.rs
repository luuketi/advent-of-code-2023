#![allow(dead_code)]

use std::collections::{BTreeMap, HashSet};
use crate::inputs::day4::{INPUT1, INPUT2};

#[derive(Clone, Debug)]
struct Card {
    id : u32,
    winning_numbers : HashSet<u32>
}

impl Card {
    fn new(id: u32, winning_numbers : Vec<u32>) -> Self {
        Card{id, winning_numbers: HashSet::from_iter(winning_numbers)}
    }

    fn points(self, numbers : Vec<u32>) -> u32 {
        let mut points = 0;
        for n in numbers {
            if self.winning_numbers.contains(&n) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }
        points
    }

    fn matching_numbers(self, numbers : Vec<u32>) -> u32 {
        let matching_numbers : Vec<u32> = numbers
            .into_iter()
            .filter(|n| self.winning_numbers.contains(&n))
            .collect();
        matching_numbers.len() as u32
    }
}

struct CardPile {
    cards : BTreeMap<u32, Card>,
    numbers : Vec<Vec<u32>>,
    copies : BTreeMap<u32, u32>,
}

impl CardPile {
    fn new() -> Self {
        CardPile{ cards: Default::default(), numbers: vec![], copies: Default::default() }
    }

    fn add_card_and_numbers(&mut self, card: Card, numbers : Vec<u32>) {
        let card_id = card.id;
        self.cards.insert(card_id, card);
        self.numbers.push(numbers);
        self.copies.insert(card_id, 0);
    }

    fn total_cards(&mut self) -> u32 {
        for (id, card) in &self.cards.clone() {
            self.add_copies(id, card);
        }
        Vec::from_iter(self.copies.values().cloned()).into_iter().sum::<u32>()
            + (self.cards.len() as u32)
    }

    fn add_copies(&mut self, id: &u32, card: &Card) {
        let matching_numbers = card
            .clone()
            .matching_numbers(self.numbers.get((id - 1) as usize)
            .unwrap()
            .to_vec());
        let copies: Vec<u32> = ((id + 1)..(id + 1 + matching_numbers)).collect();
        for copy in copies {
            *self.copies.get_mut(&copy).unwrap() += 1;
            for _ in 0..*self.copies.get(&id).unwrap() {
                *self.copies.get_mut(&copy).unwrap() += 1;
            }
        }

    }
}

fn sum_points(input: &str) -> u32 {
    let mut total_points = 0;
    for line in input.split("\n") {
        let mut splitted_numbers = line.split(": ").last().unwrap().split(" | ");
        let winning_numbers : Vec<u32>= splitted_numbers
            .next()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect();
        let numbers : Vec<u32> = splitted_numbers
            .last()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect();
        total_points += Card::new(0, winning_numbers).points(numbers);
    }
    total_points
}

fn count_total_cards(input: &str) -> u32 {
    let mut card_pile = CardPile::new();
    for line in input.split("\n") {
        let mut splitted_line = line.split(": ");
        let id : u32 = splitted_line.next().unwrap().split(" ").last().unwrap().parse().unwrap();
        let mut splitted_numbers = splitted_line.last().unwrap().split(" | ");
        let winning_numbers : Vec<u32>= splitted_numbers
            .next()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect();
        let numbers : Vec<u32> = splitted_numbers
            .last()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect();
        card_pile.add_card_and_numbers(Card::new(id, winning_numbers), numbers);
    }
    card_pile.total_cards()
}

pub fn day4() {
    let result1 = sum_points(INPUT1);
    println!("Result 1: {result1}");
    let result2 = sum_points(INPUT2);
    println!("Result 2: {result2}");
    let result3 = count_total_cards(INPUT1);
    println!("Result 3: {result3}");
    let result4 = count_total_cards(INPUT2);
    println!("Result 4: {result4}");
}


#[test]
fn test_day1() {
    assert_eq!(13, sum_points(INPUT1));
    assert_eq!(30, count_total_cards(INPUT1));
}