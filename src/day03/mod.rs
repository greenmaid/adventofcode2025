use crate::tools;

const CURRENT_DAY : &str = "day03";

fn get_input_lines() -> Vec<String> {
    tools::read_lines(CURRENT_DAY)
}

pub fn solve1() -> usize {
    let input_lines = get_input_lines();
    solve(&input_lines, 2)
}

pub fn solve2() -> usize {
    let input_lines = get_input_lines();
    solve(&input_lines, 12)
}

fn solve(input_lines: &Vec<String>, digits_count: usize) -> usize {
    let mut result = 0;
    for line in input_lines.iter() {
        let chars: Vec<char> = line.chars().collect();
        let mut best_num_str: Vec<char> = vec!['0'; digits_count];
        let mut index = 0;
        for k in 0..digits_count {
            let (digit, idx) = get_best_digit(&chars[index..chars.len()-digits_count+k+1].to_vec());
            index += idx + 1;
            best_num_str[k] = digit;
        }
        let best_num_str = best_num_str.iter().collect::<String>();
        let best_num = best_num_str.parse::<usize>().unwrap();
        // println!("Best num: {:?}", best_num);
        result += best_num;
    };
    result
}


fn  get_best_digit(charts: &Vec<char>) -> (char, usize) {
    let best_char = *charts.iter().max().unwrap();
    let index = charts.iter().position(|&r| r == best_char).unwrap() as usize;
    return (best_char, index);
}
