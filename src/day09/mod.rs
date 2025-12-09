use crate::tools;

const CURRENT_DAY : &str = "day09";

fn get_input() -> Vec<Vec<usize>> {
    let lines = tools::read_lines(CURRENT_DAY);
    lines.iter()
        .map(|l| {
            l.split(",").map(|s| s.parse::<usize>().unwrap()).collect()
        })
        .collect()
}

fn find_candidates(tiles: Vec<Vec<usize>>) -> (Vec<Vec<usize>>,Vec<Vec<usize>>,Vec<Vec<usize>>,Vec<Vec<usize>>) {

    let mut upper_left_candidates: Vec<Vec<usize>> = vec![];
    let mut upper_right_candidates: Vec<Vec<usize>> = vec![];
    let mut bottom_right_candidates: Vec<Vec<usize>> = vec![];
    let mut bottom_left_candidates: Vec<Vec<usize>> = vec![];

    for t1 in &tiles{
        let mut better_upper_right = false;
        let mut better_upper_left = false;
        let mut better_bottom_left = false;
        let mut better_bottom_right = false;
        for t2 in &tiles {
            if t2 == t1 {continue}
            if !better_upper_right && t1[0] >= t2[0] && t1[1] >= t2[1] {
                better_upper_right = true;
            }
            if !better_upper_left && t1[0] <= t2[0] && t1[1] >= t2[1] {
                better_upper_left = true;
            }
            if !better_bottom_left && t1[0] <= t2[0] && t1[1] <= t2[1] {
                better_bottom_left = true;
            }
            if !better_bottom_right && t1[0] >= t2[0] && t1[1] <= t2[1] {
                better_bottom_right = true;
            }
            if better_upper_right && better_upper_left && better_bottom_left && better_bottom_right {break}
        }
        if ! better_upper_right { upper_right_candidates.push(t1.to_vec())}
        if ! better_upper_left { upper_left_candidates.push(t1.to_vec())}
        if ! better_bottom_left { bottom_left_candidates.push(t1.to_vec())}
        if ! better_bottom_right { bottom_right_candidates.push(t1.to_vec())}
    };

    (upper_left_candidates, bottom_right_candidates, upper_right_candidates, bottom_left_candidates)

}


fn area(tile1: &Vec<usize>, tile2: &Vec<usize>) -> usize {
    let x = (tile1[0] as isize -  tile2[0] as isize).abs() + 1;
    let y = (tile1[1] as isize -  tile2[1] as isize).abs() + 1;
    let result = x * y;
    result as usize
}

pub fn solve1() -> usize {
    let tiles = get_input();
    let (upper_left_candidates, bottom_right_candidates, upper_right_candidates, bottom_left_candidates) = find_candidates(tiles);

    let max1 = upper_left_candidates.iter()
        .flat_map({ |v1|
            bottom_right_candidates.iter().map(|v2| area(v1, v2))
        })
        .max().unwrap();
    let max2 = upper_right_candidates.iter()
        .flat_map({ |v1|
            bottom_left_candidates.iter().map(|v2| area(v1, v2))
        })
        .max().unwrap();
    usize::max(max1,max2)
}

pub fn solve2() -> usize {
    let _input = get_input();
    let result2 = 0;
    return result2;
}
