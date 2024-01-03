pub fn solve_part1(input: String) -> String {
    part1(&input).to_string()
}

pub fn solve_part2(input: String) -> String {
    part2(&input).to_string()
}

fn part1(input: &str) -> u64 {
    input.split('\n').fold(0, |acc, line| {
        let l = line.find(|c: char| c.is_ascii_digit()).unwrap();
        let r = line.rfind(|c: char| c.is_ascii_digit()).unwrap();
        let line = line.as_bytes();
        acc + ((line[l] - b'0') * 10 + (line[r] - b'0')) as u64
    })
}

fn part2(input: &str) -> u64 {
    const DIGITS: &[&str] = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    input.split('\n').fold(0, |acc, line| {
        let b = line.as_bytes();
        let (mut l, mut r) = (0, 0);
        for i in 0..line.len() {
            if b[i].is_ascii_digit() {
                l = (b[i] - b'0') as u64;
                break;
            }
            for (j, &d) in DIGITS.iter().enumerate() {
                if line[i..].starts_with(d) {
                    l = (j + 1) as u64;
                    break;
                }
            }
            if l > 0 {
                break;
            }
        }
        for i in (0..line.len()).rev() {
            if b[i].is_ascii_digit() {
                r = (b[i] - b'0') as u64;
                break;
            }
            for (j, &d) in DIGITS.iter().enumerate() {
                if line[..=i].ends_with(d) {
                    r = (j + 1) as u64;
                    break;
                }
            }
            if r > 0 {
                break;
            }
        }

        acc + l * 10 + r
    })
}
