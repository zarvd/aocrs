pub fn solve_part1(input: String) -> String {
    part1(normalize_input(input)).to_string()
}

pub fn solve_part2(input: String) -> String {
    part2(normalize_input(input)).to_string()
}

fn normalize_input(s: String) -> Vec<Vec<i64>> {
    s.split('\n')
        .map(|line| line.split(' ').map(|v| v.parse::<i64>().unwrap()).collect())
        .collect()
}

fn build_levels(vs: Vec<i64>) -> Vec<Vec<i64>> {
    let mut levels = vec![vs];
    loop {
        let last = levels.last().unwrap();
        let mut nexts = Vec::with_capacity(last.len() - 1);
        for i in 1..last.len() {
            nexts.push(last[i] - last[i - 1]);
        }
        if nexts.iter().all(|&x| x == 0) {
            levels.push(nexts);
            break;
        }
        levels.push(nexts);
    }
    levels
}

fn part1(vs: Vec<Vec<i64>>) -> i64 {
    fn next_value(vs: Vec<i64>) -> i64 {
        let levels = build_levels(vs);
        let mut diff = 0;
        for i in (0..levels.len() - 1).rev() {
            let vs = &levels[i];
            diff = vs.last().unwrap() + diff;
        }
        diff
    }

    vs.into_iter().map(next_value).sum()
}

fn part2(vs: Vec<Vec<i64>>) -> i64 {
    fn previous_value(vs: Vec<i64>) -> i64 {
        let levels = build_levels(vs);
        let mut diff = 0;
        for i in (0..levels.len() - 1).rev() {
            let vs = &levels[i];
            diff = vs.first().unwrap() - diff;
        }
        diff
    }

    vs.into_iter().map(previous_value).sum()
}
