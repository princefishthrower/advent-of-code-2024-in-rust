use std::collections::HashMap;

use crate::utils::read_lines_as_int_arrays::read_lines_as_int_arrays;

pub fn run_a() -> std::io::Result<()> {
    let input = read_lines_as_int_arrays("src/day_11/input.txt", " ")?;

    // input is just a single line, so take the first element
    let initial_stone_engravings = input[0].clone();

    // convert the initial stone engravings to a vector of i64
    let initial_stone_engravings: Vec<i64> = initial_stone_engravings.iter().map(|&x| x as i64).collect();

    // execute blink 25 times on the initial stone engravings
    let mut stone_engravings = initial_stone_engravings.clone();
    for _ in 0..25 {
        stone_engravings = execute_blink(stone_engravings);
    }

    // log the count of stones
    println!("Count of stones after 25 blinks: {}", stone_engravings.len());

    Ok(())
}

pub fn run_b() -> std::io::Result<()> {
    let input = read_lines_as_int_arrays("src/day_11/input.txt", " ")?;

    // input is just a single line, so take the first element
    let initial_stones = input[0].clone();

    // convert the initial stone engravings to a vector of i64
    let initial_stones: Vec<i64> = initial_stones.iter().map(|&x| x as i64).collect();
    let blinks = 75;

    // Create memoization map
    let mut memo = HashMap::new();
    
    // Calculate total for each initial stone
    let total: usize = initial_stones.iter()
        .map(|&stone| {
            let count = count_stones_recursive(stone, blinks, &mut memo);
            count
        })
        .sum();

    println!("\nTotal stones after {} blinks: {}", blinks, total);

    Ok(())
}


fn execute_blink(stone_engravings: Vec<i64>) -> Vec<i64> {
    let mut new_stones = Vec::new();

    for stone in stone_engravings {
        if stone == 0 {
            // Rule 1: stone = 0 -> replaced by a stone marked 1
            new_stones.push(1);
        } else {
            let stone_str = stone.to_string();
            let stone_len = stone_str.len();

            if stone_len % 2 == 0 {
                // Rule 2: stone has even number of digits -> split into two stones
                let half = stone_len / 2;
                let left_stone_str = &stone_str[0..half];
                let right_stone_str = &stone_str[half..];

                let left_stone = left_stone_str.parse::<i64>().unwrap();
                let right_stone = right_stone_str.parse::<i64>().unwrap();

                new_stones.push(left_stone);
                new_stones.push(right_stone);
            } else {
                // Rule 3: otherwise, multiply by 2024
                new_stones.push(stone * 2024);
            }
        }
    }

    new_stones
}

type MemoKey = (i64, usize);  // (stone_value, blinks_remaining)
type MemoMap = HashMap<MemoKey, usize>;

fn count_stones_recursive(stone: i64, blinks: usize, memo: &mut MemoMap) -> usize {
    // Base case - no more blinks
    if blinks == 0 {
        return 1;
    }

    // Check if we've seen this (stone, blinks) combination before
    let key = (stone, blinks);
    if let Some(&count) = memo.get(&key) {
        return count;
    }

    // Calculate the result based on the rules
    let result = if stone == 0 {
        // Rule 1: 0 becomes 1
        count_stones_recursive(1, blinks - 1, memo)
    } else {
        let digit_count = stone.to_string().len();
        if digit_count % 2 == 0 {
            // Rule 2: even number of digits - split into two
            let stone_str = stone.to_string();
            let half = digit_count / 2;
            let left = stone_str[..half].parse::<i64>().unwrap();
            let right = stone_str[half..].parse::<i64>().unwrap();
            // Each half contributes to the total
            count_stones_recursive(left, blinks - 1, memo) +
            count_stones_recursive(right, blinks - 1, memo)
        } else {
            // Rule 3: multiply by 2024
            count_stones_recursive(stone * 2024, blinks - 1, memo)
        }
    };

    // Memoize and return
    memo.insert(key, result);
    result
}