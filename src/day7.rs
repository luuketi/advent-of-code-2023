#![allow(dead_code)]

use counter::Counter;
use std::cmp::Ordering;
use std::collections::{HashMap};
use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;
use crate::inputs::day7::{INPUT1, INPUT2};


const CARDS : &str = "AKQJT98765432";
const CARDS_WITH_JOKER : &str = "AKQT98765432J";

#[derive(Debug, PartialEq, Eq)]
struct ParseError;

trait CardCmp {
    fn get_cards(&self) -> String;
    fn compare_cards(&self, other: &Self) -> Ordering;

    fn five_of_a_kind(&self) -> bool {
        self.get_cards().chars().collect::<Counter<_>>().len() == 1
    }

    fn four_of_a_kind(&self) -> bool {
        let counts = self.get_cards().chars().collect::<Counter<_>>();
        let top_two = counts.k_most_common_ordered(2);
        counts.len() == 2 && top_two[0].1 == 4 && top_two[1].1 == 1
    }

    fn full_house(&self) -> bool {
        let counts = self.get_cards().chars().collect::<Counter<_>>();
        let top_two = counts.k_most_common_ordered(2);
        counts.len() == 2 && top_two[0].1 == 3 && top_two[1].1 == 2
    }

    fn three_of_a_kind(&self) -> bool {
        let counts = self.get_cards().chars().collect::<Counter<_>>();
        let top_two = counts.k_most_common_ordered(2);
        counts.len() == 3 && top_two[0].1 == 3 && top_two[1].1 == 1
    }

    fn two_pairs(&self) -> bool {
        let counts = self.get_cards().chars().collect::<Counter<_>>();
        let top_two = counts.k_most_common_ordered(2);
        counts.len() == 3 && top_two[0].1 == 2 && top_two[1].1 == 2
    }

    fn one_pair(&self) -> bool {
        let counts = self.get_cards().chars().collect::<Counter<_>>();
        let top_two = counts.k_most_common_ordered(2);
        counts.len() == 4 && top_two[0].1 == 2 && top_two[1].1 == 1
    }

    fn high_card(&self) -> bool {
        let counts = self.get_cards().chars().collect::<Counter<_>>();
        counts.len() == 5
    }

    fn compare(&self, other: &Self) -> Ordering {
        if self.five_of_a_kind() && !other.five_of_a_kind() {
            return Ordering::Greater
        }
        if other.five_of_a_kind() && !self.five_of_a_kind() {
            return Ordering::Less
        }
        if self.four_of_a_kind() && !other.four_of_a_kind() {
            return Ordering::Greater
        }
        if other.four_of_a_kind() && !self.four_of_a_kind() {
            return Ordering::Less
        }
        if self.full_house() && !other.full_house() {
            return Ordering::Greater
        }
        if other.full_house() && !self.full_house() {
            return Ordering::Less
        }
        if self.three_of_a_kind() && !other.three_of_a_kind() {
            return Ordering::Greater
        }
        if other.three_of_a_kind() && !self.three_of_a_kind() {
            return Ordering::Less
        }
        if self.two_pairs() && !other.two_pairs() {
            return Ordering::Greater
        }
        if other.two_pairs() && !self.two_pairs() {
            return Ordering::Less
        }
        if self.one_pair() && !other.one_pair() {
            return Ordering::Greater
        }
        if other.one_pair() && !self.one_pair() {
            return Ordering::Less
        }
        if self.high_card() && !other.high_card() {
            return Ordering::Greater
        }
        if other.high_card() && !self.high_card() {
            return Ordering::Less
        }
        self.compare_cards(other)
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Eq)]
#[derive(Hash)]
struct Hand {
    cards : String,
}

impl FromStr for Hand {
    type Err = ParseError;

    fn from_str(cards: &str) -> Result<Self, Self::Err> {
        Ok(Hand{ cards: cards.into() })
    }
}

impl CardCmp for Hand {

    fn get_cards(&self) -> String {
        self.cards.clone()
    }

    fn compare_cards(&self, other: &Self) -> Ordering {
        for (a, b) in self.get_cards().chars().zip(other.get_cards().chars()) {
            let idx_a = CARDS.find(a).unwrap();
            let idx_b = CARDS.find(b).unwrap();
            if idx_a < idx_b {
                return Ordering::Greater
            } else if idx_a > idx_b {
                return Ordering::Less
            }
        }
        Ordering::Equal
    }

}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd<Hand> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        CardCmp::compare(self, other)
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Eq)]
#[derive(Hash)]
struct HandWithJockers {
    cards : String,
    cards_without_jockers : String,
}

fn cards_with_replaced_jockers(cards : &str) -> String {
    let most_repeated_card_excluding_jocker = str::replace(cards, "J", "")
        .chars()
        .collect::<Counter<_>>()
        .k_most_common_ordered(1);
    if !most_repeated_card_excluding_jocker.is_empty() {
        str::replace(cards, "J", &*most_repeated_card_excluding_jocker[0].0.to_string())
    } else {
        cards.into()
    }
}


impl FromStr for HandWithJockers {
    type Err = ParseError;

    fn from_str(cards: &str) -> Result<Self, Self::Err> {
        Ok(HandWithJockers{
            cards: cards.into(),
            cards_without_jockers: cards_with_replaced_jockers(cards)
        })
    }
}

impl CardCmp for HandWithJockers {

    fn get_cards(&self) -> String {
        self.cards_without_jockers.clone()
    }

    fn compare_cards(&self, other: &Self) -> Ordering {
        for (a, b) in self.cards.chars().zip(other.cards.chars()) {
            let idx_a = CARDS_WITH_JOKER.find(a).unwrap();
            let idx_b = CARDS_WITH_JOKER.find(b).unwrap();
            if idx_a < idx_b {
                return Ordering::Greater
            } else if idx_a > idx_b {
                return Ordering::Less
            }
        }
        Ordering::Equal
    }
}

impl PartialEq for HandWithJockers {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd<HandWithJockers> for HandWithJockers {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandWithJockers {
    fn cmp(&self, other: &Self) -> Ordering {
        CardCmp::compare(self, other)
    }
}

struct CamelCards<T> {
    hands : Vec<T>,
    bids : HashMap<T, u32>,
}

impl<T: Clone + Debug + FromStr + Hash + Ord> CamelCards<T> {
    pub fn new( input : &str ) -> Self where <T as FromStr>::Err: Debug  {
        let mut hands = Vec::new();
        let mut bids = HashMap::default();
        for line in input.split("\n") {
            let mut splitted_line = line.split(" ");
            let cards: String = splitted_line.next().unwrap().into();
            let bid: u32 = splitted_line.next().unwrap().parse().unwrap();
            hands.push(T::from_str(&*cards).unwrap());
            bids.insert(T::from_str(&*cards).unwrap(), bid);
        }
        CamelCards{hands, bids}
    }

    fn total_winnings(mut self) -> u32 {
        self.hands.sort();
        self.hands.clone()
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, hand)| (self.hands.len()-i) as u32 * *self.bids.get(&hand).unwrap())
            .collect::<Vec<u32>>()
            .iter()
            .sum()
    }
}


pub fn day7() {
    let result1 = CamelCards::<Hand>::new(INPUT1).total_winnings();
    println!("Result 1: {result1}");
    let result2 = CamelCards::<Hand>::new(INPUT2).total_winnings();
    println!("Result 2: {result2}");
    let result3 = CamelCards::<HandWithJockers>::new(INPUT1).total_winnings();
    println!("Result 3: {result3}");
    let result4 = CamelCards::<HandWithJockers>::new(INPUT2).total_winnings();
    println!("Result 4: {result4}");
}

#[test]
fn test_day7() {
    assert_eq!(6440, CamelCards::<Hand>::new(INPUT1).total_winnings());
    assert_eq!(5905, CamelCards::<HandWithJockers>::new(INPUT1).total_winnings());
}

#[test]
fn test_hand() {
    assert_eq!(true, Hand{cards:"KTJJT".into()} < Hand{cards:"KK677".into()});
}

#[test]
fn test_hand_with_jockers() {
    assert_eq!(true, HandWithJockers::from_str("32T3K").unwrap() < HandWithJockers::from_str("KK677").unwrap());
    assert_eq!(true, HandWithJockers::from_str("KK677").unwrap() < HandWithJockers::from_str("T55J5").unwrap());
    assert_eq!(true, HandWithJockers::from_str("T55J5").unwrap() < HandWithJockers::from_str("QQQJA").unwrap());
    assert_eq!(true, HandWithJockers::from_str("QQQJA").unwrap() < HandWithJockers::from_str("KTJJT").unwrap());
}