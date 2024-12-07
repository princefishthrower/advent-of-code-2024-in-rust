use std::fs;

pub fn read_as_matrix(filename: &str) -> Vec<Vec<char>> {
    fs::read_to_string(filename)
        .expect("Failed to read input file")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}