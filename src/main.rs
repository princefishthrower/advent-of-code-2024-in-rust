mod day_10;
mod utils;

fn main() {
    if let Err(e) = day_10::day_10::run_a() {
        eprintln!("Error running day 10 part A: {}", e);
    }
    if let Err(e) = day_10::day_10::run_b() {
        eprintln!("Error running day 10 part A: {}", e);
    }
}


