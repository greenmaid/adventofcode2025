use crate::tools;

const CURRENT_DAY : &str = "day02";

fn get_input_text() -> String {
    tools::read_file_as_string(CURRENT_DAY)
}

pub fn solve1() -> i64 {
    let mut result1 = 0;
    let input_text = get_input_text();
    for id_ranges in input_text.split(',') {
        let parts: Vec<&str> = id_ranges.split('-').collect();
        let start: i64 = parts[0].parse().unwrap();
        let end: i64 = parts[1].parse().unwrap();
        for i in start..=end {
            let i_str = format!("{i}");
            if i_str.len() % 2 == 0 {
                let mid = i_str.len() / 2;
                let first_half = &i_str[..mid];
                let second_half = &i_str[mid..];
                if first_half == second_half {
                    // println!("Found double: {}", i_str);
                    result1 += i;
                }
            }
        }
    }
    return result1;
}

pub fn solve2() -> i64 {
    let mut result2 = 0;
    let input_text = get_input_text();
    for id_ranges in input_text.split(',') {
        let parts: Vec<&str> = id_ranges.split('-').collect();
        let start: i64 = parts[0].parse().unwrap();
        let end: i64 = parts[1].parse().unwrap();
        for i in start..=end {
            let i_str = format!("{i}");
            let mid = i_str.len() / 2;
            for k in 1..=mid {
                if i_str.len() % k == 0 {
                    let mut chunks = i_str.as_bytes().chunks(k);
                    let first_chunk = chunks.clone().next().unwrap();
                    if chunks.all(|chunk| chunk == first_chunk) {
                        // println!("Found repeat of size {}: {}", k, i_str);
                        result2 += i;
                        break;
                    }
                }
            }

        }
    }
    return result2;
}

// pub fn solve2() -> i64 {
//     let input_text = get_input_text();
//     input_text
//         .split(',')
//         .flat_map(|id_ranges| {
//             let parts: Vec<&str> = id_ranges.split('-').collect();
//             let start: i64 = parts[0].parse().unwrap();
//             let end: i64 = parts[1].parse().unwrap();
//             (start..=end).filter(|&i| has_repeating_pattern(&i.to_string()))
//         })
//         .sum()
// }

// fn has_repeating_pattern(s: &str) -> bool {
//     let len = s.len();
//     (1..=len / 2).any(|k| {
//         len % k == 0 && {
//             let pattern = &s[..k];
//             s.as_bytes().chunks(k).all(|chunk| chunk == pattern.as_bytes())
//         }
//     })
// }
