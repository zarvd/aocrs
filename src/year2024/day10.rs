use std::collections::HashSet;

use crate::Grid;

pub fn solve_part1(input: String) -> String {
    part1(&input).to_string()
}

pub fn solve_part2(input: String) -> String {
    part2(&input).to_string()
}

fn list_trailheads(grid: &Grid<u8>) -> Vec<(usize, usize)> {
    let mut rv = Vec::new();
    for r in 0..grid.rows() {
        for c in 0..grid.cols() {
            if grid.get(r, c) == &b'0' {
                rv.push((r, c));
            }
        }
    }
    rv
}

fn part1(input: &str) -> usize {
    let grid = Grid::from_str(input);

    let trailheads = list_trailheads(&grid);

    let mut rv = 0;
    for (r, c) in trailheads {
        rv += scores(&grid, r, c).len();
    }

    rv
}

fn part2(input: &str) -> usize {
    let grid = Grid::from_str(input);

    let trailheads = list_trailheads(&grid);

    let mut rv = 0;
    for (r, c) in trailheads {
        rv += ratings(&grid, r, c);
    }

    rv
}

fn candidates_of(r: usize, c: usize) -> Vec<(i64, i64)> {
    let (r, c) = (r as i64, c as i64);
    vec![(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
}

fn scores(grid: &Grid<u8>, r: usize, c: usize) -> HashSet<(usize, usize)> {
    let v = *grid.get(r, c);
    let candidates = candidates_of(r, c);

    let mut rv = HashSet::new();
    for (nr, nc) in candidates {
        match grid.try_get(nr, nc) {
            Some(&next) if next == v + 1 && next == b'9' => {
                rv.insert((nr as usize, nc as usize));
            }
            Some(&next) if next == v + 1 => {
                rv.extend(scores(grid, nr as usize, nc as usize));
            }
            _ => {}
        }
    }

    rv
}

fn ratings(grid: &Grid<u8>, r: usize, c: usize) -> usize {
    let v = *grid.get(r, c);
    let candidates = candidates_of(r, c);

    let mut rv = 0;
    for (nr, nc) in candidates {
        match grid.try_get(nr, nc) {
            Some(&next) if next == v + 1 && next == b'9' => {
                rv += 1;
            }
            Some(&next) if next == v + 1 => {
                rv += ratings(grid, nr as usize, nc as usize);
            }
            _ => {}
        }
    }

    rv
}
