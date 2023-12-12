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

fn find_possibilities(springs: &str, known_patches: &Vec<u64>) -> u64 {
    if springs.contains("?") {
        find_possibilities(springs.replacen("?", "#", 1).as_str(), known_patches)
            + find_possibilities(springs.replacen("?", ".", 1).as_str(), known_patches)
    } else if spring_matches(springs, known_patches) {
        1
    } else {
        0
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

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let (springs, known_patches) = line.split_once(' ').unwrap();
            let known_patches = known_patches
                .split(',')
                .filter_map(|number| number.parse::<u64>().ok())
                .collect::<Vec<u64>>();
            find_possibilities(springs, &known_patches)
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let (springs, known_patches) = line.split_once(' ').unwrap();
            let springs = repeats(springs, '?');
            let known_patches = repeats(known_patches, ',');
            let known_patches = known_patches
                .split(',')
                .filter_map(|number| number.parse::<u64>().ok())
                .collect::<Vec<u64>>();
            find_possibilities(springs.as_str(), &known_patches)
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
