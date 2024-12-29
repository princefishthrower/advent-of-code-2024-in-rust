use crate::utils::read_as_columns::read_as_columns;

use std::collections::HashSet;

#[derive(Debug)]
enum SchematicType {
    Lock,
    Key,
}

#[derive(Debug)]
struct Schematic {
    heights: Vec<usize>,
    schematic_type: SchematicType,
}

impl Schematic {
    fn from_str(input: &str) -> Option<Self> {
        let lines: Vec<&str> = input.trim().lines().collect();
        if lines.is_empty() {
            return None;
        }

        // Determine if this is a lock (starts with #) or key (starts with .)
        let schematic_type = if lines[0].starts_with('#') {
            SchematicType::Lock
        } else {
            SchematicType::Key
        };

        let width = lines[0].len();
        let height = lines.len();
        let mut heights = Vec::with_capacity(width);

        // Process each column
        for col in 0..width {
            let column_height = match schematic_type {
                SchematicType::Lock => {
                    // For locks, count from top until we hit a '.'
                    lines.iter()
                        .position(|line| line.chars().nth(col) == Some('.'))
                        .unwrap_or(height)
                },
                SchematicType::Key => {
                    // For keys, count from bottom until we hit a '.'
                    lines.iter()
                        .rev()
                        .position(|line| line.chars().nth(col) == Some('.'))
                        .unwrap_or(height)
                }
            };
            heights.push(column_height);
        }

        Some(Schematic {
            heights,
            schematic_type,
        })
    }
}

fn parse_input(input: &str) -> (Vec<Schematic>, Vec<Schematic>) {
    let schematics: Vec<Schematic> = input
        .split("\n\n")
        .filter_map(Schematic::from_str)
        .collect();

    // Separate locks and keys
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for schematic in schematics {
        match schematic.schematic_type {
            SchematicType::Lock => locks.push(schematic),
            SchematicType::Key => keys.push(schematic),
        }
    }

    (locks, keys)
}

fn check_overlap(lock: &Schematic, key: &Schematic, total_height: usize) -> bool {
    lock.heights.iter().zip(key.heights.iter())
        .any(|(&lock_height, &key_height)| lock_height + key_height > total_height)
}

fn count_valid_pairs(locks: &[Schematic], keys: &[Schematic], total_height: usize) -> usize {
    let mut valid_pairs = HashSet::new();

    for (lock_idx, lock) in locks.iter().enumerate() {
        for (key_idx, key) in keys.iter().enumerate() {
            if !check_overlap(lock, key, total_height) {
                // Store unique pair using indices
                valid_pairs.insert((lock_idx, key_idx));
            }
        }
    }

    valid_pairs.len()
}

// make a const for the total height
const TOTAL_HEIGHT: usize = 7;

pub fn run_a() -> std::io::Result<()> {
    // read input
    let input = include_str!("../../src/day_25/input.txt");

    // parse input
    let (locks, keys) = parse_input(input);
    
    // count valid pairs
    let valid_pairs = count_valid_pairs(&locks, &keys, TOTAL_HEIGHT);

    // print result
    println!("Number of valid lock/key pairs: {}", valid_pairs);

    Ok(())
}

pub fn run_b() -> std::io::Result<()> {
    
    Ok(())
}
