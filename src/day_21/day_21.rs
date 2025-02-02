

pub fn run_a() -> std::io::Result<()> {
    let input = include_str!("../../src/day_21/input.txt");

    // chain 3
    let result = solve(input, 2);

    // print result
    println!("Sum of complexities: {}", result);
    
    Ok(())
}

pub fn run_b() -> std::io::Result<()> {
    
    let input = include_str!("../../src/day_21/input.txt");

    // chain 3
    let result = solve(input, 25);

    // print result
    println!("Sum of complexities: {}", result);
    
    Ok(())
}

use itertools::Itertools;
use std::cell::LazyCell;
use std::collections::HashMap;

/*
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+
*/
const NUMERIC_KEYPAD: LazyCell<HashMap<char, (isize, isize)>> = LazyCell::new(|| {
    ["789", "456", "123", " 0A"]
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(move |(x, key)| (key, (x as isize, y as isize)))
        })
        .collect()
});

/*
    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+
*/
const DIRECTIONAL_KEYPAD: LazyCell<HashMap<char, (isize, isize)>> = LazyCell::new(|| {
    [" ^A", "<v>"]
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(move |(x, key)| (key, (x as isize, y as isize)))
        })
        .collect()
});

fn key_path_cost(
    cache: &HashMap<(usize, char, char), usize>,
    robot: usize,
    key_start: char,
    key_end: char,
) -> usize {
    if robot == 0 {
        1
    } else {
        *cache
            .get(&(robot, key_start, key_end))
            .unwrap_or_else(|| panic!("invalid key '{} {} {}'", robot, key_start, key_end))
    }
}

fn keypresses_cost(
    cache: &HashMap<(usize, char, char), usize>,
    robot: usize,
    key_seq: &str,
) -> usize {
    format!("A{key_seq}") // robots always start at 'A'
        .chars()
        .tuple_windows()
        .map(|(key_start, key_end)| key_path_cost(cache, robot, key_start, key_end))
        .sum()
}

fn cache_robot(
    cache: &mut HashMap<(usize, char, char), usize>,
    robot: usize,
    keypad: &HashMap<char, (isize, isize)>,
) {
    for (&key_start, &(x_start, y_start)) in keypad.iter() {
        for (&key_end, &(x_end, y_end)) in keypad.iter() {
            let horizontal_dist = (x_end - x_start).abs() as usize;
            let vertical_dist = (y_end - y_start).abs() as usize;

            let horizontal_keys = if x_end > x_start { ">" } else { "<" }.repeat(horizontal_dist);
            let vertical_keys = if y_end < y_start { "^" } else { "v" }.repeat(vertical_dist);

            let horizontal_key_seq = format!("{horizontal_keys}{vertical_keys}A");
            let vertical_key_seq = format!("{vertical_keys}{horizontal_keys}A");

            let min_horizontal = if (x_end, y_start) != keypad[&' '] {
                keypresses_cost(cache, robot - 1, &horizontal_key_seq)
            } else {
                usize::MAX
            };

            let min_vertical = if (x_start, y_end) != keypad[&' '] {
                keypresses_cost(cache, robot - 1, &vertical_key_seq)
            } else {
                usize::MAX
            };

            cache.insert(
                (robot, key_start, key_end),
                min_horizontal.min(min_vertical),
            );
        }
    }
}

fn cache_robots(n_robots: usize) -> HashMap<(usize, char, char), usize> {
    let mut cache: HashMap<(usize, char, char), usize> = HashMap::new();

    // build cache on top of each other
    for robot in 1..=n_robots {
        cache_robot(&mut cache, robot, &DIRECTIONAL_KEYPAD);
    }

    // last robot
    cache_robot(&mut cache, n_robots + 1, &NUMERIC_KEYPAD);

    cache
}

fn min_keypresses(code: &str, n_robots: usize) -> usize {
    let cache = cache_robots(n_robots);
    keypresses_cost(&cache, n_robots + 1, code)
}

fn solve(input: &str, n_robots: usize) -> usize {
    input
        .lines()
        .map(|line| {
            let keypresses = min_keypresses(line, n_robots);
            let code = line[0..line.len() - 1].parse::<usize>().unwrap();
            code * keypresses
        })
        .sum()
}