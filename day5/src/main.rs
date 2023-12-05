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

fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let seeds: Vec<u64> = lines.next().unwrap()[6..]
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut mappings: HashMap<(&str, &str), Vec<(u64, u64, u64)>> = HashMap::new();
    let mut current_map: (&str, &str) = ("", "");
    lines.for_each(|line| {
        if line.contains("map") {
            let mut new_map = line.split_once(' ').unwrap().0.split('-');
            current_map.0 = new_map.next().unwrap();
            current_map.1 = new_map.last().unwrap();
        };
        if line.contains(|x: char| x.is_digit(10)) {
            let mut line_iter = line.split_whitespace().map(|x| x.parse().unwrap());
            let new_mapping = (
                line_iter.next().unwrap(),
                line_iter.next().unwrap(),
                line_iter.next().unwrap(),
            );
            mappings
                .entry(current_map)
                .or_insert(vec![])
                .push(new_mapping);
        };
    });

    seeds
        .into_iter()
        .map(|seed| {
            let mut category = "seed";
            let mut seed = seed;
            while category != "location" {
                let next_category = mappings
                    .keys()
                    .find(|(source, _)| *source == category)
                    .unwrap()
                    .1;

                for (destination_start, source_start, range) in
                    mappings.get(&(category, next_category)).unwrap()
                {
                    if seed >= *source_start && seed <= source_start + range {
                        seed = destination_start + seed - source_start;
                        break;
                    }
                }
                category = next_category;
            }
            seed
        })
        .min()
        .unwrap()
}

fn part2(input: &str) -> u64 {
    2
}
