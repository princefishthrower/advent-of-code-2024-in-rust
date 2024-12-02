use std::io::{self};
use std::path::Path;
use crate::utils::read_lines::read_lines;

pub fn read_as_rows<P>(filename: P) -> io::Result<Vec<Vec<String>>>
where
    P: AsRef<Path>,
{
    let lines = read_lines(filename)?;
    let mut rows: Vec<Vec<String>> = Vec::new();
    
    // Process each line into its own vector
    for line in lines {
        let values: Vec<String> = line.split_whitespace().map(String::from).collect();
        rows.push(values);
    }
    
    Ok(rows)
}