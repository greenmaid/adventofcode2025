use crate::tools;
use std::cmp;

const CURRENT_DAY : &str = "day05";

fn get_input() -> (Vec<(isize, isize)>, Vec<isize>) {
    let text = tools::read_file_as_string(CURRENT_DAY);
    let splitted: Vec<&str> = text.split("\n\n").collect();

    let fresh_ranges: Vec<(isize, isize)> = splitted[0].split("\n")
        .map(|s| {
            let parts: Vec<&str> = s.split("-").collect();
            let start: isize = parts[0].parse().unwrap();
            let end: isize = parts[1].parse().unwrap();
            (start, end)
            }
        ).collect();

    let ingredients: Vec<isize> = splitted[1].split("\n")
        .map(|s| {
            s.parse().unwrap()
            }
        ).collect();

    (fresh_ranges, ingredients)
}

pub fn solve1() -> usize {
    let (fresh_ranges, ingredients) = get_input();
    ingredients.iter()
        .filter( |&ing| {
            fresh_ranges.iter().any(|fr| *ing >= fr.0 && *ing <= fr.1)
            })
        .count()
}

pub fn solve2() -> usize {
    let (mut fresh_ranges, _) = get_input();

    fresh_ranges.sort_by_key(|&(a, _)| a);

    let mut index = 1;
    loop {
        if index == fresh_ranges.len() {break};
        if fresh_ranges[index].0 >  fresh_ranges[index-1].1 {
            index += 1;
            continue
        }
        fresh_ranges[index-1].1 = cmp::max(fresh_ranges[index-1].1, fresh_ranges[index].1);
        fresh_ranges.remove(index);
    }

    let result: isize = fresh_ranges.iter()
        .map( |&(a,b)| b - a + 1 )
        .sum();
    result as usize

}
