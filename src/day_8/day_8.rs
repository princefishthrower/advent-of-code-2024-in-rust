use crate::utils::read_as_matrix::read_as_matrix;

#[derive(PartialEq, Clone, Debug, Hash, Eq)]  // Added Hash, Eq for HashSet
struct Point {
    x: i32,
    y: i32,
}

pub fn run_a() -> std::io::Result<()> {
    let matrix = read_as_matrix("src/day_8/input.txt");

    let mut unique_colocation_points = Vec::<Point>::new();
    // for a-z, A-Z, and 0-9, find the location of each antenna and the unique colocation points
    for antennae_char in 'a'..='z' {
        let antenna_locations = get_location_of_antennae_with_letter(&matrix, antennae_char);
        let new_unique_colocation_points = get_unique_collinearity_points_within_matrix(&matrix, &antenna_locations, &unique_colocation_points);
        unique_colocation_points.extend(new_unique_colocation_points);
    }
    for antennae_char in 'A'..='Z' {
        let antenna_locations = get_location_of_antennae_with_letter(&matrix, antennae_char);
        let new_unique_colocation_points = get_unique_collinearity_points_within_matrix(&matrix, &antenna_locations, &unique_colocation_points);
        unique_colocation_points.extend(new_unique_colocation_points);
    }
    for antennae_char in '0'..='9' {
        let antenna_locations = get_location_of_antennae_with_letter(&matrix, antennae_char);
        let new_unique_colocation_points = get_unique_collinearity_points_within_matrix(&matrix, &antenna_locations, &unique_colocation_points);
        unique_colocation_points.extend(new_unique_colocation_points);
    }

    // print length of unique colocation points
    println!("Number of unique colocation points: {}", unique_colocation_points.len());
    
    Ok(())
}

pub fn run_b() -> std::io::Result<()> {
    let matrix = read_as_matrix("src/day_8/input.txt");

    let mut unique_harmonic_points = Vec::<Point>::new();
    // for a-z, A-Z, and 0-9, find the location of each antenna and the unique colocation points
    for antennae_char in 'a'..='z' {
        let antenna_locations = get_location_of_antennae_with_letter(&matrix, antennae_char);
        let new_unique_colocation_points = get_unique_resonant_harmonic_points_within_matrix(&matrix, &antenna_locations, &unique_harmonic_points);
        unique_harmonic_points.extend(new_unique_colocation_points);
    }
    for antennae_char in 'A'..='Z' {
        let antenna_locations = get_location_of_antennae_with_letter(&matrix, antennae_char);
        let new_unique_colocation_points = get_unique_resonant_harmonic_points_within_matrix(&matrix, &antenna_locations, &unique_harmonic_points);
        unique_harmonic_points.extend(new_unique_colocation_points);
    }
    for antennae_char in '0'..='9' {
        let antenna_locations = get_location_of_antennae_with_letter(&matrix, antennae_char);
        let new_unique_colocation_points = get_unique_resonant_harmonic_points_within_matrix(&matrix, &antenna_locations, &unique_harmonic_points);
        unique_harmonic_points.extend(new_unique_colocation_points);
    }

    // print length of unique colocation points
    println!("Number of unique resonate harmonic points: {}", unique_harmonic_points.len());
    
    Ok(())
}

fn get_location_of_antennae_with_letter(matrix: &Vec<Vec<char>>, letter: char) -> Vec<Point> {
    let mut locations = Vec::new();
    for (y, row) in matrix.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == letter {
                locations.push(Point { x: x as i32, y: y as i32 });
            }
        }
    }
    locations
}

fn get_unique_collinearity_points_within_matrix(matrix: &Vec<Vec<char>>, points: &Vec<Point>, existing_points: &Vec<Point>) -> Vec<Point> {
    // a collinear point is a point that is on the same line as two other points, but d1 from one and d2 from the other
    // if the calculated collinear point is outside the matrix bounds, it is not a valid collinear point
    // we also need to check if the collinear point is already in the existing points list, then we continue to the next point
    let mut unique_collinear_points = Vec::new();
    for (i, point1) in points.iter().enumerate() {
        for (j, point2) in points.iter().enumerate() {
            if i == j {
                continue;
            }
            let d1 = point1.x - point2.x;
            let d2 = point1.y - point2.y;
            let collinear_point = Point { x: point1.x + d1, y: point1.y + d2 };
            if collinear_point.x < 0 || collinear_point.x >= matrix[0].len() as i32 || collinear_point.y < 0 || collinear_point.y >= matrix.len() as i32 {
                continue;
            }
            if existing_points.contains(&collinear_point) {
                continue;
            }
            unique_collinear_points.push(collinear_point);
        }
    }
    unique_collinear_points
}

fn get_unique_resonant_harmonic_points_within_matrix(
    matrix: &Vec<Vec<char>>, 
    points: &Vec<Point>, 
    existing_points: &Vec<Point>
) -> Vec<Point> {
    use std::collections::HashSet;
    let mut unique_resonant_harmonic_points = HashSet::new();
    
    for (i, point1) in points.iter().enumerate() {
        for (j, point2) in points.iter().enumerate() {
            if i == j {
                continue;
            }

            let dx = point1.x - point2.x;
            let dy = point1.y - point2.y;

            // If dx and dy are both zero, skip
            if dx == 0 && dy == 0 {
                continue;
            }

            // Reduce (dx, dy) to its simplest form to ensure we generate every harmonic point
            let gcd = gcd(dx.abs() as i32, dy.abs() as i32);
            let step_x = dx / gcd;
            let step_y = dy / gcd;

            // Explore forward direction
            let mut k = 1;
            loop {
                let new_x = point1.x + step_x * k;
                let new_y = point1.y + step_y * k;
                if new_x < 0 || new_x >= matrix[0].len() as i32 || new_y < 0 || new_y >= matrix.len() as i32 {
                    break; // out of bounds
                }
                let new_point = Point { x: new_x, y: new_y };
                if existing_points.contains(&new_point) || unique_resonant_harmonic_points.contains(&new_point) {
                    // Already counted
                    k += 1;
                    continue;
                }
                unique_resonant_harmonic_points.insert(new_point);
                k += 1;
            }

            // Explore backward direction (if desired)
            // Depending on definition, you may want to also go "behind" point1
            // For instance:
            let mut k = 1;
            loop {
                let new_x = point1.x - step_x * k;
                let new_y = point1.y - step_y * k;
                if new_x < 0 || new_x >= matrix[0].len() as i32 || new_y < 0 || new_y >= matrix.len() as i32 {
                    break; // out of bounds
                }
                let new_point = Point { x: new_x, y: new_y };
                if existing_points.contains(&new_point) || unique_resonant_harmonic_points.contains(&new_point) {
                    // Already counted
                    k += 1;
                    continue;
                }
                unique_resonant_harmonic_points.insert(new_point);
                k += 1;
            }
        }
    }

    unique_resonant_harmonic_points.into_iter().collect()
}

// Helper function to compute gcd
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}