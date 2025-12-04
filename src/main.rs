mod day01;
mod day02;
mod day03;
mod day04;
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

    /// use test data
    #[arg(long, short = 't')]
    test_mode: bool,
}

fn main() {
    println!("Hello, Advent of Code 2025!");

    let args = Args::parse();
    // dbg!(args);

    if args.test_mode {
        unsafe {
            std::env::set_var("TEST_MODE", "true");
        }
    }

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
        4 => solve_day04(),
        _ => println!("Day not implemented yet"),
    }
}

fn print_solution(day: u8, part: impl std::fmt::Display, solution: impl std::fmt::Display) {
    println!("Day {day:02}, Part {part} solution: {solution}");
}

fn solve_day01() {
    let (res1, res2) = day01::solve();
    print_solution(1, 1, res1);
    print_solution(1, 2, res2);
}

fn solve_day02() {
    let part1 = day02::solve1();
    print_solution(2, 1, part1);
    let part2 = day02::solve2();
    print_solution(2, 2, part2);
}

fn solve_day03() {
    let part1 = day03::solve1();
    print_solution(3, 1, part1);
    let part2 = day03::solve2();
    print_solution(3, 2, part2);
}

fn solve_day04() {
    let part1 = day04::solve1();
    print_solution(4, 1, part1);
    let part2 = day04::solve2();
    print_solution(4, 2, part2);
}
