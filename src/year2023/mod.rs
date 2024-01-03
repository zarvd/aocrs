pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

pub fn run(day: u64, part: u64, input: String) -> String {
    use std::collections::HashMap;

    let handlers = [
        ("1_1", day1::solve_part1 as fn(String) -> String),
        ("1_2", day1::solve_part2 as fn(String) -> String),
        ("2_1", day2::solve_part1 as fn(String) -> String),
        ("2_2", day2::solve_part2 as fn(String) -> String),
        ("3_1", day3::solve_part1 as fn(String) -> String),
        ("3_2", day3::solve_part2 as fn(String) -> String),
        ("4_1", day4::solve_part1 as fn(String) -> String),
        ("4_2", day4::solve_part2 as fn(String) -> String),
        ("5_1", day5::solve_part1 as fn(String) -> String),
        ("5_2", day5::solve_part2 as fn(String) -> String),
        ("6_1", day6::solve_part1 as fn(String) -> String),
        ("6_2", day6::solve_part2 as fn(String) -> String),
        ("7_1", day7::solve_part1 as fn(String) -> String),
        ("7_2", day7::solve_part2 as fn(String) -> String),
        ("8_1", day8::solve_part1 as fn(String) -> String),
        ("8_2", day8::solve_part2 as fn(String) -> String),
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
        for day in 1..=8 {
            for part in 1..=2 {
                let num_tests = {
                    let mut rv = -1;
                    while fs::metadata(format!(
                        "./dataset/2023/day{day}/part{part}/{}.input",
                        rv + 1
                    ))
                    .is_ok()
                    {
                        rv += 1;
                    }
                    rv + 1
                };

                println!("====================================================");
                println!("Running {num_tests} tests for year 2023 - day {day} - part {part}");

                for i in 0..num_tests {
                    let input =
                        fs::read_to_string(format!("./dataset/2023/day{day}/part{part}/{i}.input"))
                            .expect("read test input");
                    let output = fs::read_to_string(format!(
                        "./dataset/2023/day{day}/part{part}/{i}.output"
                    ))
                    .expect("read test output");

                    let start = Instant::now();
                    let actual = run(day, part, input);
                    let elapsed = start.elapsed();
                    println!(
                        "Test #{i} Elapsed: {}ms / {}ns",
                        elapsed.as_millis(),
                        elapsed.as_nanos()
                    );
                    assert_eq!(actual, output, "Test #{i} failed");
                }

                println!("====================================================");
            }
        }
    }
}
