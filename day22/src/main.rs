use itertools::iproduct;
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

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq, PartialOrd)]
struct Brick {
    x: (usize, usize),
    y: (usize, usize),
    z: (usize, usize),
}

impl Brick {
    fn coords(&self) -> Vec<(usize, usize)> {
        iproduct!(self.x.0..=self.x.1, self.y.0..=self.y.1).collect()
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.z
            .0
            .cmp(&other.z.0)
            .then(self.y.0.cmp(&other.y.0))
            .then(self.x.0.cmp(&other.x.0))
    }
}

fn gravity(bricks: &Vec<Brick>) -> Vec<(Brick, bool)> {
    let mut current_height = vec![vec![0; 10 as usize]; 10 as usize];
    bricks
        .into_iter()
        .map(|brick| {
            let coords = brick.coords();
            let max_z = coords
                .iter()
                .map(|(x, y)| current_height[*y][*x])
                .max()
                .unwrap_or(0);
            // dbg!(max_z, brick.z);
            let z_diff = brick.z.0 - max_z - 1;

            let new_brick = Brick {
                x: brick.x,
                y: brick.y,
                z: (brick.z.0 - z_diff, brick.z.1 - z_diff),
            };

            for (x, y) in coords {
                current_height[y][x] = new_brick.z.1;
            }

            (new_brick, new_brick.z != brick.z)
        })
        .collect()
}

fn part1(input: &str) -> u64 {
    let mut bricks: Vec<Brick> = input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('~').unwrap();
            let start: Vec<usize> = start
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            let end: Vec<usize> = end
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            Brick {
                x: (start[0], end[0]),
                y: (start[1], end[1]),
                z: (start[2], end[2]),
            }
        })
        .collect();
    bricks.sort_unstable();

    let bricks: Vec<Brick> = gravity(&bricks).into_iter().map(|x| x.0).collect();

    let mut count = 0;
    for i in 0..bricks.len() {
        if gravity(
            &bricks
                .iter()
                .enumerate()
                .filter_map(|(j, b)| (i != j).then_some(*b))
                .collect(),
        )
        .iter()
        .all(|x| !x.1)
        {
            count += 1;
        }
    }
    // dbg!(supports.clone());
    // dbg!(supported_by.clone());
    count
}

fn part2(input: &str) -> u64 {
    2
}
