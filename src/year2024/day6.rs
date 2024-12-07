pub fn solve_part1(input: String) -> String {
    part1(&input).to_string()
}

pub fn solve_part2(input: String) -> String {
    part2(&input).to_string()
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub const fn mask(&self) -> u8 {
        match self {
            Self::Up => 0b0000_1000,
            Self::Down => 0b0000_0100,
            Self::Left => 0b0000_0010,
            Self::Right => 0b0000_0001,
        }
    }

    pub const fn delta(&self) -> (i64, i64) {
        match self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
        }
    }

    pub const fn apply_delta(&self, p: (usize, usize)) -> (i64, i64) {
        let d = self.delta();
        (p.0 as i64 + d.0, p.1 as i64 + d.1)
    }

    pub const fn next(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

fn read_grid(input: &str) -> Vec<Vec<u8>> {
    input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.as_bytes().to_vec())
        .collect()
}

fn find_start_point(grid: &[Vec<u8>]) -> (usize, usize) {
    let (r, c) = (grid.len(), grid[0].len());
    for i in 0..r {
        for j in 0..c {
            if grid[i][j] == b'^' {
                return (i, j);
            }
        }
    }
    unreachable!()
}

fn traverse(grid: &Vec<Vec<u8>>, mut p: (usize, usize)) -> Option<u64> {
    let (r, c) = (grid.len(), grid[0].len());
    let mut d = Direction::Up;
    let mut visited = vec![vec![0u8; c]; r];
    visited[p.0][p.1] = d.mask();

    loop {
        let next = d.apply_delta(p);
        if next.0 < 0 || next.0 >= r as i64 || next.1 < 0 || next.1 >= c as i64 {
            break;
        }
        match grid[next.0 as usize][next.1 as usize] {
            b'#' => {
                d = d.next();
                continue;
            }
            _ => {
                if visited[next.0 as usize][next.1 as usize] & d.mask() > 0 {
                    return None;
                }
                p = (next.0 as usize, next.1 as usize);
                visited[next.0 as usize][next.1 as usize] |= d.mask();
            }
        }
    }

    let mut rv = 0;
    for i in 0..r {
        for j in 0..c {
            if visited[i][j] > 0 {
                rv += 1;
            }
        }
    }

    Some(rv)
}

fn part1(input: &str) -> u64 {
    let grid = read_grid(input);
    let p = find_start_point(&grid);

    traverse(&grid, p).unwrap()
}

fn part2(input: &str) -> u64 {
    let mut grid = read_grid(input);
    let (r, c) = (grid.len(), grid[0].len());

    let p = find_start_point(&grid);
    let mut rv = 0;
    for i in 0..r {
        for j in 0..c {
            if grid[i][j] == b'.' {
                grid[i][j] = b'#';
                if traverse(&grid, p).is_none() {
                    rv += 1;
                }
                grid[i][j] = b'.';
            }
        }
    }
    rv
}
