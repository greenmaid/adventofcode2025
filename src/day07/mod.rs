use crate::tools;
use std::collections::HashSet;
use std::collections::HashMap;

const CURRENT_DAY : &str = "day07";

fn get_input() -> ((usize, usize), HashSet<(usize, usize)>, HashMap<usize, HashSet<usize>>) {
    let grid = tools::read_as_grid(CURRENT_DAY);
    let mut splitters_coord: HashSet<(usize, usize)> = HashSet::new();
    let mut splitters_y: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut start = (0, 0);

    for y in 0..grid.len() {
        let mut h = HashSet::new();
        for x in 0..grid[0].len() {
            let cell = grid[y][x];
            match cell {
                '^' => {
                    splitters_coord.insert((x, y));
                    h.insert(x);
                },
                'S' => {start = (x, y);},
                _ => {}
            }
        }
        splitters_y.insert(y, h);
    };
    (start, splitters_coord, splitters_y)
}

pub fn solve1() -> usize {
    let (start, _, splitters) = get_input();
    let mut beams: HashSet<usize> = [start.0].into_iter().collect();
    let mut splits = 0;

    for y in start.1+1..splitters.len() {
        let mut new_beams = beams.clone();
        let current_splitters = &splitters[&y];
        if tools::is_test_mode() {
            println!("{y} - {beams:?} - {current_splitters:?}");
        }
        for &x in &beams {
            match current_splitters.contains(&x) {
                true => {
                    new_beams.insert(x+1);
                    new_beams.insert(x-1);
                    new_beams.remove(&x);
                    splits += 1;
                },
                false => (),
            }
        }
        beams = new_beams;
    }
    return splits;
}



fn split_beams(x: usize, y: usize, splitters: &HashSet<(usize, usize)>, current: usize, max: usize, cache: & mut HashMap<(usize, usize), usize>) -> usize {

    if cache.contains_key(&(x,y)) {
        let result =  cache.get(&(x,y)).unwrap();
        return *result + current
    }

    let mut next_splitter: Option<usize> = None;
    for i in (y+1)..=max {
        if splitters.contains(&(x,i)) {
            next_splitter = Some(i);
            break
        }
    }

    let mut result = current + 1;
    match next_splitter {
        None => (),
        Some(ny ) => {
            result = split_beams(x+1, ny, splitters, current, max, cache);
            if x != 0 {
                result = split_beams(x-1, ny, splitters, result, max, cache);
            }
        }
    };
    cache.insert((x,y), result-current);
    result
}

pub fn solve2() -> usize {
    let (start, splitters, y) = get_input();
    let max = y.len();
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
    let result = split_beams(start.0, start.1, &splitters, 0, max, &mut cache);
    return result;
}

