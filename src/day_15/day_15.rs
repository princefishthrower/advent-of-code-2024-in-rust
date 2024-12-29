use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;

use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn origin() -> Self {
        Self::new(0, 0)
    }

    pub fn up() -> Self {
        Self::new(0, -1)
    }

    pub fn down() -> Self {
        Self::new(0, 1)
    }

    pub fn left() -> Self {
        Self::new(-1, 0)
    }

    pub fn right() -> Self {
        Self::new(1, 0)
    }

    pub fn diagonal() -> impl Iterator<Item = Self> {
        (-1..=1).flat_map(|x| {
            (-1..=1)
                .filter(move |&y| x != 0 && y != 0)
                .map(move |y| Self::new(x, y))
        })
    }

    pub fn moore() -> impl Iterator<Item = Self> {
        (-1..=1).flat_map(|x| {
            (-1..=1)
                .filter(move |&y| x != 0 || y != 0)
                .map(move |y| Self::new(x, y))
        })
    }

    pub fn von_neumann() -> impl Iterator<Item = Self> {
        (-1..=1).flat_map(|x| {
            (-1..=1)
                .filter(move |&y| (x == 0) ^ (y == 0))
                .map(move |y| Self::new(x, y))
        })
    }

    pub fn manhattan_distance(&self, other: &Point) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub fn opposite(&self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl From<u8> for Point {
    fn from(value: u8) -> Self {
        match value {
            b'^' => Self::up(),
            b'v' => Self::down(),
            b'<' => Self::left(),
            b'>' => Self::right(),
            _ => unreachable!(),
        }
    }
}

impl From<Point> for u8 {
    fn from(value: Point) -> Self {
        match value {
            Point { x: 0, y: -1 } => b'^',
            Point { x: 0, y: 1 } => b'v',
            Point { x: -1, y: 0 } => b'<',
            Point { x: 1, y: 0 } => b'>',
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Warehouse {
    walls: HashSet<Position>,
    boxes: HashSet<Position>,
    robot: Position,
    width: usize,
    height: usize,
}

#[derive(Debug)]
struct BigWarehouse {
    walls: HashSet<Position>,
    boxes: HashSet<Position>, // Stores the left position of each wide box
    robot: Position,
    width: usize,
    height: usize,
}

impl Move {
    fn from_char(c: char) -> Option<Move> {
        match c {
            '<' => Some(Move::Left),
            '>' => Some(Move::Right),
            '^' => Some(Move::Up),
            'v' => Some(Move::Down),
            _ => None,
        }
    }

    fn get_direction(&self) -> (isize, isize) {
        match self {
            Move::Left => (0, -1),
            Move::Right => (0, 1),
            Move::Up => (-1, 0),
            Move::Down => (1, 0),
        }
    }
}

impl Position {
    fn apply_direction(&self, (dx, dy): (isize, isize)) -> Option<Position> {
        let new_x = self.x.checked_add_signed(dx);
        let new_y = self.y.checked_add_signed(dy);

        match (new_x, new_y) {
            (Some(x), Some(y)) => Some(Position { x, y }),
            _ => None,
        }
    }
}

impl Warehouse {
    // Helper function to check if a position is empty (no wall or box)
    fn is_empty(&self, pos: &Position) -> bool {
        !self.walls.contains(pos) && !self.boxes.contains(pos)
    }

    // Helper function to find the length of a box chain starting at a position
    fn get_box_chain_length(&self, start: Position, direction: (isize, isize)) -> usize {
        let mut length = 0;
        let mut current = start;

        while self.boxes.contains(&current) {
            length += 1;
            if let Some(next) = current.apply_direction(direction) {
                current = next;
            } else {
                break;
            }
        }

        length
    }

    // Helper function to check if a box chain can be moved
    fn can_move_box_chain(&self, start: Position, direction: (isize, isize)) -> bool {
        let chain_length = self.get_box_chain_length(start, direction);
        let mut current = start;

        // Find the last box in the chain
        for _ in 0..chain_length - 1 {
            if let Some(next) = current.apply_direction(direction) {
                current = next;
            } else {
                return false;
            }
        }

        // Check if there's space after the last box
        if let Some(final_pos) = current.apply_direction(direction) {
            self.is_empty(&final_pos)
        } else {
            false
        }
    }

    // Main movement execution function
    fn execute_move(&mut self, movement: &Move) {
        let direction = movement.get_direction();

        // Get the position the robot wants to move to
        let next_robot_pos = match self.robot.apply_direction(direction) {
            Some(pos) => pos,
            None => return,
        };

        // Check if the next position is a wall
        if self.walls.contains(&next_robot_pos) {
            return;
        }

        // If there's no box, just move the robot
        if !self.boxes.contains(&next_robot_pos) {
            self.robot = next_robot_pos;
            return;
        }

        // If there's a box, check if the box chain can be moved
        if !self.can_move_box_chain(next_robot_pos, direction) {
            return;
        }

        // Find all boxes in the chain
        let mut box_positions = Vec::new();
        let mut current = next_robot_pos;
        while self.boxes.contains(&current) {
            box_positions.push(current);
            if let Some(next) = current.apply_direction(direction) {
                current = next;
            } else {
                break;
            }
        }

        // Move boxes from back to front
        for &box_pos in box_positions.iter().rev() {
            if let Some(new_pos) = box_pos.apply_direction(direction) {
                self.boxes.remove(&box_pos);
                self.boxes.insert(new_pos);
            }
        }

        // Move robot
        self.robot = next_robot_pos;
    }

    // Parse function remains the same
    fn parse<P: AsRef<Path>>(file_path: P) -> io::Result<(Self, Vec<Move>)> {
        let content = fs::read_to_string(file_path)?;

        let mut walls = HashSet::new();
        let mut boxes = HashSet::new();
        let mut robot = Position { x: 0, y: 0 };
        let mut height = 0;
        let mut width = 0;

        // Parse warehouse layout
        for (x, line) in content.lines().enumerate() {
            if line.trim().is_empty() || !line.contains('#') {
                break;
            }
            for (y, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        walls.insert(Position { x, y });
                    }
                    'O' => {
                        boxes.insert(Position { x, y });
                    }
                    '@' => {
                        robot = Position { x, y };
                    }
                    '.' => {}
                    _ => {}
                }
                width = width.max(y + 1);
            }
            height = x + 1;
        }

        // Parse moves
        let moves = content
            .lines()
            .skip_while(|line| line.contains('#') || line.trim().is_empty())
            .flat_map(|line| line.chars())
            .filter_map(Move::from_char)
            .collect();

        Ok((
            Warehouse {
                walls,
                boxes,
                robot,
                width,
                height,
            },
            moves,
        ))
    }

    fn calculate_gps_sum(&self) -> usize {
        self.boxes.iter().map(|pos| 100 * pos.x + pos.y).sum()
    }

    fn generate_snapshot(&self) -> String {
        let mut output = String::new();

        for x in 0..self.height {
            for y in 0..self.width {
                let pos = Position { x, y };
                let char = if self.walls.contains(&pos) {
                    '#'
                } else if self.boxes.contains(&pos) {
                    'O'
                } else if self.robot == pos {
                    '@'
                } else {
                    '.'
                };
                output.push(char);
            }
            output.push('\n');
        }

        output
    }
}

impl BigWarehouse {
    // Helper to get the full box (both positions) given the left position
    fn get_full_box(&self, left_pos: Position) -> (Position, Position) {
        (left_pos, Position { x: left_pos.x, y: left_pos.y + 1 })
    }

    // Check if a position is part of any box
    fn has_box_at(&self, pos: &Position) -> bool {
        if self.boxes.contains(pos) {
            return true;
        }
        // Check if this is the right half of a box
        if pos.y > 0 && self.boxes.contains(&Position { x: pos.x, y: pos.y - 1 }) {
            return true;
        }
        false
    }

    // Get the left position of a box if pos is part of a box
    fn get_box_left_pos(&self, pos: &Position) -> Option<Position> {
        if self.boxes.contains(pos) {
            Some(*pos)
        } else if pos.y > 0 && self.boxes.contains(&Position { x: pos.x, y: pos.y - 1 }) {
            Some(Position { x: pos.x, y: pos.y - 1 })
        } else {
            None
        }
    }

    // Check if both positions for a wide box would be valid after movement
    fn can_move_wide_box_to(&self, left_pos: Position) -> bool {
        !self.walls.contains(&left_pos) && 
        !self.walls.contains(&Position { x: left_pos.x, y: left_pos.y + 1 })
    }

    // Get all boxes that would be pushed in this movement
    fn get_movable_boxes(&self, start_pos: &Position, direction: (isize, isize)) -> Vec<Position> {
        let mut boxes = Vec::new();
        let mut current = *start_pos;

        // If we're pushing the right half of a box, adjust to the left position
        if let Some(left_pos) = self.get_box_left_pos(&current) {
            current = left_pos;
        }

        match direction {
            (0, 1) => { // Moving right
                // Include the current box and any boxes immediately to its right
                while let Some(left_pos) = self.get_box_left_pos(&current) {
                    boxes.push(left_pos);
                    if let Some(next) = (Position { x: current.x, y: current.y + 2 }).apply_direction((0, 0)) {
                        if !self.has_box_at(&next) {
                            break;
                        }
                        current = next;
                    } else {
                        break;
                    }
                }
            },
            (0, -1) => { // Moving left
                // Include all boxes to the left
                while let Some(left_pos) = self.get_box_left_pos(&current) {
                    boxes.push(left_pos);
                    if let Some(next) = current.apply_direction((0, -2)) {
                        if !self.has_box_at(&next) {
                            break;
                        }
                        current = next;
                    } else {
                        break;
                    }
                }
            },
            (dx, 0) => { // Vertical movement
                // For vertical movement, we need to handle each column separately
                // First, add any box at the current position
                if let Some(left_pos) = self.get_box_left_pos(&current) {
                    boxes.push(left_pos);
                }
                
                // If this is the right half of a wide box and we're pushing down,
                // we also need to check the left half
                if self.has_box_at(&Position { x: current.x, y: current.y - 1 }) {
                    if let Some(left_pos) = self.get_box_left_pos(&Position { x: current.x, y: current.y - 1 }) {
                        if !boxes.contains(&left_pos) {
                            boxes.push(left_pos);
                        }
                    }
                }
                
                // If this is the left half of a wide box and we're pushing down,
                // we also need to check the right half
                if self.has_box_at(&Position { x: current.x, y: current.y + 1 }) {
                    if let Some(left_pos) = self.get_box_left_pos(&Position { x: current.x, y: current.y + 1 }) {
                        if !boxes.contains(&left_pos) {
                            boxes.push(left_pos);
                        }
                    }
                }
            },
            _ => {}
        }

        boxes
    }

    fn get_position_ahead(&self, box_pos: Position, direction: (isize, isize)) -> Option<Position> {
        match direction {
            (0, 1) => Some(Position { x: box_pos.x, y: box_pos.y + 2 }), // Right: jump 2 spaces
            (0, -1) => Some(Position { x: box_pos.x, y: box_pos.y - 1 }), // Left: check one space
            (1, 0) | (-1, 0) => box_pos.apply_direction(direction), // Up/Down: normal movement
            _ => None,
        }
    }

    fn execute_move(&mut self, movement: &Move) {
        let direction = movement.get_direction();
        
        // Get next robot position
        let next_robot_pos = match self.robot.apply_direction(direction) {
            Some(pos) => pos,
            None => return,
        };

        // Check for wall collision
        if self.walls.contains(&next_robot_pos) {
            return;
        }

        // If there's no box, just move the robot
        if !self.has_box_at(&next_robot_pos) {
            self.robot = next_robot_pos;
            return;
        }

        // Get boxes that need to be moved
        let boxes_to_move = self.get_movable_boxes(&next_robot_pos, direction);
        
        // Check if we can move all boxes
        let mut can_move = true;
        for &box_pos in &boxes_to_move {
            if let Some(new_pos) = box_pos.apply_direction(direction) {
                // Check both positions for the wide box
                if !self.can_move_wide_box_to(new_pos) {
                    can_move = false;
                    break;
                }
                // For horizontal movement, need to check if we're pushing into another box
                match direction {
                    (0, 1) => { // Moving right
                        if let Some(next_box) = (Position { x: new_pos.x, y: new_pos.y + 2 }).apply_direction((0, 0)) {
                            if self.has_box_at(&next_box) && !boxes_to_move.contains(&Position { x: next_box.x, y: next_box.y - 1 }) {
                                can_move = false;
                                break;
                            }
                        }
                    },
                    (0, -1) => { // Moving left
                        if let Some(prev_box) = new_pos.apply_direction((0, -1)) {
                            if self.has_box_at(&prev_box) && !boxes_to_move.contains(&self.get_box_left_pos(&prev_box).unwrap_or(prev_box)) {
                                can_move = false;
                                break;
                            }
                        }
                    },
                    _ => {}
                }
            } else {
                can_move = false;
                break;
            }
        }

        if !can_move {
            return;
        }

        // Find the final position for the robot
        let mut final_robot_pos = next_robot_pos;
        
        // For vertical movements, we need to find where the robot will end up
        match direction {
            (1, 0) | (-1, 0) => { // Vertical movement
                if !boxes_to_move.is_empty() {
                    // Robot should end up next to the last moved box
                    let last_box = boxes_to_move.last().unwrap();
                    if let Some(new_box_pos) = last_box.apply_direction(direction) {
                        final_robot_pos = new_box_pos;
                    }
                }
            },
            _ => {}
        }

        // Move boxes from back to front based on movement direction
        let sorted_boxes = match direction {
            (0, 1) => { // Moving right - move rightmost boxes first
                let mut boxes = boxes_to_move;
                boxes.sort_by_key(|pos| std::cmp::Reverse(pos.y));
                boxes
            },
            (0, -1) => { // Moving left - move leftmost boxes first
                let mut boxes = boxes_to_move;
                boxes.sort_by_key(|pos| pos.y);
                boxes
            },
            (1, 0) => { // Moving down - move bottom boxes first
                let mut boxes = boxes_to_move;
                boxes.sort_by_key(|pos| std::cmp::Reverse(pos.x));
                boxes
            },
            (-1, 0) => { // Moving up - move top boxes first
                let mut boxes = boxes_to_move;
                boxes.sort_by_key(|pos| pos.x);
                boxes
            },
            _ => boxes_to_move,
        };

        // Move boxes
        for &box_pos in &sorted_boxes {
            if let Some(new_pos) = box_pos.apply_direction(direction) {
                // For vertical movement, we need to ensure we're not creating overlapping boxes
                match direction {
                    (1, 0) | (-1, 0) => {
                        // Check if there's already a box at the target position
                        if !sorted_boxes.iter().any(|&pos| 
                            pos.apply_direction(direction).map_or(false, |p| p == new_pos)
                        ) {
                            self.boxes.remove(&box_pos);
                            self.boxes.insert(new_pos);
                        }
                    },
                    _ => {
                        self.boxes.remove(&box_pos);
                        self.boxes.insert(new_pos);
                    }
                }
            }
        }

        // Move robot to its final position
        self.robot = final_robot_pos;
    }

    // Parse function that doubles the width
    fn parse<P: AsRef<Path>>(file_path: P) -> io::Result<(Self, Vec<Move>)> {
        let content = fs::read_to_string(file_path)?;

        let mut walls = HashSet::new();
        let mut boxes = HashSet::new();
        let mut robot = Position { x: 0, y: 0 };
        let mut height = 0;
        let mut width = 0;

        // Parse warehouse layout
        for (x, line) in content.lines().enumerate() {
            if line.trim().is_empty() || !line.contains('#') {
                break;
            }
            let mut double_y = 0;
            for (y, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        walls.insert(Position { x, y: double_y });
                        walls.insert(Position { x, y: double_y + 1 });
                        double_y += 2;
                    }
                    'O' => {
                        boxes.insert(Position { x, y: double_y });
                        double_y += 2;
                    }
                    '@' => {
                        robot = Position { x, y: double_y };
                        double_y += 2;
                    }
                    '.' => {
                        double_y += 2;
                    }
                    _ => {}
                }
                width = width.max(double_y);
            }
            height = x + 1;
        }

        // Parse moves
        let moves = content
            .lines()
            .skip_while(|line| line.contains('#') || line.trim().is_empty())
            .flat_map(|line| line.chars())
            .filter_map(Move::from_char)
            .collect();

        Ok((
            BigWarehouse {
                walls,
                boxes,
                robot,
                width,
                height,
            },
            moves,
        ))
    }

    fn calculate_gps_sum(&self) -> usize {
        self.boxes.iter().map(|pos| 100 * pos.x + pos.y).sum()
    }

    fn generate_snapshot(&self) -> String {
        let mut output = String::new();

        for x in 0..self.height {
            for y in 0..self.width {
                let pos = Position { x, y };
                let char = if self.walls.contains(&pos) {
                    '#'
                } else if self.boxes.contains(&pos) {
                    '['
                } else if y > 0 && self.boxes.contains(&Position { x, y: y - 1 }) {
                    ']'
                } else if self.robot == pos {
                    '@'
                } else {
                    '.'
                };
                output.push(char);
            }
            output.push('\n');
        }

        output
    }
}

pub fn run_a() -> std::io::Result<()> {
    let (mut warehouse, moves) = Warehouse::parse("src/day_15/input.txt")?;

    // Execute all moves
    for movement in moves {
        warehouse.execute_move(&movement);
        // println!("After applying {}", &movement);
        // println!("{}", warehouse.generate_snapshot());
    }

    // print GPS score
    println!("Final GPS sum: {}", warehouse.calculate_gps_sum());

    Ok(())
}

pub fn run_b() -> std::io::Result<()> {
    let input = include_str!("../../src/day_15/input.txt");
    let (mut grid, moves, mut robot) = parse(input, 2);

    for &m in moves {
        let direction = Point::from(m);
        let next = robot + direction;

        match grid[next.y as usize][next.x as usize] {
            b'.' => {
                // Trivially move the robot to the next spot if it's empty.
                robot = next;
            }
            side @ b'[' | side @ b']' => {
                // If a box is in the next spot, we need to then find all boxes and
                // determine if it's possible to move them all.
                let mut boxes = vec![next];

                // Add the other side of the box to our list of boxes.
                if side == b'[' {
                    boxes.push(next + Point::right());
                } else {
                    boxes.push(next + Point::left());
                }

                // Search through boxes until we find them all.
                let mut blocked = false;

                match m {
                    b'^' | b'v' => {
                        // If we're moving up or down, finding the boxes is a little more complex
                        // than left and right as we could have two boxes behind a single box.
                        //
                        // For example, if the robot is moving up in the scenario below, we need
                        // to move both boxes behind that first box in front of the robot.
                        //
                        //     [][]
                        //      []
                        //       @
                        //

                        // Start the search with the first box.
                        let mut current = boxes.clone();

                        while current.len() > 1 {
                            let mut next = Vec::new();

                            for b in current {
                                let path = b + direction;

                                match grid[path.y as usize][path.x as usize] {
                                    b'#' => {
                                        // If we found a wall above/below the box we're searching, that
                                        // means we're blocked, and we can stop the search here.
                                        blocked = true;
                                        next.clear();
                                        break;
                                    }
                                    side @ b'[' | side @ b']' => {
                                        // If we found another box, add it to our total list of boxes and
                                        // the list of next boxes to search if we don't already have it.
                                        if !next.contains(&path) {
                                            boxes.push(path);
                                            next.push(path);

                                            if side == b'[' {
                                                boxes.push(path + Point::right());
                                                next.push(path + Point::right());
                                            } else {
                                                boxes.push(path + Point::left());
                                                next.push(path + Point::left());
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }

                            current = next;
                        }
                    }
                    b'<' | b'>' => {
                        // If we're moving left or right, finding all the boxes is simple as
                        // we just need to search in a straight line, similar to part 1.
                        //
                        //     @[][]
                        //

                        // Start the search after the first box.
                        let mut path = next + direction + direction;

                        // Continue searching while we're finding boxes.
                        while [b'[', b']'].contains(&grid[path.y as usize][path.x as usize]) {
                            boxes.push(path);
                            path += direction;
                        }

                        // If the first non-box spot is not empty, we are blocked from moving.
                        if grid[path.y as usize][path.x as usize] != b'.' {
                            blocked = true;
                        }
                    }
                    _ => {}
                }

                // Move all the boxes and the robot only if we're not blocked.
                if !blocked {
                    for &b in boxes.iter().rev() {
                        let mov = b + direction;
                        grid[mov.y as usize][mov.x as usize] = grid[b.y as usize][b.x as usize];
                        grid[b.y as usize][b.x as usize] = b'.';
                    }

                    robot = next;
                }
            }
            _ => {}
        }
    }

    let result = coordinates(&grid);

    println!("The GPS score is: {}", result);

    Ok(())

}

fn parse(input: &str, part: u8) -> (Vec<Vec<u8>>, Vec<&u8>, Point) {
    let (grid, moves) = input.split_once("\n\n").unwrap();
    let mut robot = None;

    (
        grid.lines()
            .enumerate()
            .map(|(y, line)| {
                if part == 1 {
                    line.bytes()
                        .enumerate()
                        .map(|(x, b)| {
                            if b == b'@' {
                                robot = Some(Point::new(x as i32, y as i32));
                                b'.'
                            } else {
                                b
                            }
                        })
                        .collect()
                } else {
                    line.bytes()
                        .enumerate()
                        .flat_map(|(x, b)| match b {
                            b'#' => [b'#', b'#'],
                            b'O' => [b'[', b']'],
                            b'.' => [b'.', b'.'],
                            b'@' => {
                                robot = Some(Point::new(x as i32 * 2, y as i32));
                                [b'.', b'.']
                            }
                            _ => unreachable!(),
                        })
                        .collect()
                }
            })
            .collect(),
        moves.lines().flat_map(|line| line.as_bytes()).collect(),
        robot.unwrap(),
    )
}

fn coordinates(grid: &[Vec<u8>]) -> usize {
    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, b)| {
                    if *b == b'O' || *b == b'[' {
                        y * 100 + x
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}
