use std::time::Instant;

mod day_13;
mod day_15;
mod utils;

fn main() {
    // still haven't solved day 13 part b
    let start = Instant::now();
    if let Err(e) = day_13::day_13::run_b() {
        eprintln!("Error running day 13 part B: {}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_15::day_15::run_a() {
        eprintln!("Error running day 15 part A: {}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_15::day_15::run_b() {
        eprintln!("Error running day 15 part B: {}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);
}


