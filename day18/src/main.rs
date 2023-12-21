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

fn part1(input: &str) -> i32 {
    let mut x = 0;
    let mut y = 0;
    input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            let direction = split.next().expect("line is not of supported format");
            let amount: i32 = split.next().unwrap().parse::<i32>().unwrap();
            let x2 = x;
            let y2 = y;
            match direction {
                "R" => {
                    x += amount;
                }
                "L" => {
                    x -= amount;
                }
                "D" => {
                    y += amount;
                }
                "U" => {
                    y -= amount;
                }
                _ => {
                    dbg!(direction);
                    unreachable!()
                }
            }

            (x2 - x) * (y2 + y) + amount
        })
        .sum::<i32>()
        / 2
        + 1
}

fn part2(input: &str) -> i64 {
    let mut x = 0;
    let mut y = 0;
    input
        .lines()
        .map(|line| {
            let split = &line.split_whitespace().last().unwrap()[2..8];
            let direction = split.chars().last().unwrap();
            let amount: i64 = split[0..5]
                .chars()
                .map(|x| x.to_digit(16).unwrap() as i64)
                .fold(0, |acc, x| acc * 16 + x);
            // dbg!(direction, amount);
            let x2 = x;
            let y2 = y;
            match direction {
                '0' => {
                    x += amount;
                }
                '2' => {
                    x -= amount;
                }
                '1' => {
                    y += amount;
                }
                '3' => {
                    y -= amount;
                }
                _ => {
                    dbg!(direction);
                    unreachable!()
                }
            }

            (x2 - x) * (y2 + y) + amount
        })
        .sum::<i64>()
        / 2
        + 1
}
