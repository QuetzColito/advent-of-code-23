use std::{
    collections::{HashMap, VecDeque},
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
    // let test = include_str!("test.txt");
    // println!("{}", part2(test));
    println!("{}", part2(input));

    let end_time = Instant::now();
    println!("{:?}", end_time - start_time);
}

type Pulse = (u64, bool);

trait Process {
    fn process(&mut self, pulse: Pulse) -> Vec<Pulse>;
}

struct NoneProcess;

impl Process for NoneProcess {
    fn process(&mut self, _pulse: Pulse) -> Vec<Pulse> {
        vec![]
    }
}

struct FlipFlop {
    state: bool,
    destinations: Vec<u64>,
}

impl FlipFlop {
    fn new(destinations: &str) -> Self {
        FlipFlop {
            state: false,
            destinations: destinations.split(',').map(|x| encode(x.trim())).collect(),
        }
    }
}

impl Process for FlipFlop {
    fn process(&mut self, pulse: Pulse) -> Vec<Pulse> {
        if pulse.1 {
            vec![]
        } else {
            self.state = !self.state;
            self.destinations.iter().map(|x| (*x, self.state)).collect()
        }
    }
}

struct Conjunction {
    state: HashMap<u64, bool>,
    destinations: Vec<u64>,
}

impl Conjunction {
    fn new(destinations: &str, modules: &Vec<u64>) -> Self {
        Conjunction {
            state: HashMap::from_iter(modules.iter().map(|x| (*x, false))),
            destinations: destinations.split(',').map(|x| encode(x.trim())).collect(),
        }
    }
}

impl Process for Conjunction {
    fn process(&mut self, pulse: Pulse) -> Vec<Pulse> {
        self.state.insert(pulse.0, pulse.1);
        let out = !(self.state.values().all(|x| *x));
        self.destinations.iter().map(|x| (*x, out)).collect()
    }
}

fn encode(input: &str) -> u64 {
    input
        .trim()
        .chars()
        .fold(0, |acc, x| x.to_digit(36).unwrap() as u64 + acc * 36)
}

fn part1(input: &str) -> u64 {
    let mut broadcaster = vec![];
    let mut nodes: HashMap<u64, Box<dyn Process>> = HashMap::new();
    let mut modules: HashMap<u64, Vec<u64>> = HashMap::new();
    for line in input.lines() {
        if !line.starts_with("broadcaster") {
            let (source, destination) = line[1..].split_once("->").unwrap();
            let source = encode(source.trim());
            let destinations: Vec<u64> = destination.split(',').map(|x| encode(x.trim())).collect();
            for d in destinations {
                modules.entry(d).or_insert(vec![]).push(source);
            }
        }
    }
    for line in input.lines() {
        if line.starts_with("broadcaster") {
            broadcaster = line
                .split_once("->")
                .unwrap()
                .1
                .split(',')
                .map(|x| (0, (encode(x.trim()), false)))
                .collect()
        } else if line.starts_with("%") {
            let (source, destination) = line[1..].split_once("->").unwrap();
            nodes.insert(encode(source.trim()), Box::new(FlipFlop::new(destination)));
        } else if line.starts_with("&") {
            let (source, destination) = line[1..].split_once("->").unwrap();
            let source = encode(source.trim());
            nodes.insert(
                source,
                Box::new(Conjunction::new(
                    destination,
                    &modules.get(&source).unwrap_or(&vec![]),
                )),
            );
        }
    }
    let mut lows = 0;
    let mut highs = 0;
    let mut none: Box<dyn Process> = Box::new(NoneProcess);
    for _ in 0..1000 {
        lows += 1;
        let mut pulses: VecDeque<(u64, Pulse)> = VecDeque::from(broadcaster.clone());
        while let Some((origin, pulse)) = pulses.pop_front() {
            // dbg!(origin, pulse);
            if pulse.1 {
                highs += 1;
            } else {
                lows += 1;
            }

            for new_pulse in nodes
                .get_mut(&pulse.0)
                .unwrap_or(&mut none)
                .process((origin, pulse.1))
            {
                // dbg!(new_pulse);
                pulses.push_back((pulse.0, new_pulse));
            }
        }
    }
    lows * highs
}

fn part2(input: &str) -> u64 {
    let mut broadcaster = vec![];
    let mut nodes: HashMap<u64, Box<dyn Process>> = HashMap::new();
    let mut modules: HashMap<u64, Vec<u64>> = HashMap::new();
    for line in input.lines() {
        if !line.starts_with("broadcaster") {
            let (source, destination) = line[1..].split_once("->").unwrap();
            let source = encode(source.trim());
            let destinations: Vec<u64> = destination.split(',').map(|x| encode(x.trim())).collect();
            for d in destinations {
                modules.entry(d).or_insert(vec![]).push(source);
            }
        }
    }
    for line in input.lines() {
        if line.starts_with("broadcaster") {
            broadcaster = line
                .split_once("->")
                .unwrap()
                .1
                .split(',')
                .map(|x| (0, (encode(x.trim()), false)))
                .collect()
        } else if line.starts_with("%") {
            let (source, destination) = line[1..].split_once("->").unwrap();
            nodes.insert(encode(source.trim()), Box::new(FlipFlop::new(destination)));
        } else if line.starts_with("&") {
            let (source, destination) = line[1..].split_once("->").unwrap();
            let source = encode(source.trim());
            nodes.insert(
                source,
                Box::new(Conjunction::new(
                    destination,
                    &modules.get(&source).unwrap_or(&vec![]),
                )),
            );
        }
    }
    let mut none: Box<dyn Process> = Box::new(NoneProcess);
    let mut counter = 0;
    'outer: loop {
        counter += 1;
        let mut pulses: VecDeque<(u64, Pulse)> = VecDeque::from(broadcaster.clone());
        while let Some((origin, pulse)) = pulses.pop_front() {
            // dbg!(origin, pulse);
            if pulse.0 == 1005 && !pulse.1 {
                break 'outer counter;
            }
            for new_pulse in nodes
                .get_mut(&pulse.0)
                .unwrap_or(&mut none)
                .process((origin, pulse.1))
            {
                // dbg!(new_pulse);
                pulses.push_back((pulse.0, new_pulse));
            }
        }
    }
}
