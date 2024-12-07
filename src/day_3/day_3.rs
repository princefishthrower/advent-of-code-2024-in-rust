use crate::utils::read_as_string::read_as_string;
use regex::Regex;

pub fn run_a() -> std::io::Result<()> {
    let input = read_as_string("src/day_3/input.txt")?;

    // regex for all mult([0-9]+),([0-9]+) occurrences
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut result = 0;
    for cap in re.captures_iter(&input) {
        let a: i32 = cap[1].parse().unwrap();
        let b: i32 = cap[2].parse().unwrap();

        // calculate the result as we go
        result += a * b;
    }

    println!("Sum of multiplications: {}", result);
    
    Ok(())
}

pub fn run_b() -> std::io::Result<()> {
    let input = read_as_string("src/day_3/input.txt")?;
    
    // Find all command positions and their types
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();
    let re_mul = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    // Create a vector of all command positions with their types
    let mut commands: Vec<(usize, &str)> = vec![];
    
    // Add all commands to our vector with their positions
    for m in re_do.find_iter(&input) {
        commands.push((m.start(), "do"));
    }
    for m in re_dont.find_iter(&input) {
        commands.push((m.start(), "dont"));
    }
    for m in re_mul.find_iter(&input) {
        commands.push((m.start(), "mul"));
    }
    
    // Sort commands by position
    commands.sort_by_key(|&(pos, _)| pos);

    let mut result = 0;
    let mut mul_active = true;  // Start with mul active

    for (pos, cmd_type) in commands {
        match cmd_type {
            "do" => mul_active = true,
            "dont" => mul_active = false,
            "mul" if mul_active => {
                if let Some(cap) = re_mul.captures(&input[pos..]) {
                    let a: i32 = cap[1].parse().unwrap();
                    let b: i32 = cap[2].parse().unwrap();
                    result += a * b;
                }
            }
            _ => {}
        }
    }

    println!("Sum of multiplications between do() and don't(): {}", result);
    Ok(())
}
