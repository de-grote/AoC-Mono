#![allow(clippy::type_complexity, mismatched_lifetime_syntaxes)]

use chrono::Datelike;
use clap::{Parser, Subcommand};
use itertools::Itertools;
use std::{
    fs::{self, read_to_string},
    io::Write,
    path::PathBuf,
    time::{Duration, Instant},
};

pub mod aoclib;
pub mod prelude;

#[cfg(test)]
mod test;

// TODO update daily
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

macro_rules! solution {
    ($day:ident, $part:ident) => {{
        let input = read_to_string(
            ["src", stringify!($day), "input.txt"]
                .into_iter()
                .collect::<PathBuf>(),
        )
        .unwrap();
        let start = Instant::now();
        let result = $day::$part(&input);
        let time = start.elapsed();
        (result.unwrap().to_string(), time)
    }};
}

fn main() {
    let _ = dotenvy::dotenv();

    let cli = Cli::parse();

    let now = chrono::Local::now();

    let day = if cli.day != 0 {
        cli.day
    } else if now.month() == 12 {
        now.day() as u8
    } else if let Some(Commands::All) = cli.subcommand {
        0
    } else {
        panic!("can only automatically get date in december")
    };
    let part = cli.part;

    let (result, duration) = match cli.subcommand.unwrap_or_default() {
        Commands::Solve => get_solution(day, part),
        Commands::All => run_all(),
        Commands::FetchInput => {
            let start = Instant::now();
            fetch_input(day);
            (
                format!("downloaded the input file for day {}", day),
                start.elapsed(),
            )
        }
        Commands::FetchAllInput => {
            let start = Instant::now();
            let days = fetch_all_input(day);
            (
                if days.is_empty() {
                    "downloaded no input files".to_string()
                } else {
                    format!(
                        "downloaded the input files for days: {}",
                        days.into_iter().join(" ")
                    )
                },
                start.elapsed(),
            )
        }
    };

    println!();
    println!("{}", result);
    println!();
    print!("solved in {:?} ", duration);
    #[cfg(debug_assertions)]
    println!("on debug mode");
    #[cfg(not(debug_assertions))]
    println!("on release mode");
}

fn get_solution(day: u8, part: u8) -> (String, Duration) {
    if !(1..=12).contains(&day) || !(1..=2).contains(&part) {
        panic!("invalid format: day or part number invalid")
    }
    match (day, part) {
        // TODO update daily
        (1, 1) => solution!(day01, part1),
        (1, 2) => solution!(day01, part2),
        (2, 1) => solution!(day02, part1),
        (2, 2) => solution!(day02, part2),
        (3, 1) => solution!(day03, part1),
        (3, 2) => solution!(day03, part2),
        (4, 1) => solution!(day04, part1),
        (4, 2) => solution!(day04, part2),

        _ => (
            "This day is not solved by me yet".to_string(),
            Duration::ZERO,
        ),
    }
}

fn run_all() -> (String, Duration) {
    let mut time = Duration::ZERO;
    let mut last_day = None;
    for (day, part) in (1..=12).cartesian_product(1..=2) {
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

fn fetch_input(day: u8) {
    let session = std::env::var("SESSION").expect("no session token in env");
    let buf = ureq::get(&format!("https://adventofcode.com/2025/day/{day}/input"))
        .header("Cookie", &format!("session={}", session))
        .call()
        .expect("couldn't reach aoc server")
        .into_body()
        .read_to_vec()
        .expect("couldn't convert input to string");
    let day_str = day_to_string(day);
    fs::create_dir_all(format!("src/day{}", day_str))
        .expect("couldnt create directory in src folder");
    let mut file =
        fs::File::create(format!("src/day{}/input.txt", day_str)).expect("couldn't create file");
    file.write_all(&buf).expect("couldn't write file");
}

fn day_to_string(day: u8) -> String {
    if day <= 9 {
        format!("0{}", day)
    } else {
        day.to_string()
    }
}

fn fetch_all_input(last_day: u8) -> Vec<u8> {
    let mut res = Vec::new();
    for day in 1..=last_day {
        if let Ok(false) = fs::exists(format!("src/day{}/input.txt", day_to_string(day))) {
            res.push(day);
            fetch_input(day);
        }
    }
    res
}

/// aoc 2024 cli
#[derive(Parser, Debug)]
#[command(name = "aoc2024", author, version, long_about = None, subcommand_required = false)]
struct Cli {
    #[arg(short, long, default_value_t = 0)]
    pub day: u8,
    #[arg(short, long, default_value_t = 1)]
    pub part: u8,
    #[command(subcommand)]
    pub subcommand: Option<Commands>,
}

#[derive(Debug, Subcommand, Default, PartialEq, Eq)]
enum Commands {
    #[default]
    /// Solve a day
    Solve,
    /// Solve all days
    All,
    /// Fetch input and load it in, does replace existing files
    FetchInput,
    /// Fetch all inputs until the specified day and load it in, does not replace existing files
    FetchAllInput,
}
