use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &str, size: usize, num_bytes: usize) -> Vec<Vec<bool>> {
    let mut grid = vec![vec![false; size]; size];
    
    for line in input.lines().take(num_bytes) {
        let coords: Vec<usize> = line
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
            
        if coords.len() == 2 {
            grid[coords[1]][coords[0]] = true;
        }
    }
    
    grid
}

fn get_neighbors(pos: (usize, usize), size: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let (row, col) = pos;
    
    if row > 0 { neighbors.push((row - 1, col)); }
    if row < size - 1 { neighbors.push((row + 1, col)); }
    if col > 0 { neighbors.push((row, col - 1)); }
    if col < size - 1 { neighbors.push((row, col + 1)); }
    
    neighbors
}

fn find_shortest_path(grid: &Vec<Vec<bool>>, size: usize) -> Option<u32> {
    let start = (0, 0);
    let goal = (size - 1, size - 1);
    
    if grid[start.0][start.1] || grid[goal.0][goal.1] {
        return None;
    }
    
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    
    heap.push(State { cost: 0, position: start });
    visited.insert(start);
    
    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }
        
        for next_pos in get_neighbors(position, size) {
            if visited.contains(&next_pos) || grid[next_pos.0][next_pos.1] {
                continue;
            }
            
            visited.insert(next_pos);
            heap.push(State {
                cost: cost + 1,
                position: next_pos,
            });
        }
    }
    
    None
}

fn solve_with_bytes(input: &str, size: usize, num_bytes: usize) -> Option<u32> {
    let grid = parse_input(input, size, num_bytes);
    find_shortest_path(&grid, size)
}

fn find_blocking_byte(input: &str, size: usize) -> Option<(usize, usize)> {
    let mut prev_num_bytes = 1;
    
    // Keep checking bytes until we find one that blocks the path
    while let Some(_) = solve_with_bytes(input, size, prev_num_bytes) {
        prev_num_bytes += 1;
    }
    
    // Get the coordinates of the blocking byte from the input
    input.lines()
        .nth(prev_num_bytes - 1)
        .and_then(|line| {
            let coords: Vec<usize> = line
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            
            if coords.len() == 2 {
                Some((coords[0], coords[1]))
            } else {
                None
            }
        })
}

// define const for grid width and height
const INPUT_PATH: &str = "src/day_18/input.txt";
const GRID_SIZE: usize = 71;
const NUM_BYTES_FALLEN: usize = 1024;

// for examples
// const INPUT_PATH: &str = "src/day_18/input_example.txt";
// const GRID_SIZE: usize = 7;
// const NUM_BYTES_FALLEN: usize = 12;

pub fn run_a() -> std::io::Result<()> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    
    // Use first 1024 bytes for part A
    if let Some(steps) = solve_with_bytes(&input, GRID_SIZE, NUM_BYTES_FALLEN) {
        println!("Minimum steps needed: {}", steps);
    } else {
        println!("No path found!");
    }
    
    Ok(())
}

pub fn run_b() -> std::io::Result<()> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    
    if let Some((x, y)) = find_blocking_byte(&input, GRID_SIZE) {
        println!("{},{}", x, y);
    } else {
        println!("No blocking byte found!");
    }

    Ok(())
}