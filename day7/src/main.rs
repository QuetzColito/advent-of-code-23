use std::{collections::HashMap, time::Instant};

fn main() {
    let start_time = Instant::now();

    let input = include_str!("input.txt");
    let test = include_str!("test.txt");
    println!("{}", part1(test));
    println!("{}", part1(input));
    let end_time = Instant::now();
    println!("{:?}", end_time - start_time);

    let start_time = Instant::now();
    let input = include_str!("input.txt");
    let test = include_str!("test.txt");
    println!("{}", part2(test));
    println!("{}", part2(input));

    let end_time = Instant::now();
    println!("{:?}", end_time - start_time);
}

fn determine_strength_with_jokers(hand: &str) -> u64 {
    let mut unique_cards = HashMap::new();
    let mut jokers = 0;
    // dbg!(jokers);
    hand.chars().for_each(|card| {
        if card == 'J' {
            jokers += 1;
        }
        let counter = unique_cards.get(&card).unwrap_or(&0);
        unique_cards.insert(card, counter + 1);
    });

    let mut current_max = 0;
    let mut current_best_card = ' ';
    unique_cards.iter().for_each(|(card, amount)| {
        if *amount >= current_max && *card != 'J' {
            current_max = *amount;
            current_best_card = *card;
        }
    });

    if jokers > 0 {
        unique_cards.insert(current_best_card, current_max + jokers);
        unique_cards.remove(&'J');
    }

    match unique_cards.keys().len() {
        1 => 7,
        2 => {
            if unique_cards.values().any(|x| *x == 1 || *x == 4) {
                6
            } else {
                5
            }
        }
        3 => {
            if unique_cards.values().any(|x| *x == 3) {
                4
            } else {
                3
            }
        }
        4 => 2,
        _ => 1,
    }
}

fn determine_strength(hand: &str) -> u64 {
    let mut unique_cards = HashMap::new();
    hand.chars().for_each(|card| {
        let counter = unique_cards.get(&card).unwrap_or(&0);
        unique_cards.insert(card, counter + 1);
    });

    match unique_cards.keys().len() {
        1 => 7,
        2 => {
            if unique_cards.values().any(|x| *x == 1 || *x == 4) {
                6
            } else {
                5
            }
        }
        3 => {
            if unique_cards.values().any(|x| *x == 3) {
                4
            } else {
                3
            }
        }
        4 => 2,
        _ => 1,
    }
}

fn card_to_value(card: char) -> u64 {
    if card.is_digit(10) {
        card.to_digit(10).unwrap() as u64
    } else {
        match card {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => unreachable!(),
        }
    }
}

fn card_to_value_with_jokers(card: char) -> u64 {
    if card.is_digit(10) {
        card.to_digit(10).unwrap() as u64
    } else {
        match card {
            'T' => 10,
            'J' => 1,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => unreachable!(),
        }
    }
}

fn hand_to_value(hand: &str) -> u64 {
    hand.chars().fold(determine_strength(hand), |acc, card| {
        acc * 16 + card_to_value(card)
    })
}

fn hand_to_value_with_jokers(hand: &str) -> u64 {
    hand.chars()
        .fold(determine_strength_with_jokers(hand), |acc, card| {
            acc * 16 + card_to_value_with_jokers(card)
        })
}

fn part1(input: &str) -> u64 {
    let mut hands: Vec<(u64, u64)> = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let bid = bid.parse::<u64>().unwrap();
            let hand = hand_to_value(hand);
            (hand, bid)
        })
        .collect();
    hands.sort_by_key(|pair| pair.0);
    // dbg!(hands.clone());

    hands
        .into_iter()
        .map(|pair| pair.1)
        .enumerate()
        .map(|(rank, bid)| ((rank as u64) + 1) * bid)
        .sum()
}

fn part2(input: &str) -> u64 {
    let mut hands: Vec<(u64, u64)> = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let bid = bid.parse::<u64>().unwrap();
            let hand = hand_to_value_with_jokers(hand);
            (hand, bid)
        })
        .collect();
    hands.sort_by_key(|pair| pair.0);

    hands
        .into_iter()
        .map(|pair| pair.1)
        .enumerate()
        .map(|(rank, bid)| ((rank as u64) + 1) * bid)
        .sum()
}
