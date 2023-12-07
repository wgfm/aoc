use std::collections::{HashMap, HashSet};

use crate::{problem::Solution, solution};

solution!(2023, 7);

impl Solution for Problem {
    fn part_a(&self) -> anyhow::Result<String> {
        let mut hand_bids = self
            .input
            .lines()
            .map(|s| {
                let (hand, bid) = s.split_once(" ").unwrap();
                let hand = Hand::parse(hand);
                let bid = usize::from_str_radix(bid, 10).unwrap();
                (hand, bid)
            })
            .collect::<Vec<_>>();

        hand_bids.sort_by(|(a, _), (b, _)| b.partial_cmp(a).unwrap());

        dbg!(&hand_bids[0..5]);

        let answer = hand_bids
            .iter()
            .map(|(_, bid)| bid)
            .enumerate()
            .map(|(i, bid)| (i + 1) * bid)
            .sum::<usize>();

        Ok(answer.to_string())
    }

    fn part_b(&self) -> anyhow::Result<String> {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
struct Hand {
    cards: Vec<Card>,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.hand_type() != other.hand_type() {
            return self.hand_type().partial_cmp(&other.hand_type());
        }

        for (a, b) in self.cards.iter().zip(other.cards.iter()) {
            if a != b {
                return a.partial_cmp(b);
            }
        }

        Some(std::cmp::Ordering::Equal)
    }
}

impl Hand {
    fn parse(input: &str) -> Self {
        let cards = input.chars().map(Card::parse).collect();

        Hand { cards }
    }

    // Hand type with Jacks
    /*
    fn hand_type(&self) -> HandType {
        // check which type of hand this is. A hand always has 5 cards.
        let map = self.cards.iter().fold(HashMap::new(), |mut map, card| {
            *map.entry(card).or_insert(0) += 1;
            map
        });

        if map.len() == 1 {
            HandType::FiveOfAKind
        } else if map.len() == 2 {
            if map.values().any(|&v| v == 4) {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            }
        } else if map.len() == 3 {
            if map.values().any(|&v| v == 3) {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPair
            }
        } else if map.len() == 4 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
    */

    // Hand type with Jokers
    fn hand_type(&self) -> HandType {
        let num_jokers = self.cards.iter().filter(|c| c == &&Card::J).count();
        let mut map =
            self.cards
                .iter()
                .filter(|c| c != &&Card::J)
                .fold(HashMap::new(), |mut map, card| {
                    *map.entry(card).or_insert(0) += 1;
                    map
                });

        if map.len() == 0 {
            return HandType::FiveOfAKind;
        }
        let (highest_card, _) = map.iter().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap();
        map.entry(highest_card).and_modify(|v| *v += num_jokers);
        if map.len() == 1 {
            HandType::FiveOfAKind
        } else if map.len() == 2 {
            if map.values().any(|&v| v == 4) {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            }
        } else if map.len() == 3 {
            if map.values().any(|&v| v == 3) {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPair
            }
        } else if map.len() == 4 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd)]
enum Card {
    A,
    K,
    Q,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    J,
}

impl Card {
    fn parse(c: char) -> Card {
        match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'T' => Card::T,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            'J' => Card::J,
            _ => panic!("invalid card: {}", c),
        }
    }
}
