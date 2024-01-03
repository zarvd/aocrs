use std::collections::HashSet;

pub fn solve_part1(input: String) -> String {
    part1(normalize_input(input)).to_string()
}

pub fn solve_part2(input: String) -> String {
    part2(normalize_input(input)).to_string()
}

fn normalize_input(s: String) -> Vec<(HashSet<u64>, HashSet<u64>)> {
    fn parse_list_of_numbers(line: &str) -> HashSet<u64> {
        line.split(' ')
            .map(|v| v.trim())
            .filter(|v| !v.is_empty())
            .map(|v| v.parse::<u64>().unwrap())
            .collect()
    }

    s.split('\n')
        .map(|line| {
            // line: `Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53`
            let (winnings, owns) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
            (parse_list_of_numbers(winnings), parse_list_of_numbers(owns))
        })
        .collect()
}

fn part1(input: Vec<(HashSet<u64>, HashSet<u64>)>) -> u64 {
    let mut rv = 0;
    for (winnings, owns) in input {
        let mut p = 0;
        for num in owns {
            if !winnings.contains(&num) {
                continue;
            }
            if p == 0 {
                p = 1;
            } else {
                p <<= 1;
            }
        }
        rv += p as u64;
    }

    rv
}

fn part2(input: Vec<(HashSet<u64>, HashSet<u64>)>) -> u64 {
    let mut rv = 0;
    let mut cards = vec![1; input.len()];
    for (i, (winnings, owns)) in input.into_iter().enumerate() {
        let mut p = 0;
        for num in owns {
            if !winnings.contains(&num) {
                continue;
            }
            p += 1;
        }
        for j in 1..=p {
            cards[i + j] += cards[i];
        }
        rv += cards[i];
    }

    rv
}
