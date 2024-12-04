pub fn solve_part1(input: String) -> String {
    part1(&input).to_string()
}

pub fn solve_part2(input: String) -> String {
    part2(&input).to_string()
}

fn part1(input: &str) -> u64 {
    let mut i = 0;
    let mut rv = 0;
    while i + 7 < input.len() {
        if !input[i..].starts_with("mul(") {
            i += 1;
            continue;
        }
        let (x, x_idx) = if let Some(v) = read_int(&input[i + 4..].as_bytes()) {
            v
        } else {
            i += 4;
            continue;
        };

        if input[i + 4 + x_idx..].as_bytes()[0] != b',' {
            i += 4 + x_idx;
            continue;
        }
        let (y, y_idx) = if let Some(v) = read_int(&input[i + 4 + x_idx + 1..].as_bytes()) {
            v
        } else {
            i += 4 + x_idx + 1;
            continue;
        };
        if input[i + 4 + x_idx + 1 + y_idx..].as_bytes()[0] != b')' {
            i += 4 + x_idx + 1 + y_idx;
            continue;
        }
        rv += x * y;
        i += 7;
    }
    rv as _
}

fn part2(input: &str) -> u64 {
    let input = input.as_bytes();

    let mut rv = 0;
    let mut enabled = true;
    for i in 0..input.len() {
        match input[i] {
            b'm' if enabled => {
                if let Some(v) = try_mul(&input[i..]) {
                    rv += v;
                }
            }
            b'd' => {
                if input[i..].starts_with(b"don't()") {
                    enabled = false;
                } else if input[i..].starts_with(b"do()") {
                    enabled = true;
                }
            }
            _ => {}
        }
    }

    rv as _
}

fn try_mul(input: &[u8]) -> Option<i64> {
    if !input.starts_with(b"mul(") {
        return None;
    }

    let mut i = 4;

    let (x, idx) = read_int(&input[i..])?;
    i += idx;
    if input[i] != b',' {
        return None;
    }
    i += 1;
    let (y, idx) = read_int(&input[i..])?;
    i += idx;
    if input[i] != b')' {
        return None;
    }
    Some(x * y)
}

fn read_int(input: &[u8]) -> Option<(i64, usize)> {
    let mut v = 0;
    let mut none = true;
    let mut idx = 0;
    for i in 0..3 {
        if !input[i].is_ascii_digit() {
            break;
        }
        v = v * 10 + (input[i] - b'0') as i64;
        none = false;
        idx = i;
    }

    if none {
        None
    } else {
        Some((v, idx + 1))
    }
}
