use std::collections::HashMap;

pub fn solve_part1(input: String) -> String {
    part1(&input).to_string()
}

pub fn solve_part2(input: String) -> String {
    part2(&input).to_string()
}

fn read_to_two_list(input: &str) -> (Vec<i64>, Vec<i64>) {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .into_iter()
        .map(|line| {
            let v: Vec<_> = line
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
            (v[0], v[1])
        })
        .unzip()
}

fn part1(input: &str) -> u64 {
    let (mut left, mut right) = read_to_two_list(input);

    left.sort_unstable();
    right.sort_unstable();

    left.into_iter()
        .zip(right.into_iter())
        .fold(0, |acc, (l, r)| acc + (l - r).abs()) as u64
}

fn part2(input: &str) -> u64 {
    let (left, right) = read_to_two_list(input);
    let m = {
        let mut m = HashMap::new();
        for v in right {
            m.entry(v).and_modify(|v| *v += 1).or_insert(1);
        }
        m
    };

    left.into_iter()
        .fold(0i64, |acc, v| acc + v * m.get(&v).unwrap_or(&0)) as u64
}
