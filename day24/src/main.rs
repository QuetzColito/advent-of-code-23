use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    let input = include_str!("input.txt");
    let test = include_str!("test.txt");
    println!("{}", part1(test, 7., 27.));
    println!("{}", part1(input, 200000000000000., 400000000000000.));
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

struct HailStorm {
    current: (f64, f64, f64),
    velocity: (f64, f64, f64),
}

impl HailStorm {
    fn new(line: &str) -> Self {
        let (current, velocity) = line.split_once('@').unwrap();
        HailStorm {
            current: tripify(current),
            velocity: tripify(velocity),
        }
    }

    fn collides(&self, other: &HailStorm, start: f64, end: f64) -> bool {
        let (px1, py1, _) = self.current;
        let (vx1, vy1, _) = self.velocity;
        let (px2, py2, _) = other.current;
        let (vx2, vy2, _) = other.velocity;
        let determinant = vx1 * vy2 - vx2 * vy1;
        if determinant == 0. {
            false
        } else {
            let t = ((px2 - px1) * vy2 - (py2 - py1) * vx2) / determinant;
            let s = ((px2 - px1) * vy1 - (py2 - py1) * vx1) / determinant;
            let cx = px1 + t * vx1;
            let cy = py1 + t * vy1;
            t >= 0. && s >= 0. && cx >= start && cx <= end && cy >= start && cy <= end
        }
    }
}

fn tripify(tripel: &str) -> (f64, f64, f64) {
    let values: Vec<f64> = tripel
        .split(',')
        .filter_map(|x| x.trim().parse().ok())
        .collect();
    (values[0], values[1], values[2])
}

fn part1(input: &str, start: f64, end: f64) -> u64 {
    let hailstorms: Vec<HailStorm> = input.lines().map(HailStorm::new).collect();
    let mut intersections = 0;
    let mut seen = 1;
    for i in 0..hailstorms.len() {
        for j in seen..hailstorms.len() {
            if hailstorms[i].collides(&hailstorms[j], start, end) {
                intersections += 1;
            }
        }
        seen += 1;
    }
    intersections
}

fn part2(input: &str) -> u64 {
    2
}
