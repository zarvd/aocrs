pub mod day1;
mod day10;
mod day11;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
mod day9;

pub fn run(day: u64, part: u64, input: String) -> String {
    use std::collections::HashMap;

    type SolveFn = fn(String) -> String;
    let handlers = [
        ((1, 1), day1::solve_part1 as SolveFn),
        ((1, 2), day1::solve_part2 as SolveFn),
        ((2, 1), day2::solve_part1 as SolveFn),
        ((2, 2), day2::solve_part2 as SolveFn),
        ((3, 1), day3::solve_part1 as SolveFn),
        ((3, 2), day3::solve_part2 as SolveFn),
        ((4, 1), day4::solve_part1 as SolveFn),
        ((4, 2), day4::solve_part2 as SolveFn),
        ((5, 1), day5::solve_part1 as SolveFn),
        ((5, 2), day5::solve_part2 as SolveFn),
        ((6, 1), day6::solve_part1 as SolveFn),
        ((6, 2), day6::solve_part2 as SolveFn),
        ((7, 1), day7::solve_part1 as SolveFn),
        ((7, 2), day7::solve_part2 as SolveFn),
        ((8, 1), day8::solve_part1 as SolveFn),
        ((8, 2), day8::solve_part2 as SolveFn),
        ((9, 1), day9::solve_part1 as SolveFn),
        ((9, 2), day9::solve_part2 as SolveFn),
        ((10, 1), day10::solve_part1 as SolveFn),
        ((10, 2), day10::solve_part2 as SolveFn),
        ((11, 1), day11::solve_part1 as SolveFn),
        ((11, 2), day11::solve_part2 as SolveFn),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();

    handlers.get(&(day, part)).unwrap()(input)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::Instant;

    use super::run;

    fn run_test_with_dataset(day: u64, part: u64) {
        let dir = format!("./dataset/2023/day{day}/part{part}");
        assert!(fs::metadata(&dir).is_ok(), "dataset not found");

        let input_path = format!("{dir}/{}.input", part - 1);
        let output_path = format!("{dir}/{}.output", part - 1);
        let input =
            fs::read_to_string(&input_path).expect(&format!("read test input: {input_path}"));
        let output =
            fs::read_to_string(&output_path).expect(&format!("read test output: {output_path}"));

        let start = Instant::now();
        let actual = run(day, part, input);
        let elapsed = start.elapsed();
        assert_eq!(
            actual,
            output.trim(),
            "Test for year 2023 - day {day} - part {part} failed"
        );
        println!(
            "Running test for year 2023 - day {:0>2} - part {}: {}ms / {}ns",
            day,
            part,
            elapsed.as_millis(),
            elapsed.as_nanos()
        );
    }

    #[test]
    fn test_day1() {
        run_test_with_dataset(1, 1);
        run_test_with_dataset(1, 2);
    }

    #[test]
    fn test_day2() {
        run_test_with_dataset(2, 1);
        run_test_with_dataset(2, 2);
    }

    #[test]
    fn test_day3() {
        run_test_with_dataset(3, 1);
        run_test_with_dataset(3, 2);
    }

    #[test]
    fn test_day4() {
        run_test_with_dataset(4, 1);
        run_test_with_dataset(4, 2);
    }

    #[test]
    fn test_day5() {
        run_test_with_dataset(5, 1);
        run_test_with_dataset(5, 2);
    }

    #[test]
    fn test_day6() {
        run_test_with_dataset(6, 1);
        run_test_with_dataset(6, 2);
    }

    #[test]
    fn test_day7() {
        run_test_with_dataset(7, 1);
        run_test_with_dataset(7, 2);
    }

    #[test]
    fn test_day8() {
        run_test_with_dataset(8, 1);
        run_test_with_dataset(8, 2);
    }

    #[test]
    fn test_day9() {
        run_test_with_dataset(9, 1);
        run_test_with_dataset(9, 2);
    }

    #[test]
    fn test_day10() {
        run_test_with_dataset(10, 1);
        run_test_with_dataset(10, 2);
    }

    #[test]
    fn test_day11() {
        run_test_with_dataset(11, 1);
        run_test_with_dataset(11, 2);
    }
}
