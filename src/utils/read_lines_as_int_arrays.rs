use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn read_lines_as_int_arrays<P>(filename: P, sep: &str) -> io::Result<Vec<Vec<i32>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<Result<String, io::Error>> = reader.lines().collect();
    let mut int_arrays: Vec<Vec<i32>> = Vec::new();
    // split each line by the passed in sep and parse each element as an integer
    for line in lines {
        let line: String = line?;
        let int_array: Vec<i32> = line.split(sep).map(|x| x.parse::<i32>().unwrap()).collect();
        int_arrays.push(int_array);
    }
    Ok(int_arrays)
}