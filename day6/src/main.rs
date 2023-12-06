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

fn part1(input: &str) -> f64 {
    let mut lines = input.lines();
    let times: Vec<f64> = (lines.next().unwrap())[5..]
        .split_whitespace()
        .map(|time| time.parse().unwrap())
        .collect();
    let durations: Vec<f64> = (lines.next().unwrap())[9..]
        .split_whitespace()
        .map(|time| time.parse().unwrap())
        .collect();
    times
        .into_iter()
        .zip(durations.into_iter())
        .map(|(time, duration)| {
            let mut breakpoint1 = time / 2. + ((time / 2.).powf(2.) - duration).sqrt();
            let mut breakpoint2 = time / 2. - ((time / 2.).powf(2.) - duration).sqrt();
            breakpoint1 = if breakpoint1 == breakpoint1.floor() {
                breakpoint1
            } else {
                breakpoint1.ceil()
            };
            breakpoint2 = if breakpoint2 == breakpoint2.ceil() {
                breakpoint2 + 1.
            } else {
                breakpoint2.ceil()
            };
            (breakpoint2.floor() - breakpoint1.ceil()).abs()
        })
        .product()
}

fn part2(input: &str) -> f64 {
    let mut lines = input.lines();
    let time = (lines.next().unwrap())[5..]
        .chars()
        .filter_map(|char| {
            if char.is_digit(10) {
                Some(char.to_digit(10).unwrap() as u64)
            } else {
                None
            }
        })
        .fold(0, |acc, x| acc * 10 + x) as f64;
    let duration = (lines.next().unwrap())[9..]
        .chars()
        .filter_map(|char| {
            if char.is_digit(10) {
                Some(char.to_digit(10).unwrap() as u64)
            } else {
                None
            }
        })
        .fold(0, |acc, x| acc * 10 + x) as f64;
    let mut breakpoint1 = time / 2. + ((time / 2.).powf(2.) - duration).sqrt();
    let mut breakpoint2 = time / 2. - ((time / 2.).powf(2.) - duration).sqrt();
    breakpoint1 = if breakpoint1 == breakpoint1.floor() {
        breakpoint1
    } else {
        breakpoint1.ceil()
    };
    breakpoint2 = if breakpoint2 == breakpoint2.ceil() {
        breakpoint2 + 1.
    } else {
        breakpoint2.ceil()
    };
    (breakpoint2.floor() - breakpoint1.ceil()).abs()
}
