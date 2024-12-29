use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug)]
struct ParseError;

#[derive(Debug)]
struct TowelPatterns {
    allowed_patterns: Vec<String>,
    onsen_designs: Vec<String>,
}

impl FromStr for TowelPatterns {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut sections = input.trim().split("\n\n");
        
        // Parse allowed patterns
        let allowed_patterns = sections
            .next()
            .ok_or(ParseError)?
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        // Parse onsen designs
        let onsen_designs = sections
            .next()
            .ok_or(ParseError)?
            .lines()
            .map(|s| s.trim().to_string())
            .collect();

        Ok(TowelPatterns {
            allowed_patterns,
            onsen_designs,
        })
    }
}

pub fn run_a() -> std::io::Result<()> {
    let input = fs::read_to_string(Path::new("./src/day_19/input.txt"))?;
    let patterns = TowelPatterns::from_str(&input)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Parse error"))?;
    
    // Count possible patterns
    let mut possible_count = 0;
    for design in patterns.onsen_designs.iter() {
        if !is_pattern_impossible(design, &patterns.allowed_patterns) {
            possible_count += 1;
        }
    }
    
    println!("Number of possible patterns: {}", possible_count);
    
    Ok(())
}

pub fn run_b() -> std::io::Result<()> {
    let input = fs::read_to_string(Path::new("./src/day_19/input.txt"))?;
    let patterns = TowelPatterns::from_str(&input)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Parse error"))?;
    
    let mut total_combinations = 0;    
    for design in patterns.onsen_designs.iter() {        
        let combinations = get_number_of_combinations_for_pattern(design, &patterns.allowed_patterns);
        if combinations > 0 {
            total_combinations += combinations;
        }
    }
    
    println!("Total number of combinations across all patterns: {}", total_combinations);
    
    Ok(())
}

fn is_pattern_impossible(pattern: &str, allowed_patterns: &[String]) -> bool {
    let mut dp = vec![false; pattern.len() + 1];
    dp[0] = true;  // empty string is always possible

    // For each position in the pattern
    for i in 0..=pattern.len() {
        // Skip if we can't build up to this position
        if !dp[i] {
            continue;
        }

        // Try each allowed pattern at current position
        for allowed in allowed_patterns {
            if i + allowed.len() <= pattern.len() {
                // Check if this pattern matches at this position
                if &pattern[i..i + allowed.len()] == allowed {
                    dp[i + allowed.len()] = true;
                }
            }
        }
    }

    // If we can't reach the end, it's impossible
    !dp[pattern.len()]
}

fn get_number_of_combinations_for_pattern(pattern: &str, allowed_patterns: &[String]) -> u64 {
    // Using a HashMap to store number of ways to reach each position
    let mut dp: HashMap<usize, u64> = HashMap::new();
    dp.insert(0, 1); // One way to make empty string
    
    // For each position
    for i in 0..=pattern.len() {
        // Skip positions we can't reach
        if !dp.contains_key(&i) {
            continue;
        }
        
        let ways_to_here = dp[&i];
        
        // Try each allowed pattern at this position
        for allowed in allowed_patterns {
            if i + allowed.len() <= pattern.len() {
                // Check if this pattern matches at this position
                if &pattern[i..i + allowed.len()] == allowed {
                    let end_pos = i + allowed.len();
                    *dp.entry(end_pos).or_insert(0) += ways_to_here;
                }
            }
        }
    }
    
    // Return number of ways to reach the end (or 0 if impossible)
    *dp.get(&pattern.len()).unwrap_or(&0)
}