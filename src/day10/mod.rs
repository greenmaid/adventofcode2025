use crate::tools;
use regex::Regex;
use itertools::Itertools;
use microlp::{Problem, OptimizationDirection, ComparisonOp};

const CURRENT_DAY : &str = "day10";

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
struct Machine {
    lights: Vec<char>,
    expected_lights: Vec<char>,
    buttons: Vec<Vec<u32>>,
    joltage: Vec<u32>,
    expected_joltage: Vec<u32>,
}

impl Machine {
    pub fn new(expected_lights: Vec<char>, buttons: Vec<Vec<u32>>, expected_joltage: Vec<u32>) -> Self {
        let lights = expected_lights.iter().map(|_| '.').collect();
        let joltage = expected_joltage.iter().map(|_| 0).collect();
        Machine {
            lights,
            expected_lights,
            buttons,
            joltage,
            expected_joltage,
        }
    }

    pub fn push_button_for_init(&mut self, button_index: usize) {
        let switched_lights = &self.buttons[button_index];
        for &light_index in switched_lights {
            if self.lights[light_index as usize] == '#' {
                self.lights[light_index as usize] = '.';
            } else {
                self.lights[light_index as usize] = '#';
            }
        }
    }

    pub fn push_button_for_joltage(&mut self, button_index: usize, count: u32) {
        let increasing_gauges = &self.buttons[button_index];
        for _ in 0..count {
            for &g in increasing_gauges {
                self.joltage[g as usize] += 1;
            }
        }
    }

    pub fn is_joltage_ok(&self) -> i32 {
        let diff: Vec<i32> = self.joltage.iter().zip(self.expected_joltage.iter())
            .map(|(&it, &expected)| {expected as i32 - it as i32}).collect();

        if diff.iter().all(|&d| d == 0) {return 0;}  // joltage ok
        if diff.iter().any(|&d| d < 0) {return 1;}   // over joltage
        if diff.iter().any(|&d| d > 0) {return -1;}  // under joltage
        panic!("Unreachable state in is_joltage_ok");
    }

    pub fn is_init_ok(&self) -> bool {
        self.lights == self.expected_lights
    }
}

fn get_input() -> Vec<Machine> {
    let lines = tools::read_lines(CURRENT_DAY);
    let indicator_lights_re = Regex::new(r"^\[([\.\#]+)\].*$").unwrap();
    let buttons_re = Regex::new(r"\([0-9,]+\)").unwrap();
    let joltage_re = Regex::new(r"^.*\{(.*)\}$").unwrap();

    let machines = lines.iter().map(|l| {
        let lights: Vec<char> = indicator_lights_re.captures(l).unwrap()[1].chars().collect();
        let buttons = buttons_re.captures_iter(l)
            .map(|cap| {
                cap[0][1..cap[0].len()-1]
                    .split(',')
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .sorted_by_key(|b| b.len())
            .collect::<Vec<Vec<u32>>>();
        let joltage: Vec<u32> = joltage_re.captures(l).unwrap()[1]
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        Machine::new(lights, buttons, joltage)
    }).collect();
    return machines
}

fn find_init_sequence(machine: &Machine) -> Vec<usize> {
    let button_count = machine.buttons.len();
    let mut pushed = 1;
    loop {
        let possible_sequences = (0..button_count).combinations(pushed as usize)
            .collect::<Vec<Vec<usize>>>();
        for sequence in possible_sequences {
            let mut test_machine = machine.clone();
            for &button_index in &sequence {
                test_machine.push_button_for_init(button_index);
            }
            if test_machine.is_init_ok() {
                if tools::is_test_mode() { print!("Found correct sequence {:?}, lights: {:?}\n", sequence, test_machine.lights); }
                return sequence
            }
        }
        pushed += 1;
    }
}

fn find_joltage_sequence(machine: &Machine) -> usize {

    // solving using a linear solver (rust microlp here)
    let mut problem = Problem::new(OptimizationDirection::Minimize);

    let &max = machine.expected_joltage.iter().max().unwrap();
    let variables: Vec<_> = machine.buttons.iter().enumerate()
        .map(|_| problem.add_integer_var(1.0, (0, max as i32)))
        .collect();

    for (idx,&v) in machine.expected_joltage.iter().enumerate() {
        let j = idx as u32;
        let mut constraint = vec![];
        for (i, b) in machine.buttons.iter().enumerate() {
            match b.contains(&j) {
                true  => constraint.push((variables[i], 1.0)),
                false => (),
            }
        }
        // println!("Adding contraint: {:?} = {v}", constraint);
        problem.add_constraint(&constraint, ComparisonOp::Eq, v as f64);
    }

    let solution = problem.solve().unwrap();

    let answers: Vec<_> = variables.iter()
        .map(|&var| solution[var])
        .map(|f| f.round() as usize)
        .collect();

    // check
    // println!("Check solution for machine {:?} {:?}", machine.buttons, machine.expected_joltage);
    // let mut test_machine = machine.clone();
    // for (button_index, &count) in answers.iter().enumerate() {
    //     test_machine.push_button_for_joltage(button_index, count as u32);
    // }
    // assert_eq!(machine.expected_joltage, test_machine.joltage);

    answers.iter().sum()

}

pub fn solve1() -> usize {
    let machines = get_input();
    machines.iter().map(|m| find_init_sequence(&m).len()).sum()
}

pub fn solve2() -> usize {
    let machines = get_input();
    let result = machines.iter()
        .map(|m| find_joltage_sequence(m))
        .sum();
    result
}
