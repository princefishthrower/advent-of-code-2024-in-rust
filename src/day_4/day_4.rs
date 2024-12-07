use crate::utils::read_as_matrix::read_as_matrix;

pub fn run_a() -> std::io::Result<()> {
    let input = read_as_matrix("src/day_4/input.txt");
    let xmas_count = find_xmas(&input);
    println!("XMAS appears {} times in the word search", xmas_count);
    Ok(())
}

pub fn run_b() -> std::io::Result<()> {
    let input = read_as_matrix("src/day_4/input_example.txt");
    let xmas_count = find_x_mas(&input);
    println!("X-MAS appears {} times in the word search", xmas_count);
    Ok(())
}

fn find_xmas(grid: &Vec<Vec<char>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let target = "XMAS".chars().collect::<Vec<char>>();
    let directions = [
        (0, 1),   // right
        (1, 0),   // down
        (1, 1),   // diagonal down-right
        (-1, 1),  // diagonal up-right
        (0, -1),  // left
        (-1, 0),  // up
        (-1, -1), // diagonal up-left
        (1, -1),  // diagonal down-left
    ];

    let mut count = 0;

    for i in 0..rows {
        for j in 0..cols {
            // Try each direction from this starting point
            for &(dx, dy) in &directions {
                let mut valid = true;
                let mut positions = Vec::new();

                // Check if we can find XMAS in this direction
                for k in 0..4 {
                    let new_i = i as i32 + dx * k as i32;
                    let new_j = j as i32 + dy * k as i32;

                    if new_i < 0
                        || new_i >= rows as i32
                        || new_j < 0
                        || new_j >= cols as i32
                        || grid[new_i as usize][new_j as usize] != target[k]
                    {
                        valid = false;
                        break;
                    }
                    positions.push((new_i as usize, new_j as usize));
                }

                if valid {
                    count += 1;
                }
            }
        }
    }

    count
}

fn find_x_mas(grid: &Vec<Vec<char>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    // Directions representing diagonals:
    // "\" diagonals: (1,1) and (-1,-1)
    // "/" diagonals: (1,-1) and (-1,1)
    let backslash_diags = [(1, 1), (-1, -1)];
    let slash_diags = [(1, -1), (-1, 1)];

    // Helper function to check a single diagonal line around 'A'
    // It checks cells (i - dx, j - dy) and (i + dx, j + dy).
    // Valid patterns: M - A - S or S - A - M
    fn check_mas_line(grid: &Vec<Vec<char>>, i: usize, j: usize, dx: i32, dy: i32) -> bool {
        let rows = grid.len() as i32;
        let cols = grid[0].len() as i32;
        let i = i as i32;
        let j = j as i32;

        let x1 = i - dx;
        let y1 = j - dy;
        let x2 = i + dx;
        let y2 = j + dy;

        // Check bounds
        if x1 < 0 || x1 >= rows || y1 < 0 || y1 >= cols {
            return false;
        }
        if x2 < 0 || x2 >= rows || y2 < 0 || y2 >= cols {
            return false;
        }

        let first = grid[x1 as usize][y1 as usize];
        let second = grid[x2 as usize][y2 as usize];

        // Check if pattern is MAS or SAM
        (first == 'M' && second == 'S') || (first == 'S' && second == 'M')
    }

    // For each cell, consider it as the center 'A' of an X-MAS pattern
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] != 'A' {
                continue;
            }

            // For each combination of one "\" diagonal and one "/" diagonal
            for &(dx1, dy1) in &backslash_diags {
                for &(dx2, dy2) in &slash_diags {
                    let line1 = check_mas_line(grid, i, j, dx1, dy1);
                    let line2 = check_mas_line(grid, i, j, dx2, dy2);

                    if line1 && line2 {
                        count += 1;
                    }
                }
            }
        }
    }

    count / 4
}
