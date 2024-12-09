mod day_9;
mod utils;

fn main() {
    if let Err(e) = day_9::day_9::run_a() {
        eprintln!("Error running day 9 part A: {}", e);
    }
    if let Err(e) = day_9::day_9::run_b() {
        eprintln!("Error running day 9 part A: {}", e);
    }
}


