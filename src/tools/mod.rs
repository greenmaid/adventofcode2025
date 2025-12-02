use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_file_path(day: &str) -> String {
    let test_mode: String = std::env::var("TEST_MODE").unwrap_or("".to_string());

    match test_mode.as_str() {
        "" => format!("src/{}/data.txt", day),
        _ => format!("src/{}/test-data.txt", day),
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
