pub fn solve_part1(input: String) -> String {
    part1(normalize_input(input)).to_string()
}

pub fn solve_part2(input: String) -> String {
    part2(normalize_input(input)).to_string()
}

fn normalize_input(s: String) -> Vec<(u64, u64)> {
    let (times, distances) = s
        .split_once('\n')
        .map(|(t, d)| {
            (
                t.trim().split_once(':').unwrap().1.to_owned(),
                d.trim().split_once(':').unwrap().1.to_owned(),
            )
        })
        .unwrap();
    let input = times
        .trim()
        .split(' ')
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .zip(
            distances
                .trim()
                .split(' ')
                .map(|v| v.trim())
                .filter(|v| !v.is_empty()),
        )
        .map(|(t, d)| {
            let t = t.parse::<u64>().unwrap();
            let d = d.parse::<u64>().unwrap();
            (t, d)
        })
        .collect::<Vec<_>>();

    input
}

#[inline(always)]
const fn distance(sec: u64, total: u64) -> u64 {
    assert!(sec <= total);
    sec * (total - sec)
}

fn search_for_left(target: u64, total: u64, mut l: u64, mut r: u64) -> u64 {
    while l < r {
        let m = (l + r) / 2;
        if distance(m, total) <= target && target < distance(m + 1, total) {
            return m + 1;
        }
        if target <= distance(m, total) {
            r = m;
        } else {
            l = m + 1;
        }
    }
    l
}
fn search_for_right(target: u64, total: u64, mut l: u64, mut r: u64) -> u64 {
    while l < r {
        let m = (l + r) / 2;
        if target < distance(m, total) && distance(m + 1, total) <= target {
            return m;
        }
        if distance(m, total) <= target {
            r = m;
        } else {
            l = m + 1;
        }
    }
    r
}

fn part1(input: Vec<(u64, u64)>) -> u64 {
    input
        .into_iter()
        .map(|(t, d)| {
            let l = search_for_left(d, t, 0, t);
            let r = search_for_right(d, t, 0, t);
            r - l + 1
        })
        .fold(1, |acc, v| acc * v)
}

fn part2(input: Vec<(u64, u64)>) -> u64 {
    fn concat(vs: Vec<u64>) -> u64 {
        vs.into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<u64>()
            .unwrap()
    }
    let (ts, ds) = input.into_iter().unzip();
    let (t, d) = (concat(ts), concat(ds));
    let l = search_for_left(d, t, 0, t);
    let r = search_for_right(d, t, 0, t);
    r - l + 1
}
