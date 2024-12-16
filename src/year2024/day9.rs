pub fn solve_part1(input: String) -> String {
    part1(&input).to_string()
}

pub fn solve_part2(input: String) -> String {
    part2(&input).to_string()
}

fn checksum(lst: &[i64]) -> i64 {
    let mut rv = 0;
    for i in 0..lst.len() {
        if lst[i] < 0 {
            continue;
        }
        rv += i as i64 * lst[i];
    }
    rv
}

fn build_fs(input: &str) -> Vec<i64> {
    let mut rv = vec![];
    let mut id = 0;
    let mut free = false;
    for v in input.trim().as_bytes() {
        let d = (*v - b'0') as i64;
        for _ in 0..d {
            rv.push(if free { -1 } else { id });
        }
        if !free {
            id += 1;
        }
        free = !free;
    }
    rv
}

fn part1(input: &str) -> usize {
    let mut rv = build_fs(input);

    let mut l = 0;
    let mut r = rv.len() - 1;
    while l < r {
        if rv[l] != -1 {
            l += 1;
            continue;
        }
        if rv[r] == -1 {
            r -= 1;
            continue;
        }
        rv.swap(l, r);
        l += 1;
        r -= 1;
    }

    checksum(&rv[..=r]) as _
}

fn part2(input: &str) -> usize {
    let mut rv = build_fs(input);

    let mut r = rv.len() - 1;

    while 0 < r {
        if rv[r] == -1 {
            r -= 1;
            continue;
        }
        let file_len = next_file_space_len(&rv, r);
        let mut l = 0;
        while l < r {
            if rv[l] != -1 {
                l += 1;
                continue;
            }
            let free_len = next_free_space_len(&rv, l);
            if free_len >= file_len {
                for i in 0..file_len {
                    rv.swap(l + i, r - i);
                }
                break;
            }
            l += free_len + 1;
        }
        if r > file_len {
            r -= file_len;
        } else {
            break;
        }
    }

    checksum(&rv) as _
}

fn next_free_space_len(vs: &[i64], i: usize) -> usize {
    let mut j = i;
    while j < vs.len() && vs[j] == -1 {
        j += 1;
    }
    j - i
}

fn next_file_space_len(vs: &[i64], i: usize) -> usize {
    let mut j = i;
    while j > 0 && vs[j] == vs[i] {
        j -= 1;
    }
    if vs[j] == vs[i] {
        i - j + 1
    } else {
        i - j
    }
}
