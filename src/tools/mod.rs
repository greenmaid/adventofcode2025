use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_lines(day: &str) -> Vec<String> {

    let test_mode: String = std::env::var("TEST_MODE").unwrap_or("".to_string());

    let file_path: String = match test_mode.as_str() {
        "" => format!("src/{}/data.txt", day),
        _ => format!("src/{}/test-data.txt", day),
    };

    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect()
}
