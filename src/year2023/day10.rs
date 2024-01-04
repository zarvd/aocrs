use std::collections::VecDeque;

pub fn solve_part1(input: String) -> String {
    part1(normalize_input(input)).to_string()
}

pub fn solve_part2(input: String) -> String {
    part2(normalize_input(input)).to_string()
}

fn normalize_input(s: String) -> Vec<Vec<char>> {
    s.split('\n').map(|line| line.chars().collect()).collect()
}

const SOUTH: u32 = 0b0100;
const NORTH: u32 = 0b1000;
const EAST: u32 = 0b0001;
const WEST: u32 = 0b0010;

const PIPE_VERTICAL: u32 = SOUTH | NORTH;
const PIPE_HORIZONTAL: u32 = EAST | WEST;
const PIPE_SOUTH_EAST: u32 = SOUTH | EAST;
const PIPE_SOUTH_WEST: u32 = SOUTH | WEST;
const PIPE_NORTH_EAST: u32 = NORTH | EAST;
const PIPE_NORTH_WEST: u32 = NORTH | WEST;

const POINT_START: char = 'S';
const _POINT_GROUND: char = '.';

const fn get_pipe(p: char) -> u32 {
    match p {
        '|' => PIPE_VERTICAL,
        '-' => PIPE_HORIZONTAL,
        'J' => PIPE_NORTH_WEST,
        'L' => PIPE_NORTH_EAST,
        'F' => PIPE_SOUTH_EAST,
        '7' => PIPE_SOUTH_WEST,
        POINT_START => NORTH | SOUTH | EAST | WEST,
        _ => 0,
    }
}

fn get_start(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == POINT_START {
                return (i, j);
            }
        }
    }
    unreachable!()
}

fn list_connected_neighbors(grid: &Vec<Vec<char>>, r: usize, c: usize) -> Vec<(usize, usize)> {
    let (row, col) = (grid.len(), grid[0].len());
    let mut rv = Vec::with_capacity(4);
    let pipe = get_pipe(grid[r][c]);

    if r > 0 && pipe & NORTH > 0 {
        let (nr, nc) = (r - 1, c);
        if get_pipe(grid[nr][nc]) & SOUTH > 0 {
            rv.push((nr, nc));
        }
    }
    if r < row - 1 && pipe & SOUTH > 0 {
        let (nr, nc) = (r + 1, c);
        if get_pipe(grid[nr][nc]) & NORTH > 0 {
            rv.push((nr, nc));
        }
    }
    if c > 0 && pipe & WEST > 0 {
        let (nr, nc) = (r, c - 1);
        if get_pipe(grid[nr][nc]) & EAST > 0 {
            rv.push((nr, nc));
        }
    }
    if c < col - 1 && pipe & EAST > 0 {
        let (nr, nc) = (r, c + 1);
        if get_pipe(grid[nr][nc]) & WEST > 0 {
            rv.push((nr, nc));
        }
    }
    rv
}

fn part1(grid: Vec<Vec<char>>) -> u64 {
    let (row, col) = (grid.len(), grid[0].len());
    let start = get_start(&grid);
    let mut q = VecDeque::new();
    q.push_back(start);
    let mut visited = vec![vec![false; col]; row];
    visited[start.0][start.1] = true;

    let mut rv = 0;
    while !q.is_empty() {
        let n = q.len();
        for _ in 0..n {
            let (r, c) = q.pop_front().unwrap();
            list_connected_neighbors(&grid, r, c)
                .into_iter()
                .for_each(|(r, c)| {
                    if !visited[r][c] {
                        visited[r][c] = true;
                        q.push_back((r, c));
                    }
                });
        }

        if !q.is_empty() {
            rv += 1;
        }
    }

    rv
}

fn part2(grid: Vec<Vec<char>>) -> u64 {
    let (row, col) = (grid.len(), grid[0].len());
    let start = get_start(&grid);

    let mut q = VecDeque::new();
    q.push_back(start);

    let mut visited = vec![vec![false; col]; row];
    visited[start.0][start.1] = true;

    let mut pipes = vec![(start.0 as i64, start.1 as i64)];

    while let Some((r, c)) = q.pop_back() {
        list_connected_neighbors(&grid, r, c)
            .into_iter()
            .find(|&(nr, nc)| !visited[nr][nc])
            .into_iter()
            .for_each(|(nr, nc)| {
                // NOTE: clockwise or counterclockwise
                visited[nr][nc] = true;
                q.push_back((nr, nc));
                pipes.push((nr as i64, nc as i64));
            });
    }

    ((shoelace(&pipes).abs() - pipes.len() as i64 + 3) / 2) as _
}

#[inline(always)]
fn shoelace(ps: &[(i64, i64)]) -> i64 {
    let mut rv = 0;

    for i in 1..ps.len() {
        let p0 = ps[i - 1];
        let p1 = ps[i];
        rv += (p0.1 + p1.1) * (p0.0 - p1.0);
    }

    rv
}
