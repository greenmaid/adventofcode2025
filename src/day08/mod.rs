use std::{collections::HashSet, collections::HashMap};
use itertools::Itertools;

use crate::tools;

const CURRENT_DAY : &str = "day08";

fn get_input() -> HashSet<(usize, usize, usize)> {
    let lines = tools::read_lines(CURRENT_DAY);
    let mut junctions: HashSet<(usize, usize, usize)> = HashSet::new();
    for l in lines {
        let coords: Vec<usize> = l.split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        junctions.insert((coords[0], coords[1], coords[2]));
    };
    junctions
}

fn calculate_distance(x1: i32, y1: i32,z1: i32, x2: i32, y2: i32,z2: i32) -> f64 {
    let x = (x1-x2) as i64;
    let y = (y1-y2) as i64;
    let z = (z1-z2) as i64;
    f64::sqrt((x.pow(2) + y.pow(2) + z.pow(2)) as f64)
}

fn top_pairs(junctions: &HashSet<(usize, usize, usize)>, top: usize) -> Vec<(usize, &(usize, usize, usize), &(usize, usize, usize))>{

    let mut pairs: Vec<_> = junctions.into_iter()
        .combinations(2)
        .map(|p| {
            let j1 = p[0];
            let j2 = p[1];
            let dist = calculate_distance(j1.0 as i32, j1.1 as i32, j1.2 as i32, j2.0 as i32, j2.1 as i32, j2.2 as i32) as usize;
            (dist, j1, j2)
        })
        .collect();
    pairs.sort_by_key(|&p| p.0);
    if top > 0 {
        pairs.truncate(top);
    }
    pairs
}

fn manage_wires(junctions: &HashSet<(usize, usize, usize)>, pairs:  Vec<(usize, &(usize, usize, usize), &(usize, usize, usize))>)
      -> (HashMap<usize, HashSet<(usize, usize, usize)>>, ((usize, usize, usize), (usize, usize, usize))) {

    let mut junc_group: HashMap<(usize, usize, usize), usize> = HashMap:: new();
    let mut groups: HashMap<usize, HashSet<(usize, usize, usize)>> = HashMap:: new();
    let mut next_group_id = 1;

    let mut last = ((0,0,0), (0,0,0));

    for (_, &junc1, &junc2) in pairs {

        match (junc_group.get(&junc1), junc_group.get(&junc2)) {
            (None, None) => {
                junc_group.insert(junc1, next_group_id);
                junc_group.insert(junc2, next_group_id);
                groups.insert(next_group_id, HashSet::from_iter(vec![junc1, junc2]));
                next_group_id += 1;
            },
            (Some(&i), None) => {
                junc_group.insert(junc2, i);
                groups.entry(i).and_modify(|v| {v.insert(junc2);});
            },
            (None, Some(&i)) => {
                junc_group.insert(junc1, i);
                groups.entry(i).and_modify(|v| {v.insert(junc1);});
            },
            (Some(&i), Some(&j)) => {
                if i != j {
                    let to_be_moved = groups.get(&j).unwrap().clone();
                    for junc in to_be_moved {
                        junc_group.insert(junc, i);
                        groups.entry(i).and_modify(|v| {v.insert(junc);});
                    };
                    groups.remove(&j);
                }
            }
        };

        // exit condition for part 2
        if groups.len() == 1 {
            let expected = junctions.len();
            let group_len = groups.iter().next().unwrap().1.len();
            if expected == group_len {
                last = (junc1, junc2);
                break
            }
        }
    }

    (groups, last)
}

pub fn solve1() -> usize {
    let max_connections =  match tools::is_test_mode() {
        true => 10,
        false => 1000,
    };

    let junctions = get_input();
    let pairs = top_pairs(&junctions, max_connections);
    let (groups, _) = manage_wires(&junctions, pairs);

    let mut sorted_groups: Vec<&HashSet<(usize, usize, usize)>> = groups.iter()
        .map(|(_,v)| v )
        .collect();
    sorted_groups.sort_by_key(|h| h.len());

    let result1 = (0..3)
        .map(|_| sorted_groups.pop().unwrap().len())
        .product();

    return result1;
}

pub fn solve2() -> usize {
    let junctions = get_input();
    let pairs = top_pairs(&junctions, 0);
    let (_, last) = manage_wires(&junctions, pairs);

    let result2 = last.0.0 * last.1.0;
    return result2;
}
