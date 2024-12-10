use std::time::Instant;

mod day_11;
mod utils;

fn main() {
    let start = Instant::now();
    if let Err(e) = day_11::day_11::run_a() {
        eprintln!("Error running day 10 part A: {}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_11::day_11::run_b() {
        eprintln!("Error running day 10 part A: {}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);
}


