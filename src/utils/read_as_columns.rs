use std::io::{self};
use std::path::Path;
use crate::utils::read_lines::read_lines;

pub fn read_as_columns<P>(filename: P) -> io::Result<Vec<Vec<String>>>
where
    P: AsRef<Path>,
{
    let lines = read_lines(filename)?;
    let mut columns: Vec<Vec<String>> = Vec::new();
    
    // Process each line
    for line in lines {
        let values: Vec<String> = line.split_whitespace().map(String::from).collect();
        
        // Initialize columns if this is the first line
        if columns.is_empty() {
            columns = vec![Vec::new(); values.len()];
        }
        
        // Add each value to its respective column
        for (i, value) in values.into_iter().enumerate() {
            if i < columns.len() {
                columns[i].push(value);
            }
        }
    }
    
    Ok(columns)
}