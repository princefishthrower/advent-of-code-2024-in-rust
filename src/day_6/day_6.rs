use std::collections::HashSet;

use crate::utils::read_as_matrix::read_as_matrix;

#[derive(PartialEq, Clone, Debug, Hash, Eq)]  // Added Hash, Eq for HashSet
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]  // Added Hash, Eq for HashSet
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct GuardState {
    position: Point,
    direction: Direction,
}

pub fn run_a() -> std::io::Result<()> {
    let input = read_as_matrix("src/day_6/input.txt");
    let (guard_position, guard_direction) = find_guard_initial_position_and_direction(&input);
    let mut guard_history = Vec::new();
    guard_history.push(guard_position.clone());
    
    simulate_guard_path(&input, guard_position, guard_direction, &mut guard_history);

    println!("Total unique points visited by the guard: {}", guard_history.len());
    Ok(())
}

pub fn run_b() -> std::io::Result<()> {
    let mut input = read_as_matrix("src/day_6/input.txt");
    let (guard_position, guard_direction) = find_guard_initial_position_and_direction(&input);
    
    // First, get the guard's original path
    let mut original_history = Vec::new();
    let mut states = HashSet::new();
    simulate_guard_path_with_loop_detection(
        &input,
        guard_position.clone(),
        guard_direction.clone(),
        &mut original_history,
        &mut states,
    );

    // Find all positions adjacent to the guard's path
    let mut adjacent_positions = HashSet::new();
    for pos in original_history {
        // Check all 4 adjacent positions
        let adjacents = [
            Point { x: pos.x + 1, y: pos.y },
            Point { x: pos.x - 1, y: pos.y },
            Point { x: pos.x, y: pos.y + 1 },
            Point { x: pos.x, y: pos.y - 1 },
        ];

        for adj in adjacents.iter() {
            // Skip if out of bounds or not empty
            if !is_within_bounds(adj, &input) {
                continue;
            }
            if input[adj.x as usize][adj.y as usize] != '.' {
                continue;
            }
            // Skip guard's starting position
            if adj.x == guard_position.x && adj.y == guard_position.y {
                continue;
            }
            adjacent_positions.insert(adj.clone());
        }
    }

    let mut possible_positions = Vec::new();
    // Now only test positions adjacent to the original path
    for pos in adjacent_positions {
        // Place obstacle
        input[pos.x as usize][pos.y as usize] = '#';

        // Check if this creates a loop
        let mut history = Vec::new();
        let mut states = HashSet::new();
        if let Some(_) = simulate_guard_path_with_loop_detection(
            &input,
            guard_position.clone(),
            guard_direction.clone(),
            &mut history,
            &mut states,
        ) {
            possible_positions.push(pos.clone());
        }

        // Remove obstacle for next iteration
        input[pos.x as usize][pos.y as usize] = '.';
    }

    println!("Found {} possible positions for the obstacle", possible_positions.len());

    Ok(())
}

fn simulate_guard_path_with_loop_detection(
    matrix: &Vec<Vec<char>>,
    start_pos: Point,
    start_direction: Direction,
    history: &mut Vec<Point>,
    states: &mut HashSet<GuardState>
) -> Option<(Point, Point)> {  // Returns Some((loop_start, loop_end)) if loop found
    let mut current_pos = start_pos;
    let mut current_direction = start_direction;

    loop {
        let current_state = GuardState {
            position: current_pos.clone(),
            direction: current_direction.clone(),
        };

        // If we've seen this state before, we've found a loop
        if !states.insert(current_state) {
            return Some((current_pos.clone(), current_pos.clone()));
        }

        let next_pos = get_next_position(&current_pos, &current_direction);
        
        // Check if we're out of bounds
        if !is_within_bounds(&next_pos, matrix) {
            return None;  // No loop found, guard leaves the area
        }

        // Check if there's an obstruction
        let has_obstruction = matrix[next_pos.x as usize][next_pos.y as usize] == '#';

        if has_obstruction {
            current_direction = turn_right(&current_direction);
        } else {
            current_pos = next_pos;
            history.push(current_pos.clone());
        }
    }
}

fn simulate_guard_path(matrix: &Vec<Vec<char>>, start_pos: Point, start_direction: Direction, history: &mut Vec<Point>) {
    let mut current_pos = start_pos;
    let mut current_direction = start_direction;

    loop {
        // Check if there's an obstruction in front
        let next_pos = get_next_position(&current_pos, &current_direction);
        
        // Check if we're out of bounds
        if !is_within_bounds(&next_pos, matrix) {
            break;
        }

        // Check if there's an obstruction
        let has_obstruction = matrix[next_pos.x as usize][next_pos.y as usize] == '#';

        if has_obstruction {
            // Turn right
            current_direction = turn_right(&current_direction);
        } else {
            // Move forward
            current_pos = next_pos;
            if !history.contains(&current_pos) {
                history.push(current_pos.clone());
            }
        }
    }
}

fn get_next_position(current: &Point, direction: &Direction) -> Point {
    match direction {
        Direction::Up => Point { x: current.x - 1, y: current.y },
        Direction::Down => Point { x: current.x + 1, y: current.y },
        Direction::Left => Point { x: current.x, y: current.y - 1 },
        Direction::Right => Point { x: current.x, y: current.y + 1 },
    }
}

fn turn_right(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn is_within_bounds(pos: &Point, matrix: &Vec<Vec<char>>) -> bool {
    pos.x >= 0 && 
    pos.y >= 0 && 
    pos.x < matrix.len() as i32 && 
    pos.y < matrix[0].len() as i32
}

fn find_guard_initial_position_and_direction(matrix: &Vec<Vec<char>>) -> (Point, Direction) {
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            match matrix[i][j] {
                '^' => return (Point { x: i as i32, y: j as i32 }, Direction::Up),
                '>' => return (Point { x: i as i32, y: j as i32 }, Direction::Right),
                'v' => return (Point { x: i as i32, y: j as i32 }, Direction::Down),
                '<' => return (Point { x: i as i32, y: j as i32 }, Direction::Left),
                _ => continue,
            }
        }
    }
    panic!("No guard found in input matrix!");
}