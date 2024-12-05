pub fn solve_part1(input: String) -> String {
    part1(&input).to_string()
}

pub fn solve_part2(input: String) -> String {
    part2(&input).to_string()
}

fn read_grid(input: &str) -> Vec<Vec<u8>> {
    input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.as_bytes().to_vec())
        .collect()
}

fn part1(input: &str) -> u64 {
    let grid = read_grid(input);

    let r = grid.len();
    let c = grid[0].len();

    let mut rv = 0;
    for i in 0..r {
        for j in 0..c {
            if grid[i][j] != b'X' {
                continue;
            }
            rv += search_part1(&grid, i, j);
        }
    }

    rv
}

fn search_part1(grid: &[Vec<u8>], i: usize, j: usize) -> u64 {
    let r = grid.len() as i32;
    let c = grid[0].len() as i32;

    let mut rv = 0;

    let candidates = {
        let (i, j) = (i as i32, j as i32);
        [
            [(i - 1, j), (i - 2, j), (i - 3, j)],
            [(i + 1, j), (i + 2, j), (i + 3, j)],
            [(i, j - 1), (i, j - 2), (i, j - 3)],
            [(i, j + 1), (i, j + 2), (i, j + 3)],
            [(i - 1, j - 1), (i - 2, j - 2), (i - 3, j - 3)],
            [(i - 1, j + 1), (i - 2, j + 2), (i - 3, j + 3)],
            [(i + 1, j - 1), (i + 2, j - 2), (i + 3, j - 3)],
            [(i + 1, j + 1), (i + 2, j + 2), (i + 3, j + 3)],
        ]
    };

    for line in candidates {
        if line
            .iter()
            .any(|&(x, y)| x < 0 || y < 0 || x >= r || y >= c)
        {
            continue;
        }
        if grid[line[0].0 as usize][line[0].1 as usize] == b'M'
            && grid[line[1].0 as usize][line[1].1 as usize] == b'A'
            && grid[line[2].0 as usize][line[2].1 as usize] == b'S'
        {
            rv += 1
        }
    }
    rv
}

fn part2(input: &str) -> u64 {
    let grid = read_grid(input);
    let r = grid.len();
    let c = grid[0].len();

    let mut rv = 0;
    for i in 1..r - 1 {
        for j in 1..c - 1 {
            if grid[i][j] != b'A' {
                continue;
            }
            rv += search_part2(&grid, i, j);
        }
    }

    rv
}

fn search_part2(grid: &[Vec<u8>], i: usize, j: usize) -> u64 {
    if !(grid[i - 1][j - 1] == b'M' && grid[i + 1][j + 1] == b'S')
        && !(grid[i - 1][j - 1] == b'S' && grid[i + 1][j + 1] == b'M')
    {
        return 0;
    }
    if !(grid[i - 1][j + 1] == b'M' && grid[i + 1][j - 1] == b'S')
        && !(grid[i - 1][j + 1] == b'S' && grid[i + 1][j - 1] == b'M')
    {
        return 0;
    }

    1
}
