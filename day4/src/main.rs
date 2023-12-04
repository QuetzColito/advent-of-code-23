use std::time::Instant;

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

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|x| {
            let (_, cards) = x.split_once(':').unwrap();
            let (winning_cards, drawn_cards) = cards.split_once('|').unwrap();
            let count = drawn_cards
                .split_whitespace()
                .filter(|drawn_card| {
                    winning_cards
                        .split_whitespace()
                        .any(|winning_card| *drawn_card == winning_card)
                })
                .count() as u32;
            if count == 0 {
                count
            } else {
                2_u32.pow(count - 1)
            }
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let mut wins = vec![1; input.lines().count()];
    input
        .lines()
        .map(|x| {
            let (card_number, cards) = x.split_once(':').unwrap();
            let (winning_cards, drawn_cards) = cards.split_once('|').unwrap();
            let count = drawn_cards
                .split_whitespace()
                .filter(|drawn_card| {
                    winning_cards
                        .split_whitespace()
                        .any(|winning_card| *drawn_card == winning_card)
                })
                .count() as u32;

            let number: u32 = card_number[5..].trim_start().parse().unwrap();

            for index in number..number + count {
                wins[index as usize] += wins[(number - 1) as usize];
            }

            wins[(number - 1) as usize]
        })
        .sum()
}
