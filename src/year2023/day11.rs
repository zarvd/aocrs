pub fn solve_part1(input: String) -> String {
    let (xs, ys) = normalize_input(input);
    part1(xs, ys).to_string()
}

pub fn solve_part2(input: String) -> String {
    let (xs, ys) = normalize_input(input);
    part2(xs, ys).to_string()
}

fn normalize_input(s: String) -> (Vec<u64>, Vec<u64>) {
    let grid = s
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (mut xs, mut ys) = (vec![0; grid.len()], vec![0; grid[0].len()]);
    grid.into_iter().enumerate().for_each(|(x, row)| {
        row.into_iter()
            .enumerate()
            .filter(|&(_, v)| v == '#')
            .for_each(|(y, _)| {
                xs[x] += 1;
                ys[y] += 1;
            })
    });

    (xs, ys)
}

fn distance_of(dimension: &[u64], factor: u64) -> u64 {
    let (mut rv, mut gaps, mut prefix_offset, mut n_prefix) = (0, 0, 0, 0);
    for (i, &cnt) in dimension.into_iter().enumerate() {
        let i = i as u64;
        if cnt == 0 {
            gaps += factor - 1;
            continue;
        }

        let x = i + gaps;

        rv += cnt * (n_prefix * x - prefix_offset); // [0..x] - prefix_offset = distance(galaxy of x, previous galaxy)
        prefix_offset += cnt * x;
        n_prefix += cnt;
    }
    rv
}

fn part1(xs: Vec<u64>, ys: Vec<u64>) -> u64 {
    distance_of(&xs, 2) + distance_of(&ys, 2)
}

fn part2(xs: Vec<u64>, ys: Vec<u64>) -> u64 {
    distance_of(&xs, 1_000_000) + distance_of(&ys, 1_000_000)
}
