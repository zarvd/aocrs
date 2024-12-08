use crate::Grid;

pub fn solve_part1(input: String) -> String {
    part1(&input).to_string()
}

pub fn solve_part2(input: String) -> String {
    part2(&input).to_string()
}

fn part1(input: &str) -> u64 {
    let grid = Grid::from_str(input);

    let mut rv = 0;
    for i in 0..grid.rows() {
        for j in 0..grid.cols() {
            if grid.get(i, j) != &b'X' {
                continue;
            }
            rv += search_part1(&grid, i, j);
        }
    }

    rv
}

fn search_part1(grid: &Grid<u8>, i: usize, j: usize) -> u64 {
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
        if line.iter().any(|&(x, y)| !grid.in_range(x, y)) {
            continue;
        }

        if grid.get(line[0].0, line[0].1) == &b'M'
            && grid.get(line[1].0, line[1].1) == &b'A'
            && grid.get(line[2].0, line[2].1) == &b'S'
        {
            rv += 1
        }
    }
    rv
}

fn part2(input: &str) -> u64 {
    let grid = Grid::from_str(input);

    let mut rv = 0;
    for i in 1..grid.rows() - 1 {
        for j in 1..grid.cols() - 1 {
            if grid.get(i, j) != &b'A' {
                continue;
            }
            rv += search_part2(&grid, i, j);
        }
    }

    rv
}

fn search_part2(grid: &Grid<u8>, i: usize, j: usize) -> u64 {
    if !(grid.get(i - 1, j - 1) == &b'M' && grid.get(i + 1, j + 1) == &b'S')
        && !(grid.get(i - 1, j - 1) == &b'S' && grid.get(i + 1, j + 1) == &b'M')
    {
        return 0;
    }
    if !(grid.get(i - 1, j + 1) == &b'M' && grid.get(i + 1, j - 1) == &b'S')
        && !(grid.get(i - 1, j + 1) == &b'S' && grid.get(i + 1, j - 1) == &b'M')
    {
        return 0;
    }

    1
}
