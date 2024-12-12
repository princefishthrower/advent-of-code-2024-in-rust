use std::collections::{HashMap, HashSet};
use crate::utils::read_as_matrix::read_as_matrix;
use std::cmp::Ordering;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct Region {
    letter: char,
    garden_plots: Vec<Point>,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.y.cmp(&other.y) {
            Ordering::Equal => self.x.cmp(&other.x),
            other_order => other_order,
        }
    }
}

impl Region {
    fn area(&self) -> usize {
        self.garden_plots.len()
    }
    
    fn perimeter(&self) -> i32 {
        // We'll compute the perimeter by counting adjacency.
        // If we know:
        //    perimeter = 4*A - 2*E
        // where E is the number of internal adjacencies between cells in the region.
        let area = self.area() as i32;
        
        // Put all points in a set for O(1) adjacency lookup
        let plot_set: HashSet<Point> = self.garden_plots.iter().copied().collect();
        
        let mut adjacency_count = 0;
        
        // Directions: up, down, left, right
        let directions = [(0,1), (0,-1), (1,0), (-1,0)];
        
        for &point in &self.garden_plots {
            for (dx, dy) in directions.iter() {
                let neighbor = Point { x: point.x + dx, y: point.y + dy };
                if plot_set.contains(&neighbor) {
                    adjacency_count += 1;
                }
            }
        }
        
        // Each adjacency is counted twice (once for each cell), so E = adjacency_count/2
        let e = adjacency_count as i32 / 2;
        
        calculate_perimeter_from_area(area, e)
    }
    
    fn price(&self) -> i32 {
        let a = self.area() as i32;
        let p = self.perimeter();
        a * p
    }

    fn sides(&self) -> i32 {
        let plot_set: HashSet<Point> = self.garden_plots.iter().copied().collect();
        
        // Define the corner patterns we're looking for
        let corners = [
            // (first dir, second dir, diagonal) for each corner pattern
            ((0,1), (1,0), (1,1)),    // top-right corner
            ((1,0), (0,-1), (1,-1)),  // bottom-right corner
            ((0,-1), (-1,0), (-1,-1)), // bottom-left corner
            ((-1,0), (0,1), (-1,1))   // top-left corner
        ];
        
        let mut total_corners = 0;
        
        for p in &self.garden_plots {
            for &((dx0, dy0), (dx1, dy1), (dx2, dy2)) in &corners {
                // Check for outside corner (two missing neighbors)
                let has_first = plot_set.contains(&Point { x: p.x + dx0, y: p.y + dy0 });
                let has_second = plot_set.contains(&Point { x: p.x + dx1, y: p.y + dy1 });
                if !has_first && !has_second {
                    total_corners += 1;
                }
                
                // Check for inside corner (two present neighbors but missing diagonal)
                let has_diagonal = plot_set.contains(&Point { x: p.x + dx2, y: p.y + dy2 });
                if has_first && has_second && !has_diagonal {
                    total_corners += 1;
                }
            }
        }
        
        total_corners
    }
    
    /// Compute price under the new bulk discount rule:
    /// price = area * sides
    fn new_price(&self) -> i32 {
        let a = self.area() as i32;
        let s = self.sides();
        a * s
    }
}

pub fn run_a() -> std::io::Result<()> {
    
    let input = read_as_matrix("src/day_12/input.txt");

    let regions = find_regions(&input);

    let total_price: i32 = regions.iter().map(|r| r.price()).sum();

    println!("Total price: {}", total_price);

    Ok(())
}

pub fn run_b() -> std::io::Result<()> {

    let input = read_as_matrix("src/day_12/input.txt");

    let regions = find_regions(&input);

    let new_price: i32 = regions.iter().map(|r| r.new_price()).sum();

    println!("Total price with new bulk discount: {}", new_price);
    
    Ok(())
}

// Given an area's cell count (A) and the number of internal adjacencies (E), compute perimeter.
// Perimeter formula: P = 4*A - 2*E
pub fn calculate_perimeter_from_area(area: i32, adjacencies: i32) -> i32 {
    4 * area - 2 * adjacencies
}

// Find all regions of contiguous plots with the same letter.
fn find_regions(matrix: &[Vec<char>]) -> Vec<Region> {
    let rows = matrix.len() as i32;
    let cols = matrix[0].len() as i32;
    
    let mut visited = vec![vec![false; cols as usize]; rows as usize];
    let mut regions = Vec::new();
    
    for r in 0..rows {
        for c in 0..cols {
            if visited[r as usize][c as usize] {
                continue;
            }
            
            let letter = matrix[r as usize][c as usize];
            let start = Point { x: c, y: r };
            
            // Use BFS to find all connected plots of the same letter
            let mut plots = Vec::new();
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(start);
            visited[r as usize][c as usize] = true;
            
            while let Some(p) = queue.pop_front() {
                plots.push(p);
                
                // ONLY check orthogonally adjacent cells - NEVER diagonal
                let directions = [(0,1), (0,-1), (1,0), (-1,0)];
                for (dx, dy) in directions {
                    let new_x = p.x + dx;
                    let new_y = p.y + dy;
                    
                    // Bounds check
                    if new_x >= 0 && new_x < cols && new_y >= 0 && new_y < rows {
                        let ny = new_y as usize;
                        let nx = new_x as usize;
                        
                        // Only connect if unvisited AND same letter AND orthogonally adjacent
                        if !visited[ny][nx] && matrix[ny][nx] == letter {
                            visited[ny][nx] = true;
                            queue.push_back(Point { x: new_x, y: new_y });
                        }
                    }
                }
            }
            
            regions.push(Region {
                letter,
                garden_plots: plots,
            });
        }
    }
    
    regions
}