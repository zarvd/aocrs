use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

pub fn solve_part1(input: String) -> String {
    let (map, instructions) = normalize_input(input);
    part1(map, instructions).to_string()
}

pub fn solve_part2(input: String) -> String {
    let (map, instructions) = normalize_input(input);
    part2(map, instructions).to_string()
}

fn normalize_input(s: String) -> (HashMap<String, (String, String)>, Vec<Direction>) {
    let (instructions, map) = s.split_once("\n\n").unwrap();

    let instructions = instructions
        .trim()
        .as_bytes()
        .into_iter()
        .map(|v| match v {
            b'L' => Direction::Left,
            b'R' => Direction::Right,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();
    let map = map
        .trim()
        .split('\n')
        .map(|line| {
            let (node, next) = line.split_once(" = ").unwrap();
            let (l, r) = next
                .trim_matches(|c| c == '(' || c == ')')
                .split_once(", ")
                .unwrap();
            (node.to_owned(), (l.to_owned(), r.to_owned()))
        })
        .collect::<HashMap<_, _>>();

    (map, instructions)
}

fn part1(map: HashMap<String, (String, String)>, instructions: Vec<Direction>) -> usize {
    let (mut start, end) = ("AAA".to_owned(), "ZZZ".to_owned());
    let mut steps = 0;
    while start != end {
        for d in &instructions {
            steps += 1;
            let next = map.get(&start).unwrap().to_owned();
            start = match d {
                Direction::Left => next.0,
                Direction::Right => next.1,
            };
            if start == end {
                break;
            }
        }
    }
    steps
}

fn part2(map: HashMap<String, (String, String)>, instructions: Vec<Direction>) -> usize {
    fn gcd(mut x: usize, mut y: usize) -> usize {
        while y != 0 {
            (x, y) = (y, x % y);
        }
        x
    }
    fn lcm(x: usize, y: usize) -> usize {
        x / gcd(x, y) * y
    }

    let mut rv = vec![];
    for node in map.keys() {
        if !node.ends_with('A') {
            continue;
        }
        let mut steps = 0;
        let mut node = node.clone();
        while !node.ends_with('Z') {
            let next = map.get(&node).unwrap().to_owned();
            node = match instructions[steps % instructions.len()] {
                Direction::Left => next.0,
                Direction::Right => next.1,
            };
            steps += 1;
        }
        rv.push(steps);
    }
    rv.into_iter().reduce(|acc, elem| lcm(acc, elem)).unwrap()
}
