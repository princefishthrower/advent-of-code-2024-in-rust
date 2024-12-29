use crate::utils::read_as_matrix::read_as_matrix;
use std::collections::{BinaryHeap, HashSet, HashMap};
use std::cmp::Ordering;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_clockwise(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn turn_counterclockwise(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn get_delta(&self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    score: u32,
    row: usize,
    col: usize,
    direction: Direction,
    path: Vec<(usize, usize)>, // Track the path taken
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
            .then_with(|| self.row.cmp(&other.row))
            .then_with(|| self.col.cmp(&other.col))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_start_end(grid: &Vec<Vec<char>>) -> ((usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                start = (i, j);
            } else if cell == 'E' {
                end = (i, j);
            }
        }
    }
    (start, end)
}

fn move_forward(state: &State, grid: &Vec<Vec<char>>) -> Option<State> {
    let rows = grid.len();
    let cols = grid[0].len();
    let delta = state.direction.get_delta();
    let new_row = state.row as i32 + delta.0;
    let new_col = state.col as i32 + delta.1;
    
    if new_row >= 0 && new_row < rows as i32 && 
       new_col >= 0 && new_col < cols as i32 {
        let new_row = new_row as usize;
        let new_col = new_col as usize;
        
        if grid[new_row][new_col] != '#' {
            let mut new_path = state.path.clone();
            new_path.push((new_row, new_col));
            Some(State {
                score: state.score + 1,
                row: new_row,
                col: new_col,
                direction: state.direction,
                path: new_path,
            })
        } else {
            None
        }
    } else {
        None
    }
}

fn turn_clockwise(state: &State) -> State {
    let mut new_path = state.path.clone();
    new_path.push((state.row, state.col));
    State {
        score: state.score + 1000,
        row: state.row,
        col: state.col,
        direction: state.direction.turn_clockwise(),
        path: new_path,
    }
}

fn turn_counterclockwise(state: &State) -> State {
    let mut new_path = state.path.clone();
    new_path.push((state.row, state.col));
    State {
        score: state.score + 1000,
        row: state.row,
        col: state.col,
        direction: state.direction.turn_counterclockwise(),
        path: new_path,
    }
}

fn solve_maze_all_optimal_paths(grid: &Vec<Vec<char>>) -> (Option<u32>, HashSet<(usize, usize)>) {
    let (start, end) = find_start_end(grid);
    let mut heap = BinaryHeap::new();
    let mut best_scores = HashMap::new();
    let mut optimal_tiles = HashSet::new();
    let mut min_end_score = None;
    
    let initial_state = State {
        score: 0,
        row: start.0,
        col: start.1,
        direction: Direction::East,
        path: vec![(start.0, start.1)],
    };
    
    heap.push(initial_state);
    best_scores.insert((start.0, start.1, Direction::East), 0);
    
    while let Some(current) = heap.pop() {
        // If this path is already worse than a known path to this state, skip it
        if let Some(&best) = best_scores.get(&(current.row, current.col, current.direction)) {
            if current.score > best {
                continue;
            }
        }
        
        // If we've reached the end
        if (current.row, current.col) == end {
            match min_end_score {
                None => {
                    min_end_score = Some(current.score);
                    // Add all tiles from this optimal path
                    optimal_tiles.extend(current.path.iter().cloned());
                }
                Some(score) => {
                    if current.score > score {
                        break; // We've found all optimal paths
                    }
                    if current.score == score {
                        // Add all tiles from this equally optimal path
                        optimal_tiles.extend(current.path.iter().cloned());
                    }
                }
            }
            continue;
        }
        
        // Try all possible moves
        let next_states = vec![
            move_forward(&current, grid),
            Some(turn_clockwise(&current)),
            Some(turn_counterclockwise(&current))
        ];

        for next in next_states.into_iter().flatten() {
            let state_key = (next.row, next.col, next.direction);
            
            if !best_scores.contains_key(&state_key) || 
               next.score <= *best_scores.get(&state_key).unwrap() {
                best_scores.insert(state_key, next.score);
                heap.push(next);
            }
        }
    }
    
    (min_end_score, optimal_tiles)
}

pub fn run_a() -> std::io::Result<()> {
    let input = read_as_matrix("src/day_16/input.txt");
    
    let (min_score, _) = solve_maze_all_optimal_paths(&input);
    if let Some(result) = min_score {
        println!("Lowest possible score: {}", result);
    } else {
        // println!("No solution found!");
    }
    
    Ok(())
}

pub fn run_b() -> std::io::Result<()> {
    let input = read_as_matrix("src/day_16/input.txt");
    
    let (_, optimal_tiles) = solve_maze_all_optimal_paths(&input);
    println!("Number of tiles on optimal paths: {}", optimal_tiles.len());
    
    Ok(())
}