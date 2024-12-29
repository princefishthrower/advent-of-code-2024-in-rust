use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: i32,
    col: i32,
}

fn manhattan_distance(a: &Point, b: &Point) -> i32 {
    (a.row - b.row).abs() + (a.col - b.col).abs()
}

impl Point {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    fn adjacent(&self) -> Vec<Point> {
        vec![
            Point::new(self.row - 1, self.col), // up
            Point::new(self.row + 1, self.col), // down
            Point::new(self.row, self.col - 1), // left
            Point::new(self.row, self.col + 1), // right
        ]
    }
}

struct Maze {
    grid: Vec<Vec<char>>,
    start: Point,
    end: Point,
    height: i32,
    width: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SearchState {
    point: Point,
    wall_moves: i32,
    in_wall_sequence: bool,
}


impl Maze {
    fn from_input(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let height = grid.len() as i32;
        let width = grid[0].len() as i32;
        
        let mut start = Point::new(0, 0);
        let mut end = Point::new(0, 0);
        
        for (row, line) in grid.iter().enumerate() {
            for (col, &ch) in line.iter().enumerate() {
                match ch {
                    'S' => start = Point::new(row as i32, col as i32),
                    'E' => end = Point::new(row as i32, col as i32),
                    _ => {}
                }
            }
        }
        
        Self {
            grid,
            start,
            end,
            height,
            width,
        }
    }

    fn is_valid(&self, point: &Point) -> bool {
        point.row >= 0 
        && point.row < self.height 
        && point.col >= 0 
        && point.col < self.width
        && self.grid[point.row as usize][point.col as usize] != '#'
    }

    fn find_path(&self) -> Vec<Point> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut came_from = HashMap::new();
        
        queue.push_back(self.start);
        visited.insert(self.start);
        
        while let Some(current) = queue.pop_front() {
            if current == self.end {
                break;
            }
            
            for next in current.adjacent() {
                if self.is_valid(&next) && !visited.contains(&next) {
                    visited.insert(next);
                    came_from.insert(next, current);
                    queue.push_back(next);
                }
            }
        }
        
        // Reconstruct path
        let mut path = Vec::new();
        let mut current = self.end;
        while current != self.start {
            path.push(current);
            current = came_from[&current];
        }
        path.push(self.start);
        path.reverse();
        
        path
    }

    fn find_cheats_with_picosecond_savings(&self, min_savings: i32, max_duration: i32) -> Vec<(Point, Point, i32)> {
        let path = self.find_path();
        let mut cheats = HashMap::new(); // Changed from HashSet to HashMap to track best savings
        let total_points = path.len();
        
        let path_positions: HashMap<Point, usize> = path.iter()
            .enumerate()
            .map(|(i, &p)| (p, i))
            .collect();
        
        // For each point on the path
        for (start_idx, &start_point) in path.iter().enumerate() {
            let reachable_points = self.get_points_within_moves(&start_point, max_duration);
            
            for (end_point, total_moves) in reachable_points {
                if let Some(&end_idx) = path_positions.get(&end_point) {
                    if end_idx > start_idx {
                        let original_distance = (end_idx - start_idx) as i32;
                        let savings = original_distance - total_moves;
                        
                        if savings >= min_savings {
                            // Only keep the best savings for each start-end pair
                            let key = (start_point, end_point);
                            match cheats.get(&key) {
                                None => {
                                    cheats.insert(key, savings);
                                }
                                Some(&existing_savings) if savings > existing_savings => {
                                    cheats.insert(key, savings);
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
        
        println!("Finished processing all {} path positions", total_points);
        
        // Convert the HashMap into the required Vec format
        cheats.into_iter()
            .map(|((start, end), savings)| (start, end, savings))
            .collect()
    }

    fn get_points_within_moves(&self, start: &Point, max_moves: i32) -> HashSet<(Point, i32)> {
        let mut points = HashSet::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        
        // Start state: not in wall sequence, no wall moves
        let initial_state = SearchState {
            point: *start,
            wall_moves: 0,
            in_wall_sequence: false,
        };
        
        // (state, total_moves)
        queue.push_back((initial_state, 0));
        visited.insert(initial_state);
        
        while let Some((state, total_moves)) = queue.pop_front() {
            if total_moves <= max_moves {
                // Only add points that:
                // 1. Used at least one wall move
                // 2. Are currently on a valid track (not in wall)
                // 3. Have completed their wall sequence
                let current_is_wall = self.grid[state.point.row as usize][state.point.col as usize] == '#';
                if state.wall_moves > 0 && !current_is_wall && !state.in_wall_sequence {
                    points.insert((state.point, total_moves));
                }
                
                if total_moves < max_moves {
                    for next in state.point.adjacent() {
                        if next.row >= 0 && next.row < self.height && 
                           next.col >= 0 && next.col < self.width {
                            let next_is_wall = self.grid[next.row as usize][next.col as usize] == '#';
                            
                            // Calculate next state
                            let next_state = SearchState {
                                point: next,
                                wall_moves: state.wall_moves + if next_is_wall { 1 } else { 0 },
                                in_wall_sequence: if next_is_wall {
                                    // Start or continue wall sequence
                                    true
                                } else if state.in_wall_sequence {
                                    // End wall sequence if we're exiting a wall
                                    false
                                } else {
                                    // Keep current state if not in wall sequence
                                    state.in_wall_sequence
                                },
                            };
                            
                            // Validate the transition:
                            // 1. Can't exceed max wall moves
                            // 2. Can't start a new wall sequence if we already finished one
                            let valid_transition = 
                                next_state.wall_moves <= max_moves &&
                                !(next_is_wall && !state.in_wall_sequence && state.wall_moves > 0);
                            
                            if valid_transition && !visited.contains(&next_state) {
                                visited.insert(next_state);
                                queue.push_back((next_state, total_moves + 1));
                            }
                        }
                    }
                }
            }
        }
        
        points
    }
}


pub fn run_a() -> std::io::Result<()> {
    let input = std::fs::read_to_string("src/day_20/input.txt").expect("Failed to read input file");
    
    let maze = Maze::from_input(&input);
    
    // Find the base path first
    let path = maze.find_path();
    println!("Base path length: {} moves", path.len() - 1);
    
    // Find cheats that save at least 100 picoseconds
    let cheats = maze.find_cheats_with_picosecond_savings(100, 2);
    
    println!("\nNumber of cheats that save at least 100 picoseconds: {}", cheats.len());
    
    Ok(())
}

pub fn run_b() -> std::io::Result<()> {
    let default_filename: &'static str = "./src/day_20/input.txt";
    let args: Vec<String> = env::args().collect();
    let in_file: String = match args.len() {
        // No args provided save argv[0]
        1 => String::from(default_filename),
        _ => args[1].clone(),
    };

    let lines = fs::read_to_string(&in_file)
        .expect("Should have been able to read the file")
        .lines()
        .map(String::from)
        .collect::<Vec<_>>();

    let mut grid = build_grid(lines);

    let start = grid.find_one(b'S').unwrap();
    let end   = grid.find_one(b'E').unwrap();
    grid.set_ch(start.0, start.1, b'.');
    grid.set_ch(end.0,   end.1,   b'.');

    let big_wins = big_cheats_found(&grid, start, end);
    println!("Found {} big savings", big_wins);

    Ok(())

}

use std::env;
use std::fs;

// Advent of Code: 2024 day 20, part 2

#[derive(Clone)]
struct Grid {
    w: usize,
    h: usize,
    chars: Vec<u8>,
}

// i, j, cost
type Node = (usize, usize, i32);

#[allow(dead_code)]
impl Grid {
    fn ch(&self, i: usize, j: usize) -> u8 {
        return self.chars[self.id_from_pos(i, j)];
    }

    fn ch_or(&self, i: usize, j: usize, def: u8) -> u8 {
        if self.is_in_bounds(i, j) {
            let idx = self.id_from_pos(i, j);
            return self.chars[idx];
        } else {
            return def;
        }
    }

    fn set_ch(&mut self, i: usize, j: usize, ch: u8) {
        let idx = self.id_from_pos(i, j);
        self.chars[idx] = ch;
    }

    // Find x,y position of *first* cell filled with char.
    fn find_one(&self, ch: u8) -> Option<(usize, usize)> {
        return self.chars.iter()
            .position(|c| *c == ch)
            .map(|idx| self.pos_from_id(idx));
    }

    // Find x,y position of all cells filled with char.
    fn find_all(&self, ch: u8) -> Vec<(usize, usize)> {
        return self.chars.iter()
            .enumerate()
            .filter(|(_, c)| **c == ch)
            .map(|(i, _)| self.pos_from_id(i))
            .collect::<Vec<_>>();
    }

    fn id_from_pos(&self, i: usize, j: usize) -> usize {
        let l = self.w;
        return j * l + i;
    }

    fn pos_from_id(&self, idx: usize) -> (usize, usize) {
        let l = self.w;
        return (idx % l, idx / l);
    }

    fn is_in_bounds(&self, i: usize, j: usize) -> bool {
        return i < self.w && j < self.h;
    }

    fn id_from_node(&self, node: Node) -> usize {
        let (i, j, _) = node;

        return j * self.w + i;
    }

    // starting from (i,j), finds neighbors based on a provided closure, and
    // for each cell visited calls a second closure.
    fn visit_from<N, V>(&self, i: usize, j: usize, neighbors: N, mut visit: V)
        -> i32
        where
            N: Fn(&Grid, Node) -> Vec<Node>,
            V: FnMut(Node) -> bool
    {
        let mut queue: Vec<Node> = vec![];
        let mut visited = vec![false; self.w * self.h];

        // start node
        queue.push((i, j, 0));

        while let Some(node) = queue.pop() {
            let (x, y, cost) = node;
            let visit_idx = self.id_from_node(node);

            if !self.is_in_bounds(x, y) || visited[visit_idx] {
                continue;
            }

            visited[visit_idx] = true;
            if visit(node) {
                return cost;
            }

            let mut new_neighbors = neighbors(&self, node);
            new_neighbors.retain(|node| !visited[self.id_from_node(*node)]);

            queue.append(&mut new_neighbors);
            queue.sort_by_key(|(_, _, cost)| *cost);
            queue.reverse();
        }

        return -1;
    }
}

fn build_grid(lines: Vec<String>) -> Grid {
    let w = lines[0].len();
    let h = lines.len();
    let mut result: Vec<u8> = Vec::with_capacity(w * h);

    for line in lines {
        for ch in line.bytes() {
            result.push(ch)
        };
    };

    return Grid { w, h, chars: result };
}

fn big_cheats_found(g: &Grid, start: (usize, usize), end: (usize, usize)) -> i32 {
    let w = g.w;
    let mut costs: Vec<i32> = vec![(2 * w * w) as i32; w * w * 2];
    let mut ends_to_find = 2; // with cheat or without cheat

    let visit = |node| {
        let (i, j, cost) = node;
        let idx = g.id_from_node(node);

        if cost < costs[idx] {
            costs[idx] = cost;
        }

        if end == (i, j) {
            ends_to_find -= 1;
        }

        // return true if we've found both possible end nodes
        return ends_to_find == 0;
    };

    let d = |x: usize, dx: i32| {
        return (x as i32 + dx) as usize;
    };

    let neighbors = |g: &Grid, node| {
        let (i, j, cost) = node;
        let new_cost = cost + 1;

        // we can always travel four cardinal directions
        let mut ns: Vec<Node> = vec![
            (d(i, -1), j, new_cost),
            (d(i, 1), j, new_cost),
            (i, d(j, -1), new_cost),
            (i, d(j, 1), new_cost),
        ];

        ns.retain(|(i, j, _)| g.ch_or(*i, *j, b'#') == b'.');

        return ns;
    };

    // Fill in our cost table
    g.visit_from(start.0, start.1, neighbors, visit);

    let mut big_wins = 0; // Number of cheats that save 100ps or more

    // Every empty cell will be walked by problem definition. So just iterate
    // them all and itemize all the cheats.
    for n in g.find_all(b'.') {
        let (i, j) = n;
        let idx = g.id_from_pos(i, j);
        let start_cost = costs[idx];

        // Just go through all the possible cheats within 20 steps...

        let mut ns: Vec<Node> = vec![];
        for x in 0i32..=20 {
            for y in 0i32..=(20-x) {
                assert!(x+y <= 20);

                if x == 0 && y == 0 {
                    continue;
                }

                ns.push(( d(i,  x), d(j,  y), start_cost + x + y));
                ns.push(( d(i, -y), d(j,  x), start_cost + x + y));
                ns.push(( d(i, -x), d(j, -y), start_cost + x + y));
                ns.push(( d(i,  y), d(j, -x), start_cost + x + y));
            }
        }

        ns.retain(|(i, j, _)| g.ch_or(*i, *j, b'#') == b'.');
        ns.sort();
        ns.dedup();

        for n2 in ns.iter() {
            let orig_cost = costs[g.id_from_node(*n2)];
//          let (ii, jj, end_cost) = n2;
            let (_, _, end_cost) = n2;

            if *end_cost < orig_cost {
                let savings = orig_cost - end_cost;
                if savings >= 100 {
//                  println!("Going {},{} to {},{} saves {} ps!", i, j, ii, jj, orig_cost - end_cost);
                    big_wins += 1;
                }
            }
        }
    }

    return big_wins;
}
 