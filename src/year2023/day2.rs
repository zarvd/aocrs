pub fn solve_part1(input: String) -> String {
    part1(normalize_input(input), (12, 13, 14)).to_string()
}

pub fn solve_part2(input: String) -> String {
    part2(normalize_input(input)).to_string()
}

fn normalize_input(s: String) -> Vec<Vec<(u64, u64, u64)>> {
    s.split('\n')
        .map(|line| {
            // line: `Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green`
            let sets = line.split_once(": ").unwrap().1;
            sets.split("; ")
                .map(|set| {
                    // set: `3 blue, 4 red`
                    let (mut red, mut green, mut blue) = (0, 0, 0);
                    set.split(", ").for_each(|cube| {
                        // cube: `3 blue`
                        let (num, color) = cube.split_once(' ').unwrap();
                        let num = num.parse::<u64>().unwrap();
                        match color {
                            "red" => red = num,
                            "green" => green = num,
                            "blue" => blue = num,
                            _ => panic!("unknown color: {color}"),
                        }
                    });
                    (red, green, blue)
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn part1(games: Vec<Vec<(u64, u64, u64)>>, bag: (u64, u64, u64)) -> usize {
    let mut rv = vec![];
    for (id, game) in games.into_iter().enumerate() {
        if game
            .into_iter()
            .all(|(r, g, b)| r <= bag.0 && g <= bag.1 && b <= bag.2)
        {
            rv.push(id + 1);
        }
    }
    rv.into_iter().sum::<usize>()
}

fn part2(games: Vec<Vec<(u64, u64, u64)>>) -> u64 {
    let mut rv = 0;
    for game in games {
        let (mut r, mut g, mut b) = (0, 0, 0);
        game.into_iter().for_each(|set| {
            r = r.max(set.0);
            g = g.max(set.1);
            b = b.max(set.2);
        });
        rv += r * g * b;
    }
    rv
}
