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

fn part1(input: &str) -> u64 {
    let grid = input
        .lines()
        .flat_map(|line| {
            let line_vec = line.chars().collect::<Vec<char>>();
            if line.contains('#') {
                vec![line_vec]
            } else {
                vec![line_vec.clone(), line_vec]
            }
        })
        .collect::<Vec<Vec<char>>>();

    let mut galaxies = vec![];
    let mut offset = 0;
    for x in 0..grid[0].len() {
        let mut has_galaxy = false;
        for y in 0..grid.len() {
            if grid[y][x] == '#' {
                galaxies.push((x + offset, y));
                has_galaxy = true;
            };
        }
        if !has_galaxy {
            offset += 1;
        }
    }
    // dbg!(galaxies.clone());
    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let galaxy1 = galaxies[i];
            let galaxy2 = galaxies[j];
            sum += udiff(galaxy1.0, galaxy2.0) + udiff(galaxy1.1, galaxy2.1)
        }
    }
    sum as u64
}

fn udiff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn part2(input: &str) -> u64 {
    let mut offsets_y = vec![0; input.lines().collect::<Vec<_>>().len()];
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            if !line.contains('#') {
                for i in y..offsets_y.len() {
                    offsets_y[i] += 999999;
                }
            };
            line.chars().collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    let mut galaxies = vec![];
    let mut offset = 0;
    for x in 0..grid[0].len() {
        let mut has_galaxy = false;
        for y in 0..grid.len() {
            if grid[y][x] == '#' {
                galaxies.push((x + offset, y + offsets_y[y]));
                has_galaxy = true;
            };
        }
        if !has_galaxy {
            offset += 999999;
        }
    }
    // dbg!(galaxies.clone());
    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let galaxy1 = galaxies[i];
            let galaxy2 = galaxies[j];
            sum += udiff(galaxy1.0, galaxy2.0) + udiff(galaxy1.1, galaxy2.1)
        }
    }
    sum as u64
}
