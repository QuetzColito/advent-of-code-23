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
    let seeds: Vec<(u64, u64)> = lines.next().unwrap()[6..]
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u64>>()
        .chunks(2)
        .map(|range| (range[0], range[1]))
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
        .flat_map(|seed| {
            let mut category = "seed";
            let mut new_seeds = vec![seed];
            while category != "location" {
                let next_category = maps.get(category).unwrap();
                new_seeds = new_seeds
                    .into_iter()
                    .flat_map(|range| {
                        apply_shifts(range, mappings.get(next_category).unwrap().as_slice())
                    })
                    .collect();
                category = next_category;
            }
            new_seeds
        })
        .map(|(start, _)| start)
        .min()
        .unwrap()
}

// 70224450
fn apply_shifts(range: (u64, u64), shifts: &[(u64, u64, u64)]) -> Vec<(u64, u64)> {
    if shifts.len() == 0 {
        return vec![range];
    }

    let (destination_start, source_start, map_range) = shifts[0];
    let source_end = source_start + map_range;
    let range_end = range.0 + range.1;
    if range.0 >= source_start && range_end <= source_end {
        apply_shifts(
            (range.0 + destination_start - source_start, range.1),
            &shifts[1..],
        )
    } else if range.0 < source_start && range_end > source_end {
        let mut result = vec![(source_start + destination_start - source_start, map_range)];
        result.extend(apply_shifts(
            (range.0, source_start - range.0),
            &shifts[1..],
        ));
        result.extend(apply_shifts(
            (source_end, range_end - source_end),
            &shifts[1..],
        ));
        result
    } else if range.0 < source_start && range_end <= source_end && range_end > source_start {
        let mut result = vec![(
            source_start + destination_start - source_start,
            range_end - source_start,
        )];
        result.extend(apply_shifts(
            (range.0, source_start - range.0),
            &shifts[1..],
        ));
        result
    } else if range.0 >= source_start && range_end > source_end && range.0 < source_end {
        let mut result = vec![(
            range.0 + destination_start - source_start,
            source_end - range.0,
        )];
        result.extend(apply_shifts(
            (source_end, range_end - source_end),
            &shifts[1..],
        ));
        result
    } else {
        apply_shifts(range, &shifts[1..])
    }
}
