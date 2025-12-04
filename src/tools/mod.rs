use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};


pub fn is_test_mode() -> bool {
    match std::env::var("TEST_MODE").unwrap_or("".to_string()).as_str() {
        "true" => true,
        _      => false,
    }
}

fn get_file_path(day: &str) -> String {
    if is_test_mode() {
        format!("src/{}/test-data.txt", day)
    } else {
        format!("src/{}/data.txt", day)
    }
}

pub fn read_lines(day: &str) -> Vec<String> {

    let file_path = get_file_path(day);

    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect()
}

pub fn read_file_as_string(day: &str) -> String {

    let file_path = get_file_path(day);
    let content = fs::read_to_string(file_path.as_str()).expect("Failed to read file");
    content.trim().to_string()
}

pub fn read_as_grid(day: &str) -> Vec<Vec<char>> {
    let lines = read_lines(day);
    let result = lines.iter()
        .map(|s| s.chars().collect())
        .collect();
    result
}
