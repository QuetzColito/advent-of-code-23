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

fn hash(str: &str) -> u64 {
    str.trim()
        .chars()
        .filter(|x| x.is_ascii())
        .fold(0, |acc, c| ((acc + (c as u8 as u64)) * 17) % 256)
}

fn part1(input: &str) -> u64 {
    input.split(",").map(hash).sum()
}

fn part2(input: &str) -> u64 {
    let mut boxes: Vec<Vec<(&str, u64)>> = vec![vec![]; 256];
    for command in input.split(',').map(|x| x.trim()) {
        if command.contains('-') {
            let command = &command[..(command.len() - 1)];
            boxes[hash(&command) as usize].retain(|x| x.0 != command)
        }
        if command.contains('=') {
            let strength = command.chars().last().unwrap().to_digit(10).unwrap() as u64;
            let command = &command[..(command.len() - 2)];
            let index = hash(&command) as usize;
            if let Some(position) = boxes[index].iter().position(|x| x.0 == command) {
                boxes[index][position] = (boxes[index][position].0, strength)
            } else {
                boxes[index].push((command, strength))
            }
        }
    }

    boxes
        .into_iter()
        .enumerate()
        .map(|(box_number, not_a_box)| {
            not_a_box
                .into_iter()
                .map(|x| x.1)
                .enumerate()
                .fold(0, |acc, (i, strength)| {
                    acc + (box_number as u64 + 1) * (i as u64 + 1) * strength
                })
        })
        .sum()
}
