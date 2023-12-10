use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

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

#[derive(PartialEq, Debug, Eq, Hash, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn x_add(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn y_add(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn x_sub(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn y_sub(&self) -> Point {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }
}

fn part1(input: &str) -> u64 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut last = Point { x: 0, y: 0 };
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if get(&grid, &Point { x, y }) == 'S' {
                last = Point { x, y };
            };
        }
    }

    // dbg!(&grid);
    // dbg!(&last);

    let mut current = if ['|', '7', 'F'].contains(&get(&grid, &last.y_sub())) {
        last.y_sub()
    } else if ['-', '7', 'J'].contains(&get(&grid, &last.x_add())) {
        last.x_add()
    } else {
        last.y_add()
    };

    let mut pipe = vec!['S'];

    while get(&grid, &current) != 'S' {
        // dbg!(&current);
        pipe.push(get(&grid, &current));
        let next = next_coord(&grid, last, &current);
        last = current;
        current = next;
    }

    pipe.len() as u64 / 2
}

fn get(grid: &Vec<Vec<char>>, p: &Point) -> char {
    grid[p.y][p.x]
}

fn next_coord(grid: &Vec<Vec<char>>, last: Point, current: &Point) -> Point {
    let current_char = get(grid, &current);

    let (next1, next2) = match current_char {
        '|' => (current.y_sub(), current.y_add()),
        '-' => (current.x_sub(), current.x_add()),
        'L' => (current.y_sub(), current.x_add()),
        'J' => (current.y_sub(), current.x_sub()),
        '7' => (current.y_add(), current.x_sub()),
        'F' => (current.y_add(), current.x_add()),
        _ => unreachable!(),
    };

    if next1 == last {
        next2
    } else {
        next1
    }
}

fn determine_inside(direction: &mut i32, inside: bool, pipe_type: char) -> bool {
    if ['|'].contains(&pipe_type) {
        !inside
    } else if ['L', 'J'].contains(&pipe_type) {
        match direction {
            0 => {
                *direction = 1;
                !inside
            }
            1 => {
                *direction = 0;
                !inside
            }
            _ => {
                *direction = 0;
                inside
            }
        }
    } else if ['F', '7', 'S'].contains(&pipe_type) {
        match direction {
            0 => {
                *direction = -1;
                !inside
            }
            -1 => {
                *direction = 0;
                !inside
            }
            _ => {
                *direction = 0;
                inside
            }
        }
    } else {
        inside
    }
}

fn part2(input: &str) -> u64 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut last = Point { x: 0, y: 0 };
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if get(&grid, &Point { x, y }) == 'S' {
                last = Point { x, y };
            };
        }
    }

    // dbg!(&grid);
    // dbg!(&last);

    let mut current = if ['|', '7', 'F'].contains(&get(&grid, &last.y_sub())) {
        last.y_sub()
    } else if ['-', '7', 'J'].contains(&get(&grid, &last.x_add())) {
        last.x_add()
    } else {
        last.y_add()
    };

    let mut pipe = HashMap::from([(last.clone(), 'S')]);

    while get(&grid, &current) != 'S' {
        // dbg!(&current);
        pipe.insert(current.clone(), get(&grid, &current));
        let next = next_coord(&grid, last, &current);
        last = current;
        current = next;
    }
    // dbg!(pipe.clone());

    let mut count = 0;
    let mut direction = 0;
    for y in 0..grid.len() {
        let mut inside = false;
        for x in 0..grid[y].len() {
            if pipe.keys().any(|p| p.x == x && p.y == y) {
                // dbg!((x, y));
                inside = determine_inside(&mut direction, inside, get(&grid, &Point { x, y }))
            } else if inside {
                count += 1;
            };
        }
    }

    count
}
