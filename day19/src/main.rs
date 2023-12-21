use std::{collections::HashMap, time::Instant};

fn main() {
    let start_time = Instant::now();

    let input = include_str!("input.txt");
    let test = include_str!("test.txt");
    println!("{}", part1(test, true));
    println!("{}", part1(input, false));
    let end_time = Instant::now();
    println!("{:?}", end_time - start_time);
    dbg!(encode("in"));
    dbg!(encode("R"));
    dbg!(encode("A"));

    let start_time = Instant::now();
    let input = include_str!("input.txt");
    let test = include_str!("test.txt");
    println!("{}", part2(test));
    println!("{}", part2(input));

    let end_time = Instant::now();
    println!("{:?}", end_time - start_time);
}

const accept: u64 = 10;
const reject: u64 = 27;
const start: u64 = 671;

struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

impl Part {
    fn new(part: &str) -> Self {
        let categories: Vec<i32> = part
            .split(',')
            .filter_map(|cat| cat[2..].parse().ok())
            .collect();
        Part {
            x: categories[0],
            m: categories[1],
            a: categories[2],
            s: categories[3],
        }
    }

    fn sum(&self) -> u64 {
        (self.x + self.m + self.a + self.s) as u64
    }
}

struct Workflow {
    category: char,
    operator: char,
    target: i32,
    next: u64,
}

impl Workflow {
    fn new(workflow: &str) -> Self {
        if let Some((chars, next)) = workflow.split_once(':') {
            let mut chars = chars.chars();
            let category = chars.next().unwrap();
            let operator = chars.next().unwrap();
            let target = chars.collect::<String>().parse().unwrap();
            Workflow {
                category,
                operator,
                target,
                next: encode(next),
            }
        } else {
            Workflow {
                category: 'x',
                operator: '<',
                target: i32::MAX,
                next: encode(workflow),
            }
        }
    }
}

fn encode(workflow: &str) -> u64 {
    workflow
        .chars()
        .filter_map(|x| x.to_digit(36))
        .fold(0, |acc, x| acc * 36 + x as u64)
}

fn evaluate(category: char, operator: char, target: i32, part: &Part) -> bool {
    match category {
        'x' => compare(operator, part.x, target),
        'm' => compare(operator, part.m, target),
        'a' => compare(operator, part.a, target),
        's' => compare(operator, part.s, target),
        _ => unimplemented!(),
    }
}

fn compare(operator: char, x1: i32, x2: i32) -> bool {
    match operator {
        '>' => x1 > x2,
        '<' => x1 < x2,
        _ => unreachable!(),
    }
}

fn accepted(workflows: &HashMap<u64, Vec<Workflow>>, current: u64, part: &Part) -> bool {
    for workflow in workflows.get(&current).unwrap() {
        if evaluate(workflow.category, workflow.operator, workflow.target, part) {
            return if workflow.next == reject {
                false
            } else if workflow.next == accept {
                true
            } else {
                accepted(workflows, workflow.next, part)
            };
        }
    }
    false
}

fn part1(input: &str, test: bool) -> u64 {
    // dbg!(input);
    let (workflows, parts) = if test {
        input.split_once("\r\n\r\n").unwrap()
    } else {
        input.split_once("\n\n").unwrap()
    };
    let workflows: HashMap<u64, Vec<Workflow>> = workflows
        .lines()
        .map(|line| {
            let (node, workflow) = line.split_once('{').unwrap();
            let workflow = workflow[..(workflow.len() - 1)]
                .split(',')
                .map(|wf| {
                    // dbg!(wf);
                    Workflow::new(wf)
                })
                .collect();
            (encode(node), workflow)
        })
        .collect();
    let parts: Vec<Part> = parts
        .lines()
        .map(|line| Part::new(&line[1..(line.len() - 1)]))
        .collect();

    parts
        .into_iter()
        .map(|part| {
            if accepted(&workflows, start, &part) {
                part.sum()
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    2
}
