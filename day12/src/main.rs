use std::{collections::HashMap, str::Chars, time::Instant};

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

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct State {
    last_char: char,
    signature: Vec<usize>,
}

fn find_possibilities(springs: Chars, known_patches: &Vec<usize>) -> u64 {
    let state_0 = State {
        last_char: '.',
        signature: vec![],
    };
    let mut result: HashMap<State, usize> = HashMap::from([(state_0, 1 as usize)]);

    for c in springs {
        result = match c {
            '#' => addh(&result),
            '.' => addp(&result),
            '?' => addh(&result)
                .into_iter()
                .chain(addp(&result).into_iter())
                .collect(),
            _ => unreachable!(),
        };
        result = result
            .into_iter()
            .filter(|(state, _)| is_admissable(state, known_patches))
            .collect()
    }
    result
        .iter()
        .filter_map(|(state, amount)| {
            if state.signature == *known_patches {
                Some(*amount as u64)
            } else {
                None
            }
        })
        .sum()
}

fn is_admissable(state: &State, known_patches: &Vec<usize>) -> bool {
    let signature = state.signature.clone();
    let k = signature.len();
    if k == 0 {
        return true;
    }

    k <= known_patches.len()
        && signature[..k - 1] == known_patches[..k - 1]
        && signature[k - 1] <= known_patches[k - 1]
}

fn addh(result: &HashMap<State, usize>) -> HashMap<State, usize> {
    let mut new_result: HashMap<State, usize> = HashMap::new();
    for (state, amount) in result.iter() {
        let mut signature = state.signature.clone();
        match state.last_char {
            '.' => signature.push(1),
            '#' => {
                let tmp = signature.pop().unwrap();
                signature.push(tmp + 1)
            }
            _ => unreachable!(),
        }
        *new_result
            .entry(State {
                last_char: '#',
                signature,
            })
            .or_default() += amount;
    }

    new_result
}

fn addp(result: &HashMap<State, usize>) -> HashMap<State, usize> {
    let mut new_result: HashMap<State, usize> = HashMap::new();
    for (state, amount) in result.iter() {
        *new_result
            .entry(State {
                last_char: '.',
                signature: state.signature.clone(),
            })
            .or_default() += amount;
    }
    new_result
}

// fn spring_matches(springs: &str, known_patches: &Vec<u64>) -> bool {
//     springs
//         .split_terminator('.')
//         .filter_map(|str| {
//             let length = str.len();
//             if length == 0 {
//                 None
//             } else {
//                 Some(length as u64)
//             }
//         })
//         .collect::<Vec<u64>>()
//         == *known_patches
// }

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let (springs, known_patches) = line.split_once(' ').unwrap();
            let known_patches = known_patches
                .split(',')
                .filter_map(|number| number.parse::<usize>().ok())
                .collect::<Vec<usize>>();
            // dbg!(known_patches.clone());
            find_possibilities(springs.chars(), &known_patches)
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let (springs, known_patches) = line.split_once(' ').unwrap();
            let springs = repeats(springs, '?');
            let known_patches = repeats(known_patches, ',');
            let known_patches = known_patches
                .split(',')
                .filter_map(|number| number.parse::<usize>().ok())
                .collect::<Vec<usize>>();
            find_possibilities(springs.chars(), &known_patches)
        })
        .sum()
}

fn repeats(str: &str, sep: char) -> String {
    let mut result = "".to_string();
    for _ in 0..5 {
        result = result.to_owned() + str + sep.to_string().as_str();
    }
    result[..result.len() - 1].to_string()
}
