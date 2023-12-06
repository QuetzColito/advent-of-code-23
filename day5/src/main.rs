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

    let mut maps: HashMap<&str, &str> = HashMap::new();
    let mut mappings: HashMap<&str, Vec<(u64, u64, u64)>> = HashMap::new();
    let mut current_map: (&str, &str) = ("", "");
    lines.for_each(|line| {
        if line.contains("map") {
            let mut new_map = line.split_once(' ').unwrap().0.split('-');
            current_map.0 = new_map.next().unwrap();
            current_map.1 = new_map.last().unwrap();
            maps.insert(current_map.0, current_map.1);
        };
        if line.contains(|x: char| x.is_digit(10)) {
            let mut line_iter = line.split_whitespace().map(|x| x.parse().unwrap());
            let new_mapping = (
                line_iter.next().unwrap(),
                line_iter.next().unwrap(),
                line_iter.next().unwrap(),
            );
            mappings
                .entry(current_map.1)
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
                let next_category = maps.get(category).unwrap();

                for (destination_start, source_start, range) in mappings.get(next_category).unwrap()
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
    let mut lines = input.lines();
    let mut seeds: Vec<(u64, u64)> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|line| line.parse().ok())
        .collect::<Vec<u64>>()
        .chunks(2)
        .map(|range| (range[0], range[1]))
        .collect();

    let mut shifted_seeds: Vec<(u64, u64)> = vec![];
    lines.for_each(|line| {
        if line.contains("map") {
            seeds.extend(&shifted_seeds);
            shifted_seeds = vec![];
        }

        if line.contains(|x: char| x.is_digit(10)) {
            let shift: Vec<u64> = line
                .split_whitespace()
                .filter_map(|line| line.parse().ok())
                .collect();

            let destination = shift[0];
            let source = shift[1];
            let map_range = shift[2];
            let source_end = source + map_range;

            let mut new_seeds = vec![];

            seeds.iter().for_each(|(range_start, range)| {
                let range_start = *range_start;
                let range = *range;
                let range_end = range_start + range;
                // Source        |----------|
                // Range           |------|
                if range_start >= source && range_end <= source_end {
                    shifted_seeds.push((range_start - source + destination, range));
                // Source        |----------|
                // Range      |----------------|
                } else if range_start < source && range_end > source_end {
                    new_seeds.push((range_start, source - range_start));
                    new_seeds.push((source_end, range_end - (source_end)));
                    shifted_seeds.push((destination, map_range))
                // Source        |----------|
                // Range             |----------|
                } else if range_end > source && range_end <= source_end && range_start < source {
                    new_seeds.push((range_start, source - range_start));
                    shifted_seeds.push((destination, range_end - source));
                // Source        |----------|
                // Range    |---------|
                } else if range_start < source_end
                    && range_start >= source
                    && range_end > source_end
                {
                    new_seeds.push((source_end, range_end - (source_end)));
                    shifted_seeds
                        .push((range_start - source + destination, source_end - range_start));
                } else {
                    new_seeds.push((range_start, range));
                }
            });
            seeds = new_seeds;
        }
    });
    seeds.extend(&shifted_seeds);
    seeds.into_iter().map(|(x, _)| x).min().unwrap()
}
