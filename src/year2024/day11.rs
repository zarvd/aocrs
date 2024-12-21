use std::collections::HashMap;

pub fn solve_part1(input: String) -> String {
    part1(&input).to_string()
}

pub fn solve_part2(input: String) -> String {
    part2(&input).to_string()
}

fn read_stones(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| v.parse::<u64>().unwrap())
        .collect()
}

struct State {
    stones: HashMap<u64, usize>,
    next: HashMap<u64, Vec<u64>>,
}

impl State {
    pub fn new(stones: Vec<u64>) -> Self {
        let m = {
            let mut m = HashMap::new();
            for s in stones {
                m.entry(s).and_modify(|v| *v += 1).or_insert(1);
            }
            m
        };
        Self {
            stones: m,
            next: HashMap::new(),
        }
    }

    pub fn step(&mut self) -> usize {
        let stones: HashMap<_, _> = self.stones.drain().collect();
        let mut rv = 0;
        for (k, v) in stones {
            let children = self.list_children(k);
            rv += children.len() * v;
            for child in children {
                self.stones
                    .entry(child)
                    .and_modify(|cnt| *cnt += v)
                    .or_insert(v);
            }
        }
        rv
    }

    fn list_children(&mut self, stone: u64) -> Vec<u64> {
        if let Some(v) = self.next.get(&stone) {
            return v.clone();
        }

        let rv = blink(stone);
        self.next.insert(stone, rv.clone());
        rv
    }
}

fn part1(input: &str) -> usize {
    let mut state = State::new(read_stones(input));

    let mut rv = 0;
    for _ in 0..25 {
        rv = state.step();
    }

    rv
}

fn part2(input: &str) -> usize {
    let mut state = State::new(read_stones(input));

    let mut rv = 0;
    for _ in 0..75 {
        rv = state.step();
    }

    rv
}

fn blink(stone: u64) -> Vec<u64> {
    match stone {
        0 => vec![1],
        n if n < 10 => vec![n * 2024],
        n => {
            let s = n.to_string();
            if s.len() % 2 == 0 {
                let (l, r) = s.split_at(s.len() / 2);
                let l = l.parse::<u64>().unwrap();
                let r = r.parse::<u64>().unwrap();
                vec![l, r]
            } else {
                vec![n * 2024]
            }
        }
    }
}
