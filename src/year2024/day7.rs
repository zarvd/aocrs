pub fn solve_part1(input: String) -> String {
    part1(&input).to_string()
}

pub fn solve_part2(input: String) -> String {
    part2(&input).to_string()
}

fn read_equations(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.split_once(':').unwrap())
        .map(|(l, r)| {
            (
                l.parse::<u64>().unwrap(),
                r.trim()
                    .split_whitespace()
                    .map(|v| v.parse::<u64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> usize {
    let lines = read_equations(input);

    let mut rv = 0;
    fn is_true(sum: u64, p: u64, nums: &[u64]) -> bool {
        if sum < p {
            return false;
        }
        if nums.is_empty() {
            return sum == p;
        }
        if is_true(sum, p + nums[0], &nums[1..]) {
            return true;
        }
        if is_true(sum, p * nums[0], &nums[1..]) {
            return true;
        }

        false
    }
    for (sum, nums) in lines {
        if is_true(sum, nums[0], &nums[1..]) {
            rv += sum;
        }
    }

    rv as _
}

fn part2(input: &str) -> usize {
    let lines = read_equations(input);

    let mut rv = 0;
    fn is_true(sum: u64, p: u64, nums: &[u64]) -> bool {
        if sum < p {
            return false;
        }
        if nums.is_empty() {
            return sum == p;
        }
        if is_true(sum, p + nums[0], &nums[1..]) {
            return true;
        }
        if is_true(sum, p * nums[0], &nums[1..]) {
            return true;
        }
        {
            let mut d = 10;
            while p.checked_mul(d).is_some() {
                if nums[0] / d > 0 {
                    d *= 10;
                } else {
                    break;
                }
            }
            if let Some(v) = (p * d).checked_add(nums[0]) {
                if is_true(sum, v, &nums[1..]) {
                    return true;
                }
            }
        }

        false
    }
    for (sum, nums) in lines {
        if is_true(sum, nums[0], &nums[1..]) {
            rv += sum;
        }
    }

    rv as _
}
