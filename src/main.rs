use std::fs;
use std::path::PathBuf;

use clap::Parser;

mod year2023;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct App {
    #[arg(long, short)]
    year: u64,

    #[arg(long, short)]
    day: u64,

    #[arg(long, short)]
    part: u64,

    #[arg(long, short)]
    input: PathBuf,
}

fn main() {
    let args = App::parse();

    let input = fs::read_to_string(args.input).expect("read input from file");

    match args.year {
        2023 => {
            let output = year2023::run(args.day, args.part, input);
            println!("Output:\n{output}");
        }
        y => panic!("Unknown year: {y}"),
    };
}
