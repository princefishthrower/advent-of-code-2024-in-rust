mod day_8;
mod utils;

fn main() {
    if let Err(e) = day_8::day_8::run_a() {
        eprintln!("Error running day 8 part A: {}", e);
    }
    if let Err(e) = day_8::day_8::run_b() {
        eprintln!("Error running day 8 part A: {}", e);
    }
}


