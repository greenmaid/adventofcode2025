use crate::tools;

const CURRENT_DAY : &str = "day01";

fn get_input_lines() -> Vec<String> {
    tools::read_lines(CURRENT_DAY)
}

pub fn solve() -> ( i32, i32) {
    let input_lines = get_input_lines();
    let mut dial_loc = 50;
    let mut result1 = 0;
    let mut result2 = 0;

    for line in input_lines {
        let chars: Vec<char> = line.chars().collect();

        let direction = match chars[0]{
            'R' => 1,
            'L' => -1,
            _ => 0
        };

        let offset = chars[1..].iter().collect::<String>().parse::<i32>().unwrap();

        let new = dial_loc + (direction * offset);

        if new >= 100 {
            result2 += new / 100;
        }
        if new <= 0 {
            result2 += -(new / 100);
            if dial_loc > 0 {
                result2 += 1;
            }
        }
        dial_loc = new.rem_euclid(100);

        if dial_loc == 0 {
            result1 += 1;
        }
    }

    return (result1, result2);
}
