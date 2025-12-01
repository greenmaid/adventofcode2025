mod tools;
mod day01;
fn main() {
    println!("Hello, Advent of Code 2025!");
    let res_day01 = day01::solve();
    println!("Day 01, Part 1 solution: {}", res_day01.0);
    println!("Day 01, Part 2 solution: {}", res_day01.1);
}
