mod day01;
mod day02;
mod day03;
mod tools;

use chrono::Datelike;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// day to solve
    #[arg(short = 'd', long)]
    day: Option<u8>,

    /// solve all days
    #[arg(long)]
    all: bool,
}

fn main() {
    println!("Hello, Advent of Code 2025!");

    let args = Args::parse();
    // dbg!(args);

    if args.all {
        for day in 1..=12 {
            print!("Running day {day}...\n");
            run_day(day);
            print!("\n");
        }
    } else {
        let day: u8;
        if let Some(requested_day) = args.day {
            day = requested_day;
        } else {
            day = get_current_day();
        }

        if day > 0 && day <= 12 {
            print!("Running current day {day}...\n");
            run_day(day);
        } else {
            println!("No day specified and no current day available.");
        }
    }
}

fn get_current_day() -> u8 {
    let local = chrono::Local::now();
    let date = local.naive_local();
    if date.year() == 2025 && date.month() == 12 {
        return date.day() as u8;
    }
    0
}

fn run_day(day: u8) {
    match day {
        1 => solve_day01(),
        2 => solve_day02(),
        3 => solve_day03(),
        _ => println!("Day not implemented yet"),
    }
}

fn solve_day01() {
    let res_day01 = day01::solve();
    println!("Day 01, Part 1 solution: {}", res_day01.0);
    println!("Day 01, Part 2 solution: {}", res_day01.1);
}

fn solve_day02() {
    let res_day02_1 = day02::solve1();
    println!("Day 02, Part 1 solution: {}", res_day02_1);
    let res_day02_2 = day02::solve2();
    println!("Day 02, Part 2 solution: {}", res_day02_2);
}

fn solve_day03() {
    let res_day03_1 = day03::solve1();
    println!("Day 03, Part 1 solution: {}", res_day03_1);
    let res_day03_2 = day03::solve2();
    println!("Day 03, Part 2 solution: {}", res_day03_2);
}
