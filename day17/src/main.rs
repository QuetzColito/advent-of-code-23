use std::cmp::Reverse;
use std::collections::HashMap;
use std::{collections::BinaryHeap, time::Instant};

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
    println!("{}", part2(input));

    let end_time = Instant::now();
    println!("{:?}", end_time - start_time);
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd)]
struct Crucible {
    x: isize,
    y: isize,
    streak: u64,
    time: u64,
    direction: Direction,
}

impl Ord for Crucible {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.cmp(&other.time)
    }
}

impl Crucible {
    fn get_nexts(&self, grid: &Grid) -> Vec<Crucible> {
        let mut crucible1 = self.clone();
        let mut crucible2 = self.clone();
        let mut crucible3 = self.clone();

        crucible2.streak += 1;

        match self.direction {
            Direction::North => {
                crucible1.x -= 1;
                crucible1.direction = Direction::West;
                crucible2.y -= 1;
                crucible3.direction = Direction::East;
                crucible3.x += 1;
            }
            Direction::East => {
                crucible1.y -= 1;
                crucible1.direction = Direction::North;
                crucible2.x += 1;
                crucible3.direction = Direction::South;
                crucible3.y += 1;
            }
            Direction::South => {
                crucible1.x -= 1;
                crucible1.direction = Direction::West;
                crucible2.y += 1;
                crucible3.direction = Direction::East;
                crucible3.x += 1;
            }
            Direction::West => {
                crucible1.y -= 1;
                crucible1.direction = Direction::North;
                crucible2.x -= 1;
                crucible3.direction = Direction::South;
                crucible3.y += 1;
            }
        };

        vec![crucible1, crucible2, crucible3]
            .into_iter()
            .filter(|crucible| crucible.is_in_bounds(grid) && crucible.still_heated())
            .map(|crucible| crucible.add_time(grid))
            .collect()
    }

    fn add_time(mut self, grid: &Grid) -> Self {
        self.time += grid.get(&self);
        self
    }

    fn still_heated(&self) -> bool {
        self.streak < 4
    }

    fn is_in_bounds(&self, grid: &Grid) -> bool {
        self.y >= 0
            && self.x >= 0
            && self.x < grid.tiles[0].len() as isize
            && self.y < grid.tiles.len() as isize
    }

    fn on_new_path(&self, grid: &Grid) -> bool {
        !grid.seen_tiles[self.y as usize][self.x as usize]
    }

    fn has_arrived(&self, grid: &Grid) -> bool {
        self.y == grid.tiles.len() as isize - 1 && self.x == grid.tiles[0].len() as isize - 1
    }
}

struct Grid {
    tiles: Vec<Vec<u64>>,
    crucibles: Vec<Crucible>,
    seen_tiles: Vec<Vec<bool>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let tiles: Vec<Vec<u64>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .filter_map(|c| c.to_digit(10))
                    .map(|x| x as u64)
                    .collect()
            })
            .collect();
        let seen_tiles: Vec<Vec<bool>> = vec![vec![false; tiles[0].len()]; tiles.len()];
        let crucibles: Vec<Crucible> = vec![];
        Grid {
            tiles,
            crucibles,
            seen_tiles,
        }
    }

    fn get(&self, crucible: &Crucible) -> u64 {
        self.tiles[crucible.y as usize][crucible.x as usize]
    }

    fn mark_tiles(&mut self, crucible: &Crucible) {
        self.seen_tiles[crucible.y as usize][crucible.x as usize] = true;
    }

    fn fastest_route(&mut self, start: Crucible) -> u64 {
        let mut seen: HashMap<Crucible, u32> = HashMap::new();
        let mut pq: BinaryHeap<(Reverse<u32>, Crucible)> = BinaryHeap::new();
        pq.push((Reverse(0), start.clone()));

        let mut time = 0;
        while let Some((Reverse(dist), crucible)) = pq.pop() {
            dbg!(crucible.clone());
            if seen.contains_key(&crucible) {
                continue;
            }

            let hx = self.get(&crucible) as u32;
            seen.insert(crucible.clone(), dist + hx);
            if crucible.has_arrived(self) {
                time = (dist + hx) as u64;
                break;
            }
            let next_crucibles = crucible.get_nexts(self);
            for c in next_crucibles {
                if !seen.contains_key(&c) {
                    pq.push((Reverse(dist + hx), c))
                }
            }
        }
        time
    }
}

fn part1(input: &str) -> u64 {
    let mut grid = Grid::new(input);
    grid.fastest_route(Crucible {
        x: 0,
        y: 0,
        streak: 0,
        time: 0,
        direction: Direction::East,
    })
}

fn part2(input: &str) -> u64 {
    2
}
