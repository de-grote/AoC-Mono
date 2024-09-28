use std::time;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

fn main() {
    let before = time::SystemTime::now();
    day10::part2::solve().expect("invalid input");
    let after = time::SystemTime::now();
    println!("solved in: {:?}", after.duration_since(before).unwrap());
}