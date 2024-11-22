use clap::Parser;
use itertools::Itertools;
use std::{
    fs::read_to_string,
    path::PathBuf,
    time::{Duration, Instant},
};

pub mod aoclib;
pub mod prelude;

#[cfg(test)]
mod test;

// TODO update daily
pub mod day01;

macro_rules! solution {
    ($day:ident, $part:ident) => {{
        let input = read_to_string(
            ["src", stringify!($day), "input.txt"]
                .into_iter()
                .collect::<PathBuf>(),
        )
        .unwrap();
        let start = Instant::now();
        let result = $day::$part(&input).unwrap();
        let end = Instant::now();
        (result, end - start)
    }};
}

fn main() {
    let cli = Cli::parse();

    let (result, duration) = if cli.all {
        run_all()
    } else {
        get_solution(cli.day, cli.part)
    };

    println!();
    println!("{}", result);
    println!();
    print!("solved in {:?} ", duration);
    #[cfg(debug_assertions)]
    println!("on debug mode");
    #[cfg(not(debug_assertions))]
    println!("on release mode")
}

fn get_solution(day: u8, part: u8) -> (String, Duration) {
    if !(1..=25).contains(&day) || !(1..=2).contains(&part) {
        panic!("invalid format: day or part number invalid")
    }
    match (day, part) {
        // TODO update daily
        (1, 1) => solution!(day01, part1),
        (1, 2) => solution!(day01, part2),

        _ => (
            "This day is not solved by me yet".to_string(),
            Duration::ZERO,
        ),
    }
}

fn run_all() -> (String, Duration) {
    let mut time = Duration::ZERO;
    let mut last_day = None;
    for (day, part) in (1..=25).cartesian_product(1..=2) {
        let t = get_solution(day, part).1;
        if t.is_zero() {
            last_day.get_or_insert(day);
        }
        time += t;
    }

    if last_day == Some(1) {
        return ("didn't solve anything...".to_string(), Duration::ZERO);
    }

    let result = match last_day {
        Some(day) => format!("solved all days up until day {}", day - 1),
        None => "solved all days!".to_string(),
    };

    (result, time)
}

/// aoc 2024 cli
#[derive(Parser, Debug)]
#[command(name = "aoc2024", author, version, long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = 1)]
    pub day: u8,
    #[arg(short, long, default_value_t = 1)]
    pub part: u8,
    #[arg(short, long, exclusive = true)]
    pub all: bool,
}
