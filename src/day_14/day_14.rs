use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::path::PathBuf;
use image::{RgbImage, Rgb};
use rayon::prelude::*;
use rusttype::{Font, Scale};
use image::{ImageBuffer};

// define const for tile width and height
const TILES_WIDTH: i64 = 101;
const TILES_HEIGHT: i64 = 103;

// for example input (smaller examples :D)
// const TILES_WIDTH: i64 = 11;
// const TILES_HEIGHT: i64 = 7;

#[derive(Debug, Clone)]
pub struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone)]
pub struct Velocity {
    dx: i64,
    dy: i64,
}

#[derive(Debug, Clone)]
pub struct Drone {
    position: Position,
    velocity: Velocity,
}

impl Drone {
    fn new(px: i64, py: i64, vx: i64, vy: i64) -> Self {
        Drone {
            position: Position { x: px, y: py },
            velocity: Velocity { dx: vx, dy: vy },
        }
    }
}

// Simple 5x7 bitmap font for digits and some characters
const DIGITS: [&[u8]; 10] = [
    // 0
    &[
        0b01110,
        0b10001,
        0b10001,
        0b10001,
        0b10001,
        0b10001,
        0b01110,
    ],
    // 1
    &[
        0b00100,
        0b01100,
        0b00100,
        0b00100,
        0b00100,
        0b00100,
        0b01110,
    ],
    // 2
    &[
        0b01110,
        0b10001,
        0b00001,
        0b00010,
        0b00100,
        0b01000,
        0b11111,
    ],
    // 3
    &[
        0b01110,
        0b10001,
        0b00001,
        0b00110,
        0b00001,
        0b10001,
        0b01110,
    ],
    // 4
    &[
        0b00010,
        0b00110,
        0b01010,
        0b10010,
        0b11111,
        0b00010,
        0b00010,
    ],
    // 5
    &[
        0b11111,
        0b10000,
        0b11110,
        0b00001,
        0b00001,
        0b10001,
        0b01110,
    ],
    // 6
    &[
        0b01110,
        0b10000,
        0b10000,
        0b11110,
        0b10001,
        0b10001,
        0b01110,
    ],
    // 7
    &[
        0b11111,
        0b00001,
        0b00010,
        0b00100,
        0b01000,
        0b01000,
        0b01000,
    ],
    // 8
    &[
        0b01110,
        0b10001,
        0b10001,
        0b01110,
        0b10001,
        0b10001,
        0b01110,
    ],
    // 9
    &[
        0b01110,
        0b10001,
        0b10001,
        0b01111,
        0b00001,
        0b10001,
        0b01110,
    ],
];

fn draw_number(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, num: usize, x: u32, y: u32) {
    let digits = num.to_string();
    let mut current_x = x;
    
    for digit in digits.chars() {
        let digit_idx = (digit as u8 - b'0') as usize;
        if digit_idx < DIGITS.len() {
            let bitmap = DIGITS[digit_idx];
            for (row, &bits) in bitmap.iter().enumerate() {
                for col in 0..5 {
                    if (bits & (1 << (4 - col))) != 0 {
                        // Make the number 2x2 pixels for better visibility
                        for dy in 0..2 {
                            for dx in 0..2 {
                                img.put_pixel(
                                    current_x + (col as u32 * 2) + dx,
                                    y + (row as u32 * 2) + dy,
                                    Rgb([255, 255, 255])
                                );
                            }
                        }
                    }
                }
            }
        }
        current_x += 12; // Space between digits
    }
}

pub fn run_a() -> std::io::Result<()> {
    let mut drones = read_drones("src/day_14/input.txt")?;

    // simulate drone movement and log grid snapshot
    for _ in 0..100 {
        simulate_drone_movement(&mut drones);
    }

    log_grid_snapshot(&drones);
    log_quadrant_snapshot(&drones);

    // calculate safety factor
    let safety_factor = calculate_safety_factor(&drones);

    println!("Safety factor: {}", safety_factor);
    
    Ok(())
}

pub fn run_b() -> std::io::Result<()> {

    let mut drones = read_drones("src/day_14/input.txt")?;

    let current_dir = env::current_dir()?;
    let frames_dir = current_dir.join("drone_frames");
    let output_video = current_dir.join("drone_simulation.mp4");
    
    println!("Generating frames...");
    generate_frame_images(&mut drones, &frames_dir, 10_000)?;
    
    println!("Converting to video...");
    convert_frames_to_video(&frames_dir, &output_video)?;
    
    Ok(())
}

pub fn read_drones<P: AsRef<Path>>(path: P) -> io::Result<Vec<Drone>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut drones = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        // Split the line into position and velocity parts
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            continue;
        }

        // Parse position
        let pos_str = parts[0].trim_start_matches("p=");
        let pos_parts: Vec<&str> = pos_str.split(',').collect();
        if pos_parts.len() != 2 {
            continue;
        }

        // Parse velocity
        let vel_str = parts[1].trim_start_matches("v=");
        let vel_parts: Vec<&str> = vel_str.split(',').collect();
        if vel_parts.len() != 2 {
            continue;
        }

        // Parse the numbers
        if let (Ok(px), Ok(py), Ok(vx), Ok(vy)) = (
            pos_parts[0].parse::<i64>(),
            pos_parts[1].parse::<i64>(),
            vel_parts[0].parse::<i64>(),
            vel_parts[1].parse::<i64>(),
        ) {
            drones.push(Drone::new(px, py, vx, vy));
        }
    }

    Ok(drones)
}

fn simulate_drone_movement(drones: &mut [Drone]) {
    // until a drone is at the edge of the grid, we keep moving the drones in their respective directions
    // however, at an edge, they will "teleport" i.e. to wrap around to the other side of the grid
    for drone in drones {
        // Update position with velocity
        drone.position.x += drone.velocity.dx;
        drone.position.y += drone.velocity.dy;

        // Handle wrapping with modulo arithmetic to maintain continuous motion
        // We add TILES_WIDTH/HEIGHT before taking modulo to handle negative numbers correctly
        drone.position.x = ((drone.position.x + TILES_WIDTH) % TILES_WIDTH);
        drone.position.y = ((drone.position.y + TILES_HEIGHT) % TILES_HEIGHT);
    }
}

fn log_grid_snapshot(drones: &[Drone]) {
    // Create a grid that counts drones at each position
    let mut grid = vec![vec![0; TILES_WIDTH as usize]; TILES_HEIGHT as usize];

    // Count drones at each position
    for drone in drones {
        grid[drone.position.y as usize][drone.position.x as usize] += 1;
    }

    // Print the grid using numbers for multiple drones and '.' for empty spaces
    for row in grid {
        let line: String = row.iter()
            .map(|&count| if count == 0 {
                '.'.to_string()
            } else {
                count.to_string()
            })
            .collect::<Vec<String>>()
            .join("");
        println!("{}", line);
    }

    println!();
    println!();
    println!();
}

fn log_quadrant_snapshot(drones: &[Drone]) {
    let mut grid = vec![vec![0; TILES_WIDTH as usize]; TILES_HEIGHT as usize];

    // Count drones at each position
    for drone in drones {
        grid[drone.position.y as usize][drone.position.x as usize] += 1;
    }

    // Print the grid, but leave middle lines empty
    let mid_x = TILES_WIDTH / 2;
    let mid_y = TILES_HEIGHT / 2;

    for (y, row) in grid.iter().enumerate() {
        let line: String = row.iter().enumerate()
            .map(|(x, &count)| {
                if x as i64 == mid_x || y as i64 == mid_y {
                    ' '.to_string()
                } else if count == 0 {
                    '.'.to_string()
                } else {
                    count.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join("");
        println!("{}", line);
    }

    println!();
    println!();
    println!();
}

fn calculate_safety_factor(drones: &[Drone]) -> i64 {
    let mid_x = TILES_WIDTH / 2;
    let mid_y = TILES_HEIGHT / 2;
    
    let mut quad1 = 0; // top-left
    let mut quad2 = 0; // top-right
    let mut quad3 = 0; // bottom-left
    let mut quad4 = 0; // bottom-right

    for drone in drones {
        // Skip drones that are on the middle lines
        if drone.position.x == mid_x || drone.position.y == mid_y {
            continue;
        }

        match (drone.position.x < mid_x, drone.position.y < mid_y) {
            (true, true) => quad1 += 1,   // top-left
            (false, true) => quad2 += 1,  // top-right
            (true, false) => quad3 += 1,  // bottom-left
            (false, false) => quad4 += 1, // bottom-right
        }
    }

    quad1 * quad2 * quad3 * quad4
}

fn generate_frame_images(drones: &mut Vec<Drone>, output_dir: &PathBuf, num_frames: usize) -> io::Result<()> {
    fs::create_dir_all(output_dir)?;
    
    let mut all_frames_positions = Vec::with_capacity(num_frames);
    for frame in 0..num_frames {
        let positions = drones.iter().map(|drone| {
            (drone.position.x as usize, drone.position.y as usize)
        }).collect::<Vec<_>>();
        all_frames_positions.push(positions);
        simulate_drone_movement(drones);
    }
    
    all_frames_positions.par_iter().enumerate().for_each(|(frame, positions)| {
        let mut img = ImageBuffer::new(
            TILES_WIDTH as u32 * 4,
            TILES_HEIGHT as u32 * 4 + 20  // Extra space for frame number
        );
        
        // Draw frame number at the top
        draw_number(&mut img, frame, 10, 5);
        
        // Draw drone positions
        for &(x, y) in positions {
            for dy in 0..4 {
                for dx in 0..4 {
                    let px = (x as u32 * 4) + dx;
                    let py = (y as u32 * 4) + dy + 20;  // Offset for number space
                    img.put_pixel(px, py, Rgb([255, 255, 255]));
                }
            }
        }
        
        let frame_path = output_dir.join(format!("frame_{:05}.png", frame));
        if let Err(e) = img.save(&frame_path) {
            eprintln!("Error saving frame {}: {}", frame, e);
        }
        
        if frame % 100 == 0 {
            println!("Generated frame {}", frame);
        }
    });
    
    Ok(())
}

fn convert_frames_to_video(frames_dir: &PathBuf, output_video: &PathBuf) -> io::Result<()> {
    use std::process::Command;
    
    println!("Converting frames to video...");
    
    let status = Command::new("ffmpeg")
        .args([
            "-y",
            "-framerate", "30",
            "-i", &frames_dir.join("frame_%05d.png").to_string_lossy(),
            "-c:v", "libx264",
            "-preset", "ultrafast",
            "-crf", "23",
            "-pix_fmt", "yuv420p",
            &output_video.to_string_lossy(),
        ])
        .status()?;
    
    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "FFmpeg command failed"
        ));
    }
    
    Ok(())
}