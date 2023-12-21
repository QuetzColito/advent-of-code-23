use std::{collections::HashSet, time::Instant};

fn main() {
    let start_time = Instant::now();

    let input = include_str!("input.txt");
    let test = include_str!("test.txt");
    println!("{}", part1(test, 6));
    println!("{}", part1(input, 64));
    let end_time = Instant::now();
    println!("{:?}", end_time - start_time);

    let start_time = Instant::now();
    let input = include_str!("input.txt");
    let test = include_str!("test.txt");
    println!("{}", part2(test, 500));
    // println!("{}", part2(input, 26501365));

    let end_time = Instant::now();
    println!("{:?}", end_time - start_time);
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point {
            x: x as isize,
            y: y as isize,
        }
    }

    fn xu(&self) -> usize {
        self.x as usize
    }

    fn yu(&self) -> usize {
        self.y as usize
    }
}

fn modu(n: isize, m: isize) -> usize {
    (((n % m) + m) % m) as usize
}

fn neighbours(grid: &Vec<Vec<char>>, point: Point) -> Vec<Point> {
    let mut up = point.clone();
    let mut right = point.clone();
    let mut down = point.clone();
    let mut left = point.clone();

    up.y -= 1;
    right.x += 1;
    down.y += 1;
    left.x -= 1;

    vec![up, right, down, left]
        .into_iter()
        .filter(|next| in_bounds(grid, *next) && is_steppable(grid, *next))
        .collect()
}

fn neighbours2(grid: &Vec<Vec<char>>, point: Point) -> Vec<Point> {
    let mut up = point.clone();
    let mut right = point.clone();
    let mut down = point.clone();
    let mut left = point.clone();

    up.y -= 1;
    right.x += 1;
    down.y += 1;
    left.x -= 1;

    vec![up, right, down, left]
        .into_iter()
        .filter(|next| is_steppable(grid, *next))
        .collect()
}

fn get(grid: &Vec<Vec<char>>, point: Point) -> char {
    grid[modu(point.y, grid.len() as isize)][modu(point.x, grid[0].len() as isize)]
}

fn is_steppable(grid: &Vec<Vec<char>>, point: Point) -> bool {
    get(grid, point) == '.'
}

fn in_bounds(grid: &Vec<Vec<char>>, point: Point) -> bool {
    point.x >= 0 && point.y >= 0 && point.xu() < grid[0].len() && point.yu() < grid.len()
}

fn part1(input: &str, steps: u64) -> u64 {
    let mut start = Point::new(0, 0);
    let grid: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = Point::new(x, y);
                        '.'
                    } else {
                        c
                    }
                })
                .collect()
        })
        .collect();
    let mut destinations: HashSet<_> = std::iter::once(start).collect();
    for _ in 0..steps {
        destinations = destinations
            .into_iter()
            .flat_map(|point| neighbours(&grid, point))
            .collect()
    }
    destinations.len() as u64
}

fn part2(input: &str, steps: u64) -> u64 {
    let mut start = Point::new(0, 0);
    let grid: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = Point::new(x, y);
                        '.'
                    } else {
                        c
                    }
                })
                .collect()
        })
        .collect();
    let mut destinations: HashSet<_> = std::iter::once(start).collect();
    for _ in 0..steps {
        destinations = destinations
            .into_iter()
            .flat_map(|point| neighbours2(&grid, point))
            .collect()
    }
    destinations.len() as u64
}
