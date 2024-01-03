pub fn solve_part1(input: String) -> String {
    part1(normalize_input(input)).to_string()
}

pub fn solve_part2(input: String) -> String {
    part2(normalize_input(input)).to_string()
}

fn normalize_input(s: String) -> Vec<Vec<u8>> {
    s.split('\n')
        .map(|line| line.to_owned().into_bytes())
        .collect()
}

#[inline(always)]
fn list_neighbor_indexes(r: usize, c: usize) -> Vec<(i64, i64)> {
    let (row, col) = (r as i64, c as i64);
    vec![
        (row - 1, col - 1),
        (row - 1, col),
        (row - 1, col + 1),
        (row + 1, col - 1),
        (row + 1, col),
        (row + 1, col + 1),
        (row, col - 1),
        (row, col + 1),
    ]
}

fn part1(input: Vec<Vec<u8>>) -> u64 {
    fn is_adjacent_to_symbol(input: &[Vec<u8>], r: usize, c: usize) -> bool {
        let candidates = list_neighbor_indexes(r, c);
        for (r, c) in candidates {
            if !(0 <= r && r < input.len() as i64) {
                continue;
            }
            if !(0 <= c && c < input[r as usize].len() as i64) {
                continue;
            }
            let v = input[r as usize][c as usize];
            if v == b'.' || v.is_ascii_digit() {
                continue;
            }
            return true;
        }
        false
    }

    let mut rv = 0;
    for r in 0..input.len() {
        let mut c = 0;
        while c < input[r].len() {
            if !input[r][c].is_ascii_digit() {
                c += 1;
                continue;
            }
            let mut v = (input[r][c] - b'0') as u64;
            let mut is_part_number = is_adjacent_to_symbol(&input, r, c);
            c += 1;
            while c < input[r].len() && input[r][c].is_ascii_digit() {
                v = 10 * v + (input[r][c] - b'0') as u64;
                is_part_number = is_part_number || is_adjacent_to_symbol(&input, r, c);
                c += 1;
            }
            if is_part_number {
                rv += v;
            }
        }
    }
    rv
}

fn part2(input: Vec<Vec<u8>>) -> u64 {
    use std::collections::HashSet;

    let mut rv = 0;
    for r in 0..input.len() {
        for c in 0..input[r].len() {
            if input[r][c] != b'*' {
                continue;
            }

            let mut gear_part_numbers = vec![];
            let candidates = list_neighbor_indexes(r, c);
            let mut visited = HashSet::new();

            for (r, c) in candidates {
                if !(0 <= r && r < input.len() as i64) {
                    continue;
                }
                if !(0 <= c && c < input[r as usize].len() as i64) {
                    continue;
                }

                let v = input[r as usize][c as usize];
                if !v.is_ascii_digit() {
                    continue;
                }
                let mut col = c as usize;
                while col > 0 && input[r as usize][col - 1].is_ascii_digit() {
                    col -= 1;
                }
                if visited.contains(&(r, col)) {
                    continue;
                }
                visited.insert((r, col));

                let mut v = 0;
                while col < input[r as usize].len() && input[r as usize][col].is_ascii_digit() {
                    v = 10 * v + (input[r as usize][col] - b'0') as u64;
                    col += 1;
                }
                gear_part_numbers.push(v);
            }
            if gear_part_numbers.len() <= 1 {
                continue;
            }

            rv += gear_part_numbers
                .into_iter()
                .fold(1, |acc, elem| acc * elem);
        }
    }
    rv
}
