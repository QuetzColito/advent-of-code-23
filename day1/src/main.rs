fn main() {
    let s = include_str!("input.txt");
    let data: Vec<(&str, u32)> = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let o: u32 = s
        .lines()
        .map(|x| first_number(&data, x) * 10 + last_number(&data, x))
        .sum();
    println!("{o:?}");
}

fn first_number(data: &Vec<(&str, u32)>, s: &str) -> u32 {
    if let Some(first) = s.chars().next().unwrap_or('0').to_digit(10) {
        return first;
    };
    for (n, i) in data {
        if s.starts_with(*n) {
            return *i;
        }
    }
    first_number(data, &s[1..])
}

fn last_number(data: &Vec<(&str, u32)>, s: &str) -> u32 {
    if let Some(last) = s.chars().rev().next().unwrap_or('0').to_digit(10) {
        return last;
    };
    for (n, i) in data {
        if s.ends_with(*n) {
            return *i;
        }
    }
    let len = s.len();
    last_number(data, &s[..len - 1])
}
