use std::{ops::Range, time::Instant};

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

#[derive(PartialEq, Copy, Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn mirror_up(&self) -> Self {
        let mut x = *self as u64;
        x += 1;
        if x % 2 == 0 {
            x -= 2;
        }
        x.into()
    }
    fn mirror_down(&self) -> Self {
        let mut x = *self as u64;
        x += 3;
        if x % 2 == 0 {
            x -= 2;
        }
        x.into()
    }
}

impl From<u64> for Direction {
    fn from(value: u64) -> Self {
        match value % 4 {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => panic!("not a valid direction"),
        }
    }
}

#[derive(Clone)]
enum Tile {
    Empty,
    SplitH,
    SplitV,
    MirrorDown,
    MirrorUp,
}

#[derive(PartialEq, Copy, Clone, Debug)]
struct Beam {
    x: isize,
    y: isize,
    direction: Direction,
}

impl Beam {
    fn set_direction(self, direction: Direction) -> Self {
        Beam {
            x: self.x,
            y: self.y,
            direction,
        }
    }
    fn x_add(self) -> Self {
        Beam {
            x: self.x + 1,
            y: self.y,
            direction: self.direction,
        }
    }

    fn y_add(self) -> Self {
        Beam {
            x: self.x,
            y: self.y + 1,
            direction: self.direction,
        }
    }

    fn x_sub(self) -> Self {
        Beam {
            x: self.x - 1,
            y: self.y,
            direction: self.direction,
        }
    }

    fn y_sub(self) -> Self {
        Beam {
            x: self.x,
            y: self.y - 1,
            direction: self.direction,
        }
    }
}

#[derive(Clone)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
    beams: Vec<Beam>,
    energized_tiles: Vec<Vec<Vec<Direction>>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let tiles: Vec<Vec<Tile>> = input
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| match c {
                        '|' => Tile::SplitV,
                        '-' => Tile::SplitH,
                        '\\' => Tile::MirrorDown,
                        '/' => Tile::MirrorUp,
                        '.' => Tile::Empty,
                        _ => {
                            dbg!(c);
                            unreachable!()
                        }
                    })
                    .collect()
            })
            .collect();
        let beams: Vec<Beam> = vec![];
        let energized_tiles: Vec<Vec<Vec<Direction>>> =
            vec![vec![vec![]; tiles[0].len()]; tiles.len()];
        Grid {
            tiles,
            beams,
            energized_tiles,
        }
    }

    fn get(&self, beam: Beam) -> &Tile {
        &self.tiles[beam.y as usize][beam.x as usize]
    }

    fn get_e(&self, beam: &Beam) -> &Vec<Direction> {
        &self.energized_tiles[beam.y as usize][beam.x as usize]
    }

    fn reset_energy(&mut self) {
        self.energized_tiles = vec![vec![vec![]; self.tiles[0].len()]; self.tiles.len()];
    }

    fn out_of_bounds(&self, beam: &Beam) -> bool {
        beam.x < 0
            || beam.x > self.tiles[0].len() as isize - 1
            || beam.y < 0
            || beam.y > self.tiles.len() as isize - 1
    }

    fn get_energy(mut self, beam: Beam) -> u64 {
        self.beams.push(beam);
        while !self.beams.is_empty() {
            // dbg!(self.beams.clone());
            let beams = self.beams.clone();
            self.beams = beams
                .into_iter()
                .flat_map(|beam| {
                    if self.out_of_bounds(&beam) {
                        return vec![];
                    }
                    match self.get(beam) {
                        Tile::SplitH => {
                            if beam.direction == Direction::South
                                || beam.direction == Direction::North
                            {
                                // dbg!(beam);
                                vec![
                                    beam.set_direction(Direction::West),
                                    beam.set_direction(Direction::East),
                                ]
                            } else {
                                vec![beam]
                            }
                        }
                        Tile::SplitV => {
                            if beam.direction == Direction::East
                                || beam.direction == Direction::West
                            {
                                // dbg!(beam);
                                vec![
                                    beam.set_direction(Direction::North),
                                    beam.set_direction(Direction::South),
                                ]
                            } else {
                                vec![beam]
                            }
                        }
                        Tile::MirrorDown => {
                            // dbg!(beam);
                            vec![beam.set_direction(beam.direction.mirror_down())]
                        }
                        Tile::MirrorUp => {
                            // dbg!(beam);
                            vec![beam.set_direction(beam.direction.mirror_up())]
                        }
                        _ => vec![beam],
                    }
                })
                .collect::<Vec<Beam>>()
                .into_iter()
                .filter_map(|beam| {
                    if self.get_e(&beam).contains(&beam.direction) {
                        None
                    } else {
                        self.energized_tiles[beam.y as usize][beam.x as usize].push(beam.direction);
                        Some(match beam.direction {
                            Direction::North => beam.y_sub(),
                            Direction::East => beam.x_add(),
                            Direction::South => beam.y_add(),
                            Direction::West => beam.x_sub(),
                        })
                    }
                })
                .collect();
        }
        // dbg!(self.energized_tiles.clone());
        let result = self
            .energized_tiles
            .iter()
            .flatten()
            .filter(|list| list.len() > 0)
            .count() as u64;
        self.reset_energy();
        result
    }
}

fn part1(input: &str) -> u64 {
    let grid = Grid::new(input);
    grid.get_energy(Beam {
        x: 0,
        y: 0,
        direction: Direction::East,
    })
}

fn yrange(ymax: isize) -> Range<isize> {
    0..ymax
}

fn xrange(xmax: isize) -> Range<isize> {
    0..xmax
}

fn part2(input: &str) -> u64 {
    let grid = Grid::new(input);
    let xmax = grid.tiles[0].len() as isize;
    let ymax = grid.tiles.len() as isize;
    yrange(ymax)
        .map(|y| Beam {
            x: 0,
            y,
            direction: Direction::East,
        })
        .chain(yrange(ymax).map(|y| Beam {
            x: xmax,
            y,
            direction: Direction::West,
        }))
        .chain(xrange(xmax).map(|x| Beam {
            x,
            y: 0,
            direction: Direction::South,
        }))
        .chain(xrange(xmax).map(|x| Beam {
            x,
            y: ymax,
            direction: Direction::North,
        }))
        .map(|beam| {
            let grid = grid.clone();
            grid.get_energy(beam)
        })
        .max()
        .unwrap()
}
