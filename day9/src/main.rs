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

fn extrapolate(history: &str) -> i64 {
    // dbg!(history);
    let mut result = 0;
    let mut history: Vec<i64> = history
        .split_whitespace()
        .filter_map(|value| value.parse::<i64>().ok())
        .collect();

    while !history.iter().all(|x| *x == 0) {
        // dbg!(history.clone());
        result += history.last().unwrap();
        history = history.windows(2).map(|pair| pair[1] - pair[0]).collect();
    }

    // dbg!(result);
    result
}

fn extrapolate_backwards(history: &str) -> i64 {
    // dbg!(history);
    let mut history: Vec<i64> = history
        .split_whitespace()
        .filter_map(|value| value.parse::<i64>().ok())
        .collect();

    let mut results = vec![];
    while !history.iter().all(|x| *x == 0) {
        results.push(history[0]);
        history = history.windows(2).map(|pair| pair[1] - pair[0]).collect();
    }
    results.into_iter().rev().reduce(|acc, x| x - acc).unwrap()
}
fn part1(input: &str) -> i64 {
    input.lines().map(extrapolate).sum()
}

fn part2(input: &str) -> i64 {
    input.lines().map(extrapolate_backwards).sum()
}
