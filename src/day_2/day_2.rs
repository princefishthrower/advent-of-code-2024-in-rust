use crate::utils::read_as_rows::read_as_rows;

pub fn run_a() -> std::io::Result<()> {
    // read the input file into rows
    let rows = read_as_rows("src/day_2/input.txt")?;
    let rows_int: Vec<Vec<i32>> = rows.iter().map(|row| row.iter().map(|s| s.parse::<i32>().unwrap()).collect()).collect();

    // count the number of safe rows
    let mut safe_rows = 0;
    for row in rows_int {
        let is_safe_result = is_safe(&row);
        if is_safe_result {
            safe_rows += 1;
        }
    }

    println!("Number of safe rows: {}", safe_rows);

    Ok(())
}

pub fn run_b() -> std::io::Result<()> {
    // read the input file into rows
    let rows = read_as_rows("src/day_2/input.txt")?;
    let rows_int: Vec<Vec<i32>> = rows.iter().map(|row| row.iter().map(|s| s.parse::<i32>().unwrap()).collect()).collect();

    // count the number of safe rows, but also those that are safe with one element removed
    let mut safe_rows = 0;
    for row in rows_int {
        let is_safe_result = is_safe(&row);
        if is_safe_result {
            safe_rows += 1;
        } else {
            let is_safe_with_problem_dampener_result = is_safe_with_problem_dampener(&row);
            if is_safe_with_problem_dampener_result {
                safe_rows += 1;
            }
        }
    }

    println!("Number of safe rows: {}", safe_rows);

    Ok(())
}

fn is_safe(row: &Vec<i32>) -> bool {
    // Check if monotonically increasing
    let mut increasing = true;
    for i in 1..row.len() {
        if row[i] <= row[i-1] || row[i] - row[i-1] > 3 {
            increasing = false;
            break;
        }
    }
    
    // Check if monotonically decreasing
    let mut decreasing = true;
    for i in 1..row.len() {
        if row[i] >= row[i-1] || row[i-1] - row[i] > 3 {
            decreasing = false;
            break;
        }
    }
    
    // Must be either monotonically increasing or all decreasing
    increasing || decreasing
}

fn is_safe_with_problem_dampener(row: &Vec<i32>) -> bool {
    // check if is_safe with any one element removed
    for i in 0..row.len() {
        let mut row_copy = row.clone();
        row_copy.remove(i);
        if is_safe(&row_copy) {
            return true;
        }
    }
    return false;
}