use std::time::Instant;

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
    println!("{}", part2(test));
    // println!("{}", part2(input));

    let end_time = Instant::now();
    println!("{:?}", end_time - start_time);
}

fn part1(input: &str) -> u64 {
    let height = input.lines().count();
    let length = input.lines().next().unwrap().len();
    let mut last_stones = vec![0; length];
    input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => 0,
                    '#' => {
                        last_stones[x] = y + 1;
                        0
                    }
                    'O' => {
                        last_stones[x] += 1;
                        (height - (last_stones[x] - 1)) as u64
                    }
                    _ => {
                        dbg!(c);
                        unreachable!()
                    }
                })
                .sum::<u64>()
        })
        // .inspect(|x| println!("{x}"))
        .sum()
}

fn print(grid: &Vec<Vec<Rock>>) {
    for vec in grid {
        println!("{:?}", vec.iter().map(rock_to_char).collect::<String>())
    }
    println!(" - ")
}

fn rock_to_char(rock: &Rock) -> char {
    match rock {
        Rock::Cube => '#',
        Rock::Round => 'O',
        Rock::Empty => '.',
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Rock {
    Cube,
    Round,
    Empty,
}

const NORTH: u32 = 0;
const WEST: u32 = 1;
const SOUTH: u32 = 2;
const EAST: u32 = 3;

fn part2(input: &str) -> u64 {
    let mut grid: Vec<Vec<Rock>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| match x {
                    '.' => Rock::Empty,
                    '#' => Rock::Cube,
                    'O' => Rock::Round,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    let mut grids = vec![];
    let iteration_amount = 1000000000;
    for iteration in 0..iteration_amount {
        for direction in 0..4 {
            match direction {
                NORTH => {
                    let mut last_stones = vec![0; grid[0].len()];
                    for y in 0..grid.len() {
                        for x in 0..grid[0].len() {
                            match grid[y][x] {
                                Rock::Empty => {}
                                Rock::Cube => {
                                    last_stones[x] = y + 1;
                                }
                                Rock::Round => {
                                    grid[y][x] = Rock::Empty;
                                    grid[last_stones[x]][x] = Rock::Round;
                                    last_stones[x] += 1;
                                }
                            }
                        }
                    }
                }
                WEST => {
                    let mut last_stones = vec![0; grid.len()];
                    for x in 0..grid[0].len() {
                        for y in 0..grid.len() {
                            match grid[y][x] {
                                Rock::Empty => {}
                                Rock::Cube => {
                                    last_stones[y] = x + 1;
                                }
                                Rock::Round => {
                                    grid[y][x] = Rock::Empty;
                                    grid[y][last_stones[y]] = Rock::Round;
                                    last_stones[y] += 1;
                                }
                            }
                        }
                    }
                }
                SOUTH => {
                    let mut last_stones = vec![grid.len(); grid[0].len()];
                    for y in (0..grid.len()).rev() {
                        for x in 0..grid[0].len() {
                            // dbg!(x, y, last_stones[x]);
                            match grid[y][x] {
                                Rock::Empty => {}
                                Rock::Cube => {
                                    last_stones[x] = y;
                                }
                                Rock::Round => {
                                    grid[y][x] = Rock::Empty;
                                    last_stones[x] -= 1;
                                    grid[last_stones[x]][x] = Rock::Round;
                                }
                            }
                        }
                    }
                }
                EAST => {
                    let mut last_stones = vec![grid[0].len(); grid.len()];
                    for x in (0..grid[0].len()).rev() {
                        for y in 0..grid.len() {
                            match grid[y][x] {
                                Rock::Empty => {}
                                Rock::Cube => {
                                    last_stones[y] = x;
                                }
                                Rock::Round => {
                                    grid[y][x] = Rock::Empty;
                                    last_stones[y] -= 1;
                                    grid[y][last_stones[y]] = Rock::Round;
                                }
                            }
                        }
                    }
                }
                _ => unreachable!(),
            };
        }
        print(&grid);
        if grids.contains(&grid) {
            let index = grids.iter().position(|x| *x == grid).unwrap();
            let pattern_length = grids.len() - index;
            let iterations_left = iteration_amount - iteration;
            dbg!(index, iteration, pattern_length, iterations_left);
            grids.push(grids[(iterations_left % pattern_length) + index].clone());
            break;
        } else {
            grids.push(grid.clone())
        }
    }
    grid = grids.into_iter().last().unwrap();
    let height = grid.len();
    let length = grid[0].len();
    let mut last_stones = vec![0; length];
    grid.into_iter()
        .enumerate()
        .map(|(y, line)| {
            line.into_iter()
                .enumerate()
                .map(|(x, c)| match c {
                    Rock::Empty => 0,
                    Rock::Cube => {
                        last_stones[x] = y + 1;
                        0
                    }
                    Rock::Round => {
                        last_stones[x] += 1;
                        (height - (last_stones[x] - 1)) as u64
                    }
                })
                .sum::<u64>()
        })
        // .inspect(|x| println!("{x}"))
        .sum()
}
