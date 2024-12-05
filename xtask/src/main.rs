use anyhow::Result;
use clap::{Parser, Subcommand};
use console::style;
use duct::cmd;
use std::fmt::Debug;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    action: Action,
}

#[derive(Debug, Subcommand)]
enum Action {
    Run {
        #[arg(short, long, default_value_t = 2024)]
        year: usize,
        #[arg(short, long)]
        day: usize,
        #[arg(short, long)]
        part: usize,
    },
    /// Run linting tools on the codebase
    Lint,
    /// Install required development tools
    InstallTools,
    /// Run tests
    Test,
    /// Format the code
    Fmt,
}

fn fmt() -> Result<()> {
    println!("{}", style("cargo +nightly fmt").bold());
    cmd!("cargo", "+nightly", "fmt").run()?;
    Ok(())
}

fn check_fmt() -> Result<()> {
    println!("{}", style("cargo +nightly fmt --check").bold());
    cmd!("cargo", "+nightly", "fmt", "--check").run()?;
    Ok(())
}

fn clippy() -> Result<()> {
    println!(
        "{}",
        style("cargo clippy --all-targets --all-features").bold()
    );
    cmd!("cargo", "clippy", "--all-targets", "--all-features").run()?;
    Ok(())
}

fn unit_test() -> Result<()> {
    println!("{}", style("cargo nextest run --all-features").bold());
    cmd!("cargo", "nextest", "run", "--all-features").run()?;
    Ok(())
}

fn run(year: usize, day: usize, part: usize) -> Result<()> {
    let mut i = 0;
    loop {
        let input = format!("./dataset/{year}/day{day}/part{part}/{i}.input");
        if !Path::new(&input).exists() {
            break;
        }
        let v = format!("cargo run -- --year {year} --day {day} --part {part} --input {input}");
        println!("{}", style(v).bold());
        cmd!(
            "cargo",
            "run",
            "--",
            "--year",
            year.to_string(),
            "--day",
            day.to_string(),
            "--part",
            part.to_string(),
            "--input",
            input
        )
        .run()?;
        i += 1;
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.action {
        Action::Run { year, day, part } => run(year, day, part)?,
        Action::InstallTools => {
            println!("{}", style("cargo install cargo-nextest").bold());
            cmd!("cargo", "install", "cargo-nextest", "--locked").run()?;
        }
        Action::Lint => {
            check_fmt()?;
            clippy()?;
        }
        Action::Test => {
            unit_test()?;
        }
        Action::Fmt => {
            fmt()?;
        }
    }

    Ok(())
}
