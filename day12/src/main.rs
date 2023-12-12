use std::{str::Chars, thread::current, time::Instant};

fn main() {
    let start_time = Instant::now();

    let input = include_str!("input.txt");
    let test = include_str!("test.txt");
    println!("{}", part1(test));
    // println!("{}", part1(input));
    let end_time = Instant::now();
    println!("{:?}", end_time - start_time);

    let start_time = Instant::now();
    let input = include_str!("input.txt");
    let test = include_str!("test.txt");
    // println!("{}", part2(test));
    // println!("{}", part2(input));

    let end_time = Instant::now();
    println!("{:?}", end_time - start_time);
}

fn find_possibilities(mut springs: Chars, mut current: Vec<i32>, known_patches: &Vec<i32>) -> i32 {
    if current.len() > known_patches.len()
        || current[..current.len() - 1] != known_patches[..current.len() - 1]
    {
        // dbg!(current);
        return 0;
    };
    // dbg!(springs.clone());

    match springs.next() {
        Some('.') => {
            if *current.last().unwrap() != 0 {
                current.push(0);
            }
            find_possibilities(springs, current, known_patches)
        }

        Some('#') => {
            let last = current.pop().unwrap();
            current.push(last + 1);
            find_possibilities(springs, current, known_patches)
        }

        Some('?') => {
            let mut current2 = current.clone();
            if *current.last().unwrap() != 0 {
                current.push(0);
            }
            let last = current2.pop().unwrap();
            current2.push(last + 1);
            find_possibilities(springs.clone(), current, known_patches)
                + find_possibilities(springs, current2, known_patches)
        }

        None => {
            // dbg!(known_patches.clone());
            if *current.last().unwrap() == 0 {
                current.pop();
            }
            if current == *known_patches {
                dbg!(current.clone());
                1
            } else {
                0
            }
        }

        _ => unreachable!(),
    }
}

fn spring_matches(springs: &str, known_patches: &Vec<u64>) -> bool {
    springs
        .split_terminator('.')
        .filter_map(|str| {
            let length = str.len();
            if length == 0 {
                None
            } else {
                Some(length as u64)
            }
        })
        .collect::<Vec<u64>>()
        == *known_patches
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let (springs, known_patches) = line.split_once(' ').unwrap();
            let known_patches = known_patches
                .split(',')
                .filter_map(|number| number.parse::<i32>().ok())
                .collect::<Vec<i32>>();
            dbg!(known_patches.clone());
            find_possibilities(springs.chars(), vec![0], &known_patches)
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let (springs, known_patches) = line.split_once(' ').unwrap();
            let springs = repeats(springs, '?');
            let known_patches = repeats(known_patches, ',');
            let known_patches = known_patches
                .split(',')
                .filter_map(|number| number.parse::<i32>().ok())
                .collect::<Vec<i32>>();
            find_possibilities(springs.chars(), vec![0], &known_patches)
        })
        .sum()
}

fn repeats(str: &str, sep: char) -> String {
    let mut result = "".to_string();
    for _ in 0..5 {
        result = result.to_owned() + str + sep.to_string().as_str();
    }
    result[..result.len() - 1].to_string()
}
