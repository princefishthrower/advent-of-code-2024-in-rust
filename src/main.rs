use std::time::Instant;

mod day_13;
mod utils;

fn main() {
    let start = Instant::now();
    if let Err(e) = day_13::day_13::run_a() {
        eprintln!("Error running day 13 part A: {}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_13::day_13::run_b() {
        eprintln!("Error running day 13 part B: {}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);
}


