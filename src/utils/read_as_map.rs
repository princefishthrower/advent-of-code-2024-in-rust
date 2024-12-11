use std::fs;
use std::collections::HashMap;

pub fn read_as_map(filename: &str) -> (HashMap<(usize, usize), char>, (usize, usize, usize, usize)) {
    let contents = fs::read_to_string(filename)
        .expect("Failed to read input file");

    let lines: Vec<&str> = contents.lines().collect();

    let mut map = HashMap::new();

    // Determine the height and width boundaries
    let height = lines.len();
    let mut max_width = 0;

    for (y, line) in lines.iter().enumerate() {
        if line.len() > max_width {
            max_width = line.len();
        }
        for (x, ch) in line.chars().enumerate() {
            map.insert((x, y), ch);
        }
    }

    // Boundaries: since indexing starts at 0, min_x and min_y are 0
    let min_x = 0;
    let max_x = if max_width > 0 { max_width - 1 } else { 0 };
    let min_y = 0;
    let max_y = if height > 0 { height - 1 } else { 0 };

    (map, (min_x, max_x, min_y, max_y))
}
