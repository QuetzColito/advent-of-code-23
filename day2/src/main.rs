fn main() {
    let s = include_str!("input.txt");
    println!("{}", check_games(s));
    let s2 = include_str!("input.txt");
    println!("{}", check_games2(s2));
}

fn check_games(s: &str) -> u32 {
    s.lines()
        .filter_map(|x| {
            let (id, game) = x.split_once(':').unwrap();
            if is_valid(game) {
                Some(to_number(id))
            } else {
                None
            }
        })
        .sum()
}

fn check_games2(s: &str) -> u32 {
    s.lines()
        .map(|x| {
            let (_, game) = x.split_once(':').unwrap();
            is_valid2(game)
        })
        .sum()
}

fn is_valid2(s: &str) -> u32 {
    let mut reds = vec![0];
    let mut greens = vec![0];
    let mut blues = vec![0];
    s.split(';').for_each(|roll| {
        roll.split(',').for_each(|colour| {
            if colour.contains("red") {
                reds.push(to_number(colour));
            } else if colour.contains("green") {
                greens.push(to_number(colour));
            } else if colour.contains("blue") {
                blues.push(to_number(colour));
            }
        })
    });
    reds.into_iter().max().unwrap()
        * greens.into_iter().max().unwrap()
        * blues.into_iter().max().unwrap()
}

fn is_valid(s: &str) -> bool {
    s.split(';').all(|roll| {
        roll.split(',').all(|colour| {
            if colour.contains("red") {
                to_number(colour) <= 12
            } else if colour.contains("green") {
                to_number(colour) <= 13
            } else if colour.contains("blue") {
                to_number(colour) <= 14
            } else {
                false
            }
        })
    })
}

fn to_number(s: &str) -> u32 {
    s.chars()
        .filter_map(|x| x.to_digit(10))
        .fold(0, |acc, digit| acc * 10 + digit)
}
