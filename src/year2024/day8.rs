use std::collections::HashMap;

use crate::Grid;

pub fn solve_part1(input: String) -> String {
    part1(&input).to_string()
}

pub fn solve_part2(input: String) -> String {
    part2(&input).to_string()
}

fn part1(input: &str) -> usize {
    let grid = Grid::from_str(input);
    let mut anti = Grid::with_default(grid.rows(), grid.cols(), false);

    let mut freqs = HashMap::new();
    for r in 0..grid.rows() {
        for c in 0..grid.cols() {
            let v = *grid.get(r, c);
            if !v.is_ascii_alphanumeric() {
                continue;
            }
            freqs.entry(v).or_insert_with(|| Vec::new()).push((r, c));
        }
    }

    for vs in freqs.into_values() {
        for i in 0..vs.len() {
            for j in i + 1..vs.len() {
                let ((ax, ay), (bx, by)) = (vs[i], vs[j]);

                let (ax, ay, bx, by) = (ax as i64, ay as i64, bx as i64, by as i64);

                let (x0, y0) = (ax - (bx - ax), ay - (by - ay));
                let (x1, y1) = (bx + (bx - ax), by + (by - ay));
                anti.try_set(x0, y0, true);
                anti.try_set(x1, y1, true);
            }
        }
    }

    let mut rv = 0;
    for r in 0..grid.rows() {
        for c in 0..grid.cols() {
            if *anti.get(r, c) {
                rv += 1;
            }
        }
    }

    rv
}

fn part2(input: &str) -> usize {
    let grid = Grid::from_str(input);
    let mut anti = Grid::with_default(grid.rows(), grid.cols(), false);

    let mut freqs = HashMap::new();
    for r in 0..grid.rows() {
        for c in 0..grid.cols() {
            let v = *grid.get(r, c);
            if !v.is_ascii_alphanumeric() {
                continue;
            }
            anti.set(r, c, true);
            freqs.entry(v).or_insert_with(|| Vec::new()).push((r, c));
        }
    }

    for vs in freqs.into_values() {
        for i in 0..vs.len() {
            for j in i + 1..vs.len() {
                let ((mut ax, mut ay), (mut bx, mut by)) = (vs[i], vs[j]);
                if ax > bx {
                    (ax, ay, bx, by) = (bx, by, ax, ay);
                }

                loop {
                    let (lx, ly, rx, ry) = (ax as i64, ay as i64, bx as i64, by as i64);
                    let (x0, y0) = (lx - (rx - lx), ly - (ry - ly));
                    if anti.try_set(x0, y0, true).is_some() {
                        (ax, ay, bx, by) = (x0 as usize, y0 as usize, ax, ay);
                    } else {
                        break;
                    }
                }

                let ((mut ax, mut ay), (mut bx, mut by)) = (vs[i], vs[j]);
                if ax > bx {
                    (ax, ay, bx, by) = (bx, by, ax, ay);
                }
                loop {
                    let (lx, ly, rx, ry) = (ax as i64, ay as i64, bx as i64, by as i64);
                    let (x1, y1) = (rx + (rx - lx), ry + (ry - ly));
                    if anti.try_set(x1, y1, true).is_some() {
                        (ax, ay, bx, by) = (bx, by, x1 as usize, y1 as usize);
                    } else {
                        break;
                    }
                }
            }
        }
    }

    let mut rv = 0;
    for r in 0..grid.rows() {
        for c in 0..grid.cols() {
            if *anti.get(r, c) {
                rv += 1;
            }
        }
    }

    rv
}
