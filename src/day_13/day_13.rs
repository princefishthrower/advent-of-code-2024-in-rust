pub fn run_a() -> std::io::Result<()> {
    let input = read_game_data("src/day_13/input.txt")?;
    println!("Total number of games: {}", input.len());

    let valid_games = find_valid_games(input);
    println!("Number of valid games: {}", valid_games.len());

    let tokens_needed = min_tokens_needed(valid_games);
    println!("Minimum number of tokens needed: {}", tokens_needed);

    Ok(())
}

pub fn run_b() -> std::io::Result<()> {
    let input = read_game_data("src/day_13/input.txt")?;
    println!("Total number of games: {}", input.len());

    let valid_games = find_valid_games_part2(input);
    println!("Number of valid games: {}", valid_games.len());

    let tokens_needed = min_tokens_needed_part2(valid_games);
    println!("Minimum number of tokens needed: {}", tokens_needed);
    
    Ok(())
}

use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Entry {
    button_a: (i32, i32),
    button_b: (i32, i32),
    prize: (i32, i32),
}

fn read_game_data(path: &str) -> io::Result<Vec<Entry>> {
    let reader = BufReader::new(File::open(path)?);
    let mut entries = Vec::new();
    let mut current = [0; 6];
    let mut idx = 0;

    for line in reader.lines() {
        let line = line?.trim().to_string();
        if line.is_empty() { continue; }
        
        if let Some(coords) = line.split_once(':') {
            let nums: Vec<i32> = coords.1
                .split(',')
                .filter_map(|s| s.trim()
                    .strip_prefix('X')
                    .map(|s| s.trim_start_matches(|c| c == '=' || c == '+'))
                    .and_then(|s| s.parse().ok())
                    .or_else(|| s.trim()
                        .strip_prefix('Y')
                        .map(|s| s.trim_start_matches(|c| c == '=' || c == '+'))
                        .and_then(|s| s.parse().ok())))
                .collect();

            if nums.len() == 2 {
                current[idx] = nums[0];
                current[idx + 1] = nums[1];
                idx += 2;

                if idx == 6 {
                    entries.push(Entry {
                        button_a: (current[0], current[1]),
                        button_b: (current[2], current[3]),
                        prize: (current[4], current[5]),
                    });
                    idx = 0;
                }
            }
        }
    }
    Ok(entries)
}

fn find_valid_games(entries: Vec<Entry>) -> Vec<Entry> {
    entries.into_iter()
        .filter(|entry| {
            let (a_x, a_y) = entry.button_a;
            let (b_x, b_y) = entry.button_b;
            let (p_x, p_y) = entry.prize;
            
            // Each button can be pressed up to 100 times
            for i in 0..=100 {
                for j in 0..=100 {
                    if i * a_x + j * b_x == p_x && i * a_y + j * b_y == p_y {
                        return true;
                    }
                }
            }
            false
        })
        .collect()
}

fn min_tokens_needed(entries: Vec<Entry>) -> i32 {
    entries.into_iter()
        .map(|entry| {
            let (a_x, a_y) = entry.button_a;
            let (b_x, b_y) = entry.button_b;
            let (p_x, p_y) = entry.prize;
            
            let mut min_tokens = i32::MAX;
            
            // Try combinations up to 100 presses per button
            for i in 0..=100 {
                for j in 0..=100 {
                    if i * a_x + j * b_x == p_x && i * a_y + j * b_y == p_y {
                        // A costs 3 tokens, B costs 1 token
                        let tokens = i * 3 + j * 1;
                        min_tokens = min_tokens.min(tokens);
                    }
                }
            }
            
            min_tokens
        })
        .sum()
}

// Part 2 specific functions
const OFFSET: i64 = 10_000_000_000_000;

fn min_tokens_needed_part2(entries: Vec<Entry>) -> i64 {
    entries.into_iter()
        .map(|entry| {
            let (x1, y1) = (entry.button_a.0 as f64, entry.button_a.1 as f64);
            let (x2, y2) = (entry.button_b.0 as f64, entry.button_b.1 as f64);
            let (x3, y3) = (
                (entry.prize.0 as f64) + OFFSET as f64,
                (entry.prize.1 as f64) + OFFSET as f64
            );
            
            // Solve the system of equations:
            // x1 * a + x2 * b = x3
            // y1 * a + y2 * b = y3
            
            let a = (x3 * (x2 - y2) - x2 * (x3 - y3)) / (x1 * (x2 - y2) + x2 * (y1 - x1));
            let b = (x3 - x1 * a) / x2;
            
            // Check if we have integer solutions
            if a.floor() == a && b.floor() == b && a >= 0.0 && b >= 0.0 {
                // Calculate tokens needed: 3 tokens for button A, 1 token for button B
                (a * 3.0 + b).round() as i64
            } else {
                0 // Invalid solution
            }
        })
        .sum()
}

// You'll also need to modify these related functions:

fn find_valid_games_part2(entries: Vec<Entry>) -> Vec<Entry> {
    entries.into_iter()
        .filter(|entry| {
            let (x1, y1) = (entry.button_a.0 as f64, entry.button_a.1 as f64);
            let (x2, y2) = (entry.button_b.0 as f64, entry.button_b.1 as f64);
            let (x3, y3) = (
                (entry.prize.0 as f64) + OFFSET as f64,
                (entry.prize.1 as f64) + OFFSET as f64
            );
            
            let a = (x3 * (x2 - y2) - x2 * (x3 - y3)) / (x1 * (x2 - y2) + x2 * (y1 - x1));
            let b = (x3 - x1 * a) / x2;
            
            a.floor() == a && b.floor() == b && a >= 0.0 && b >= 0.0
        })
        .collect()
}