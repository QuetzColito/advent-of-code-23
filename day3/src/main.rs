use std::{collections::HashMap, time::Instant};

fn main() {
    let start_time = Instant::now();

    let s = include_str!("test.txt");
    let s2 = include_str!("input.txt");

    println!("{:?}", part1(s));
    println!("{:?}", part1(s2));

    let end_time = Instant::now();
    println!("{:?}", end_time - start_time);

    let start_time = Instant::now();

    let s = include_str!("test.txt");
    let s2 = include_str!("input.txt");

    println!("{:?}", part2(s));
    println!("{:?}", part2(s2));

    let end_time = Instant::now();
    println!("{:?}", end_time - start_time);
}

fn part2(input: &str) -> u32 {
    make_adj(input)
        .values()
        .filter_map(|list| {
            if list.len() == 2 {
                Some(list[0] * list[1])
            } else {
                None
            }
        })
        .sum()
}

fn part1(input: &str) -> u32 {
    let input_vector: Vec<&str> = input.lines().collect();
    let mut part_numbers: Vec<u32> = vec![];

    input.lines().enumerate().for_each(|(line_index, line)| {
        let mut current = 0;
        let mut last_was_digit = false;
        let mut parts = vec![];
        line.chars().enumerate().for_each(|(char_index, char)| {
            if char.is_digit(10) {
                if !last_was_digit && char_index > 0 {
                    parts = add_parts_1(&parts, &input_vector, line_index, char_index - 1)
                }
                parts = add_parts_1(&parts, &input_vector, line_index, char_index);
                last_was_digit = true;
                current = current * 10 + char.to_digit(10).unwrap();
            } else if last_was_digit {
                last_was_digit = false;
                parts = add_parts_1(&parts, &input_vector, line_index, char_index);
                if !parts.is_empty() {
                    part_numbers.push(current);
                }
                current = 0;
                parts = vec![];
            };
        });
        if !parts.is_empty() {
            part_numbers.push(current);
        }
    });
    part_numbers.into_iter().sum()
}

fn make_adj(input: &str) -> HashMap<(u32, u32), Vec<u32>> {
    let input_vector: Vec<&str> = input.lines().collect();
    let mut adj: HashMap<(u32, u32), Vec<u32>> = HashMap::new();

    input.lines().enumerate().for_each(|(line_index, line)| {
        let mut current = 0;
        let mut last_was_digit = false;
        let mut parts = vec![];
        line.chars().enumerate().for_each(|(char_index, char)| {
            if char.is_digit(10) {
                if !last_was_digit && char_index > 0 {
                    parts = add_parts(&parts, &input_vector, line_index, char_index - 1)
                }
                parts = add_parts(&parts, &input_vector, line_index, char_index);
                last_was_digit = true;
                current = current * 10 + char.to_digit(10).unwrap();
            } else if last_was_digit {
                last_was_digit = false;
                parts = add_parts(&parts, &input_vector, line_index, char_index);
                for part in &parts {
                    adj.entry(*part).or_insert(vec![]).push(current);
                }
                current = 0;
                parts = vec![];
            };
        });
        if !parts.is_empty() {
            for part in &parts {
                adj.entry(*part).or_insert(vec![]).push(current);
            }
        }
    });
    adj
}

fn add_parts(
    parts: &Vec<(u32, u32)>,
    lines: &Vec<&str>,
    line_index: usize,
    char_index: usize,
) -> Vec<(u32, u32)> {
    let mut parts = (*parts).clone();
    if line_index > 0
        && lines[line_index - 1].chars().nth(char_index).unwrap() == '*'
        && !lines[line_index - 1]
            .chars()
            .nth(char_index)
            .unwrap()
            .is_digit(10)
    {
        parts.push(((line_index - 1) as u32, char_index as u32))
    }
    if lines[line_index].chars().nth(char_index).unwrap() == '*'
        && !lines[line_index]
            .chars()
            .nth(char_index)
            .unwrap()
            .is_digit(10)
    {
        parts.push((line_index as u32, char_index as u32))
    }
    if line_index + 1 < lines.len()
        && lines[line_index + 1].chars().nth(char_index).unwrap() == '*'
        && !lines[line_index + 1]
            .chars()
            .nth(char_index)
            .unwrap()
            .is_digit(10)
    {
        parts.push(((line_index + 1) as u32, char_index as u32))
    }
    parts
}

fn add_parts_1(
    parts: &Vec<(u32, u32)>,
    lines: &Vec<&str>,
    line_index: usize,
    char_index: usize,
) -> Vec<(u32, u32)> {
    let mut parts = (*parts).clone();
    if line_index > 0
        && lines[line_index - 1].chars().nth(char_index).unwrap() != '.'
        && !lines[line_index - 1]
            .chars()
            .nth(char_index)
            .unwrap()
            .is_digit(10)
    {
        parts.push(((line_index - 1) as u32, char_index as u32))
    }
    if lines[line_index].chars().nth(char_index).unwrap() != '.'
        && !lines[line_index]
            .chars()
            .nth(char_index)
            .unwrap()
            .is_digit(10)
    {
        parts.push((line_index as u32, char_index as u32))
    }
    if line_index + 1 < lines.len()
        && lines[line_index + 1].chars().nth(char_index).unwrap() != '.'
        && !lines[line_index + 1]
            .chars()
            .nth(char_index)
            .unwrap()
            .is_digit(10)
    {
        parts.push(((line_index + 1) as u32, char_index as u32))
    }
    parts
}
