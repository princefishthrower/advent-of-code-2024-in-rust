use std::collections::{HashMap, HashSet};

use crate::utils::read_as_matrix::read_as_matrix;

struct Point {
    x: i32,
    y: i32,
}

pub fn run_a() -> std::io::Result<()> {
    let input = read_as_matrix("src/day_10/input.txt");

    // convert from Vec<Vec<char>> to Vec<Vec<i32>>
    let matrix = input.iter().map(|row| {
        row.iter().map(|c| {
            c.to_string().parse::<i32>().unwrap()
        }).collect::<Vec<i32>>()
    }).collect::<Vec<Vec<i32>>>();

    let trailheads = find_all_trailheads(&matrix);

    let mut total_score = 0;
    let mut memo = HashMap::new();

    for th in trailheads {
        total_score += trailhead_score(&matrix, th, &mut memo);
    }

    println!("Total score of all trailheads: {}", total_score);

    Ok(())
}

pub fn run_b() -> std::io::Result<()> {
    let input = read_as_matrix("src/day_10/input.txt");

    // convert from Vec<Vec<char>> to Vec<Vec<i32>>
    let matrix = input.iter().map(|row| {
        row.iter().map(|c| {
            c.to_string().parse::<i32>().unwrap()
        }).collect::<Vec<i32>>()
    }).collect::<Vec<Vec<i32>>>();

    let trailheads = find_all_trailheads(&matrix);

    let mut total_rating = 0;
    let mut memo = HashMap::new();

    for th in trailheads {
        total_rating += trailhead_rating(&matrix, th, &mut memo);
    }

    println!("Total rating of all trailheads: {}", total_rating);
    
    Ok(())
}

// a trailhead is anywhere on the matrix where the topo is 0
fn find_all_trailheads(matrix: &Vec<Vec<i32>>) -> Vec<Point> {
    let mut trailheads = Vec::<Point>::new();
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if matrix[y][x] == 0 {
                trailheads.push(Point { x: x as i32, y: y as i32 });
            }
        }
    }
    trailheads
}

fn neighbors(x: i32, y: i32, width: usize, height: usize) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    if x > 0 { result.push((x - 1, y)); }
    if x < (width - 1) as i32 { result.push((x + 1, y)); }
    if y > 0 { result.push((x, y - 1)); }
    if y < (height - 1) as i32 { result.push((x, y + 1)); }
    result
}

// from each trailhead (0), we try to go in all directions until we can't i.e. the next number from the current elevation is not +1 in elevation
// we keep doing this and each time we hit a 9 we add it to the count
// i.e. this is DFS with memoization
fn trailhead_elevation_search(matrix: &Vec<Vec<i32>>, x: i32, y: i32, memo: &mut HashMap<(i32,i32), HashSet<(i32,i32)>>) -> HashSet<(i32,i32)> {
    if let Some(cached) = memo.get(&(x,y)) {
        return cached.clone();
    }

    let current_height = matrix[y as usize][x as usize];
    if current_height == 9 {
        let mut set = HashSet::new();
        set.insert((x,y));
        memo.insert((x,y), set.clone());
        return set;
    }

    let mut reachable_nines = HashSet::new();
    for (nx, ny) in neighbors(x, y, matrix[0].len(), matrix.len()) {
        let next_height = matrix[ny as usize][nx as usize];
        if next_height == current_height + 1 {
            // Explore further
            let result = trailhead_elevation_search(matrix, nx, ny, memo);
            for r in result {
                reachable_nines.insert(r);
            }
        }
    }

    memo.insert((x,y), reachable_nines.clone());
    reachable_nines
}

// the score is based on how many 9s we can reach from a trailhead
fn trailhead_score(matrix: &Vec<Vec<i32>>, trailhead: Point, memo: &mut HashMap<(i32,i32), HashSet<(i32,i32)>>) -> i32 {
    let result = trailhead_elevation_search(matrix, trailhead.x, trailhead.y, memo);
    result.len() as i32
}

// Counts the number of distinct trails from (x,y) to any cell of height 9
fn count_paths(matrix: &Vec<Vec<i32>>, x: i32, y: i32, memo: &mut HashMap<(i32,i32), u64>) -> u64 {
    if let Some(&cached) = memo.get(&(x,y)) {
        return cached;
    }

    let current_height = matrix[y as usize][x as usize];
    if current_height == 9 {
        // This cell itself is a height 9 endpoint
        memo.insert((x,y), 1);
        return 1;
    }

    let mut total_paths: u64 = 0;
    for (nx, ny) in neighbors(x, y, matrix[0].len(), matrix.len()) {
        let next_height = matrix[ny as usize][nx as usize];
        if next_height == current_height + 1 {
            total_paths += count_paths(matrix, nx, ny, memo);
        }
    }

    memo.insert((x,y), total_paths);
    total_paths
}

// a rating is based on the number of distinct trails that begin at that trailhead
fn trailhead_rating(matrix: &Vec<Vec<i32>>, trailhead: Point, memo: &mut HashMap<(i32,i32), u64>) -> u64 {
    count_paths(matrix, trailhead.x, trailhead.y, memo)
}