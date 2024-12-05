use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

pub fn solve_part1(input: String) -> String {
    part1(&input).to_string()
}

pub fn solve_part2(input: String) -> String {
    part2(&input).to_string()
}

fn read_input(input: &str) -> (HashMap<i64, HashSet<i64>>, Vec<Vec<i64>>) {
    let (orders, rules) = input.split_once("\n\n").unwrap();

    let mut after = HashMap::new();

    orders.split('\n').for_each(|l| {
        let (left, right) = l.split_once('|').unwrap();
        let left = left.parse::<i64>().unwrap();
        let right = right.parse::<i64>().unwrap();
        after.entry(left).or_insert_with(HashSet::new).insert(right);
    });

    let updates: Vec<_> = rules
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split(',')
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    (after, updates)
}

fn is_ordered(after: &HashMap<i64, HashSet<i64>>, update: &[i64]) -> bool {
    for i in 1..update.len() {
        let (left, right) = (update[i - 1], update[i]);
        match after.get(&left) {
            Some(afters) if afters.contains(&right) => continue,
            _ => return false,
        }
    }
    true
}

fn part1(input: &str) -> u64 {
    let (after, updates) = read_input(input);
    let mut rv = 0;

    for u in updates {
        if is_ordered(&after, &u) {
            rv += u[u.len() / 2] as u64;
        }
    }

    rv
}

fn sort(after: &HashMap<i64, HashSet<i64>>, update: &[i64]) -> Vec<i64> {
    let mut rv = update.to_vec();
    rv.sort_by(|x, y| match after.get(x) {
        Some(vs) if vs.contains(y) => Ordering::Less,
        _ => match after.get(y) {
            Some(vs) if vs.contains(x) => Ordering::Greater,
            _ => Ordering::Equal,
        },
    });
    rv
}

fn part2(input: &str) -> u64 {
    let (after, updates) = read_input(input);
    let mut rv = 0;

    for u in updates {
        if is_ordered(&after, &u) {
            continue;
        }
        let sorted = sort(&after, &u);
        rv += sorted[sorted.len() / 2] as u64;
    }

    rv
}
