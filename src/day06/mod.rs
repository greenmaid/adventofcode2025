use crate::tools;

const CURRENT_DAY : &str = "day06";

fn get_input() -> Vec<String> {
    let lines = tools::read_lines(CURRENT_DAY);
    lines
}

pub fn solve1() -> usize {
    let lines = get_input();

    let table: Vec<Vec<&str>> = lines.iter()
        .map(|l| l.split_ascii_whitespace().collect::<Vec<&str>>())
        .collect();

    let ops: Vec<String> = table[table.len()-1].iter()
        .map(|s| s.to_string())
        .collect();

    let numbers: Vec<Vec<isize>> = table[..table.len()-1].iter()
        .map(
            |v| v.iter().map(|s| s.parse::<isize>().unwrap()).collect()
        ).collect();

    let result: isize = (0..=ops.len()-1).map( |i|
        match ops[i].as_str() {
            "*" => numbers.iter().map(|v| v[i]).product(),
            "+" => numbers.iter().map(|v| v[i]).sum(),
            &_ => 0 as isize,
        }
    ).sum();
    result as usize
}

pub fn solve2() -> usize {
    let lines = get_input();
    let ops: Vec<&str> = lines[lines.len()-1].split_ascii_whitespace().collect::<Vec<&str>>();
    let max_length = lines.iter().map(|l| l.len()).max().unwrap();

    let numbers_str: Vec<String>= (0..=max_length-1).map( |idx| {
        (0..=lines.len()-2)
            .map(|i| lines[i].chars().nth(idx).unwrap_or(' '))
            .filter(|&c| c != ' ')
            .collect()
    }).collect();

    let mut result2 = 0;
    let mut nums = numbers_str.iter();
    for op in ops {
        let mut v: Vec<usize> = vec![];
        while let Some(s) = nums.next() {
            match s.parse::<usize>() {
                Ok(n) => v.push(n),
                Err(_)  => break,
            }
        }
        // dbg!(&v);
        let sub_result = match op {
            "*" => v.iter().product(),
            "+" => v.iter().sum(),
            &_ => 0 as usize,
        };
        // dbg!(sub_result);

        result2 += sub_result

    };

    return result2;
}
