pub fn solve_part1(input: String) -> String {
    part1(&input).to_string()
}

pub fn solve_part2(input: String) -> String {
    part2(&input).to_string()
}

fn part1(input: &str) -> u64 {
    let safe: Vec<_> = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|v| is_safe_part1(v))
        .collect();

    safe.len() as _
}

fn is_safe_part1(vs: &[i64]) -> bool {
    if vs.len() <= 1 {
        return true;
    }

    let delta = vs[1] - vs[0];

    for i in 1..vs.len() {
        let d = vs[i] - vs[i - 1];
        if d.is_positive() != delta.is_positive() {
            return false;
        }
        if d.abs() < 1 || d.abs() > 3 {
            return false;
        }
    }
    true
}

fn part2(input: &str) -> u64 {
    let safe: Vec<_> = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|v| is_safe_part2(v))
        .collect();

    safe.len() as _
}

fn is_safe_part2(vs: &[i64]) -> bool {
    if vs.len() <= 1 {
        return true;
    }

    for i in 0..vs.len() {
        let mut next = vs[..i].to_vec();
        next.extend_from_slice(&vs[i + 1..]);
        if is_safe_part1(&next) {
            return true;
        }
    }
    false
}
