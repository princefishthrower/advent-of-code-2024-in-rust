use std::io::{self};
use std::path::Path;
use crate::utils::read_lines::read_lines;

pub fn read_as_string<P>(filename: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let lines = read_lines(filename)?;
    
    let mut full_string = String::new();
    for line in lines {
        full_string.push_str(&line);
    }
    
    Ok(full_string)
}