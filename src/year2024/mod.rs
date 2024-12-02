mod day1;
mod day2;

pub fn run(day: u64, part: u64, input: String) -> String {
    use std::collections::HashMap;

    type SolveFn = fn(String) -> String;
    let handlers = [
        ("1_1", day1::solve_part1 as SolveFn),
        ("1_2", day1::solve_part2 as SolveFn),
        ("2_1", day2::solve_part1 as SolveFn),
        ("2_2", day2::solve_part2 as SolveFn),
    ]
    .into_iter()
    .map(|(i, f)| (i.to_owned(), f))
    .collect::<HashMap<_, _>>();

    let id = format!("{}_{}", day, part);
    handlers.get(&id).unwrap()(input)
}
