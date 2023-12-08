use num_integer::lcm;
use std::{collections::HashMap, time::Instant};

fn main() {
    let start_time = Instant::now();

    let input = include_str!("input.txt");
    let test = include_str!("test.txt");
    println!("{}", part1(test));
    println!("{}", part1(input));
    let end_time = Instant::now();
    println!("{:?}", end_time - start_time);

    println!("{:?}", str_to_node("ZZZ"));

    let start_time = Instant::now();
    let input = include_str!("input.txt");
    let test = include_str!("test2.txt");
    println!("{}", part2(test));
    println!("{}", part2(input));

    let end_time = Instant::now();
    println!("{:?}", end_time - start_time);
}

fn str_to_node(input: &str) -> u32 {
    input
        .chars()
        .filter_map(|char| char.to_digit(36))
        .fold(0, |acc, x| acc * 36 + x)
}

fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().collect::<Vec<char>>();
    lines.next();
    let nodes: HashMap<u32, (u32, u32)> = lines
        .map(|line| {
            let (key, destinations) = line.split_once('=').unwrap();
            let (left, right) = destinations.split_once(',').unwrap();
            (str_to_node(key), (str_to_node(left), str_to_node(right)))
        })
        .collect();
    // dbg!(nodes.clone());
    let mut counter = 0;
    let mut current_location = 13330;
    for instruction in instructions.iter().cycle() {
        if current_location == 46655 {
            break;
        } else {
            counter += 1;
            current_location = match instruction {
                'L' => nodes.get(&current_location).unwrap().0,
                'R' => nodes.get(&current_location).unwrap().1,
                _ => unreachable!(),
            }
        }
    }
    counter
}

fn get_steps(start: u32, instructions: &Vec<char>, nodes: &HashMap<u32, (u32, u32)>) -> u64 {
    let mut counter = 0;
    let mut current_location = start;
    for instruction in instructions.iter().cycle() {
        if current_location % 36 == 35 {
            break;
        } else {
            counter += 1;
            current_location = match instruction {
                'L' => nodes.get(&current_location).unwrap().0,
                'R' => nodes.get(&current_location).unwrap().1,
                _ => unreachable!(),
            }
        }
    }
    counter
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().collect::<Vec<char>>();
    let mut starting_points = vec![];
    lines.next();
    let nodes: HashMap<u32, (u32, u32)> = lines
        .map(|line| {
            let (key, destinations) = line.split_once('=').unwrap();
            if str_to_node(key) % 36 == 10 {
                starting_points.push(str_to_node(key))
            };
            let (left, right) = destinations.split_once(',').unwrap();
            (str_to_node(key), (str_to_node(left), str_to_node(right)))
        })
        .collect();

    // dbg!(starting_points.clone());

    starting_points
        .into_iter()
        .map(|start| get_steps(start, &instructions, &nodes))
        .fold(1, lcm)
}
