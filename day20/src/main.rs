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

fn part1(input: &str) -> u64 {
    let mut broadcaster = vec![];
    let mut flip_flops: Vec<(&str, Vec<&str>)> = vec![];
    let mut conjunctions: Vec<(&str, Vec<&str>)> = vec![];
    for line in input.lines() {
        if line.starts_with("broadcaster") {
            broadcaster = line
                .split_once("->")
                .unwrap()
                .1
                .split(',')
                .map(|x| x.trim())
                .collect()
        } else if line.starts_with("%") {
            let (source, destination) = line[1..].split_once("->").unwrap();
            flip_flops.push((
                source.trim(),
                destination.split(',').map(|x| x.trim()).collect(),
            ));
        } else if line.starts_with("&") {
            let (source, destination) = line[1..].split_once("->").unwrap();
            conjunctions.push((
                source.trim(),
                destination.split(',').map(|x| x.trim()).collect(),
            ));
        }
    }
    1
}

fn part2(input: &str) -> u64 {
    2
}
