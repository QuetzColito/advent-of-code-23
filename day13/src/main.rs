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
    input
        .split("\r\n\r\n")
        .map(|block| {
            // dbg!(block.clone());
            let horizontal = horizontal_mirror(block, 0);
            if horizontal > 0 {
                horizontal
            } else {
                vertical_mirror(block, 0)
            }
        })
        .sum()
}

fn value(c: char) -> u64 {
    match c {
        '#' => 1,
        _ => 0,
    }
}

fn find_reflection(rows: Vec<u64>, tolerance: u64) -> u64 {
    // dbg!(rows.clone());
    rows.iter().enumerate().fold(0, |acc, (index, current)| {
        let upper = index / 2 + 1;
        let lower = rows.len() - (rows.len() - index) / 2;
        // dbg!(index, upper, lower);

        if index % 2 == 1
            && diff(&vec![rows[0]], &vec![*current], tolerance)
            && rows[..upper]
                == rows[upper..index + 1]
                    .to_vec()
                    .clone()
                    .into_iter()
                    .rev()
                    .collect::<Vec<u64>>()
        {
            // dbg!(index, upper);
            upper as u64
        } else if index % 2 == rows.len() % 2
            && diff(&vec![*rows.last().unwrap()], &vec![*current], tolerance)
            && diff(
                &rows[lower..].to_vec(),
                &rows[index..lower].to_vec(),
                tolerance,
            )
        {
            // dbg!(index, lower);
            lower as u64
        } else {
            acc
        }
    })
}

fn diff(first: &Vec<u64>, second: &Vec<u64>, mut tolerance: u64) -> bool {
    let second: Vec<u64> = second.clone().into_iter().rev().collect();
    let result = second.iter().zip(first.iter()).fold(true, |acc, (x, y)| {
        let d = (*x as i64 - *y as i64).abs() as u64;
        if d == 0 {
            acc && true
        } else if tolerance > 0 && (d & (d - 1)) == 0 {
            tolerance -= 1;
            acc && true
        } else {
            false
        }
    });

    if tolerance == 0 {
        result
    } else {
        false
    }
}

fn horizontal_mirror(block: &str, tolerance: u64) -> u64 {
    find_reflection(
        block
            .lines()
            .map(|line| line.chars().fold(0, |acc, c| acc * 2 + value(c)))
            .collect(),
        tolerance,
    ) * 100
}

fn vertical_mirror(block: &str, tolerance: u64) -> u64 {
    find_reflection(
        block.lines().fold(
            vec![0; block.lines().last().unwrap_or_default().len()],
            |acc, line| {
                acc.into_iter()
                    .zip(line.chars())
                    .map(|(old, new)| old * 2 + value(new))
                    .collect()
            },
        ),
        tolerance,
    )
}

fn part2(input: &str) -> u64 {
    input
        .split("\r\n\r\n")
        .map(|block| {
            // dbg!(block.clone());
            let horizontal = horizontal_mirror(block, 1);
            if horizontal > 0 {
                horizontal
            } else {
                vertical_mirror(block, 1)
            }
        })
        .sum()
}
