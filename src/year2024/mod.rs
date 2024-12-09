mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

pub fn run(day: u64, part: u64, input: String) -> String {
    use std::collections::HashMap;

    type SolveFn = fn(String) -> String;
    let handlers = [
        ("1_1", day1::solve_part1 as SolveFn),
        ("1_2", day1::solve_part2 as SolveFn),
        ("2_1", day2::solve_part1 as SolveFn),
        ("2_2", day2::solve_part2 as SolveFn),
        ("3_1", day3::solve_part1 as SolveFn),
        ("3_2", day3::solve_part2 as SolveFn),
        ("4_1", day4::solve_part1 as SolveFn),
        ("4_2", day4::solve_part2 as SolveFn),
        ("5_1", day5::solve_part1 as SolveFn),
        ("5_2", day5::solve_part2 as SolveFn),
        ("6_1", day6::solve_part1 as SolveFn),
        ("6_2", day6::solve_part2 as SolveFn),
        ("7_1", day7::solve_part1 as SolveFn),
        ("7_2", day7::solve_part2 as SolveFn),
        ("8_1", day8::solve_part1 as SolveFn),
        ("8_2", day8::solve_part2 as SolveFn),
    ]
    .into_iter()
    .map(|(i, f)| (i.to_owned(), f))
    .collect::<HashMap<_, _>>();

    let id = format!("{}_{}", day, part);
    handlers.get(&id).unwrap()(input)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::Instant;

    use super::run;

    #[test]
    fn test_run() {
        let mut day = 1;
        while fs::metadata(format!("./dataset/2024/day{day}")).is_ok() {
            let mut part = 1;
            while fs::metadata(format!("./dataset/2024/day{day}/part{part}")).is_ok() {
                let num_tests = {
                    let mut rv = -1;
                    while fs::metadata(format!(
                        "./dataset/2024/day{day}/part{part}/{}.input",
                        rv + 1
                    ))
                    .is_ok()
                    {
                        rv += 1;
                    }
                    rv + 1
                };

                for i in 0..num_tests {
                    let input =
                        fs::read_to_string(format!("./dataset/2024/day{day}/part{part}/{i}.input"))
                            .expect("read test input");
                    let output = fs::read_to_string(format!(
                        "./dataset/2024/day{day}/part{part}/{i}.output"
                    ))
                    .expect("read test output");

                    let start = Instant::now();
                    let actual = run(day, part, input);
                    let elapsed = start.elapsed();
                    assert_eq!(
                        actual,
                        output.trim(),
                        "Test #{i} for year 2024 - day {day} - part {part} failed",
                    );
                    println!(
                        "Running #{}/{} tests for year 2024 - day {:0>2} - part {}: {}ms / {}ns",
                        i + 1,
                        num_tests,
                        day,
                        part,
                        elapsed.as_millis(),
                        elapsed.as_nanos()
                    );
                }
                part += 1;
            }
            day += 1;
        }
    }
}
