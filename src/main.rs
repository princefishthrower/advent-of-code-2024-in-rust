use std::time::Instant;

mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_2;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod utils;

fn main() {
    // for each day, call run_a and run_b, except for day 25, which only has run_a
    let start_total = Instant::now();

    let start = Instant::now();
    if let Err(e) = day_1::day_1::run_a() {
        eprintln!("Error running day 1 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_1::day_1::run_b() {
        eprintln!("Error running day 1 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    // now do the same for all other days
    let start = Instant::now();
    if let Err(e) = day_2::day_2::run_a() {
        eprintln!("Error running day 2 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_2::day_2::run_b() {
        eprintln!("Error running day 2 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_3::day_3::run_a() {
        eprintln!("Error running day 3 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_3::day_3::run_b() {
        eprintln!("Error running day 3 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_4::day_4::run_a() {
        eprintln!("Error running day 4 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_4::day_4::run_b() {
        eprintln!("Error running day 4 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_5::day_5::run_a() {
        eprintln!("Error running day 5 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_5::day_5::run_b() {
        eprintln!("Error running day 5 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_6::day_6::run_a() {
        eprintln!("Error running day 6 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_6::day_6::run_b() {
        eprintln!("Error running day 6 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_7::day_7::run_a() {
        eprintln!("Error running day 7 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_7::day_7::run_b() {
        eprintln!("Error running day 7 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_8::day_8::run_a() {
        eprintln!("Error running day 8 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_8::day_8::run_b() {
        eprintln!("Error running day 8 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_9::day_9::run_a() {
        eprintln!("Error running day 9 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_9::day_9::run_b() {
        eprintln!("Error running day 9 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_1::day_1::run_a() {
        eprintln!("Error running day 1 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_1::day_1::run_b() {
        eprintln!("Error running day 1 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_2::day_2::run_a() {
        eprintln!("Error running day 2 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_2::day_2::run_b() {
        eprintln!("Error running day 2 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_3::day_3::run_a() {
        eprintln!("Error running day 3 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_3::day_3::run_b() {
        eprintln!("Error running day 3 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_4::day_4::run_a() {
        eprintln!("Error running day 4 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_4::day_4::run_b() {
        eprintln!("Error running day 4 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_5::day_5::run_a() {
        eprintln!("Error running day 5 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_5::day_5::run_b() {
        eprintln!("Error running day 5 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_6::day_6::run_a() {
        eprintln!("Error running day 6 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_6::day_6::run_b() {
        eprintln!("Error running day 6 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_7::day_7::run_a() {
        eprintln!("Error running day 7 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_7::day_7::run_b() {
        eprintln!("Error running day 7 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_8::day_8::run_a() {
        eprintln!("Error running day 8 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_8::day_8::run_b() {
        eprintln!("Error running day 8 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_9::day_9::run_a() {
        eprintln!("Error running day 9 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_9::day_9::run_b() {
        eprintln!("Error running day 9 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_10::day_10::run_a() {
        eprintln!("Error running day 10 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_10::day_10::run_b() {
        eprintln!("Error running day 10 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_11::day_11::run_a() {
        eprintln!("Error running day 11 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_11::day_11::run_b() {
        eprintln!("Error running day 11 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_12::day_12::run_a() {
        eprintln!("Error running day 12 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_12::day_12::run_b() {
        eprintln!("Error running day 12 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_13::day_13::run_a() {
        eprintln!("Error running day 13 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_13::day_13::run_b() {
        eprintln!("Error running day 13 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_14::day_14::run_a() {
        eprintln!("Error running day 14 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_14::day_14::run_b() {
        eprintln!("Error running day 14 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_15::day_15::run_a() {
        eprintln!("Error running day 15 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_15::day_15::run_b() {
        eprintln!("Error running day 15 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_16::day_16::run_a() {
        eprintln!("Error running day 16 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_16::day_16::run_b() {
        eprintln!("Error running day 16 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_17::day_17::run_a() {
        eprintln!("Error running day 17 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_17::day_17::run_b() {
        eprintln!("Error running day 17 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_18::day_18::run_a() {
        eprintln!("Error running day 18 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_18::day_18::run_b() {
        eprintln!("Error running day 18 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_19::day_19::run_a() {
        eprintln!("Error running day 19 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_19::day_19::run_b() {
        eprintln!("Error running day 19 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_20::day_20::run_a() {
        eprintln!("Error running day 20 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_20::day_20::run_b() {
        eprintln!("Error running day 20 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_21::day_21::run_a() {
        eprintln!("Error running day 21 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_21::day_21::run_b() {
        eprintln!("Error running day 21 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_22::day_22::run_a() {
        eprintln!("Error running day 22 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_22::day_22::run_b() {
        eprintln!("Error running day 22 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_23::day_23::run_a() {
        eprintln!("Error running day 23 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_23::day_23::run_b() {
        eprintln!("Error running day 23 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_24::day_24::run_a() {
        eprintln!("Error running day 24 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_24::day_24::run_b() {
        eprintln!("Error running day 24 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let start = Instant::now();
    if let Err(e) = day_25::day_25::run_a() {
        eprintln!("Error running day 25 part A: {:?}", e);
    }
    let duration_a = start.elapsed();
    println!("run_a took: {:?}", duration_a);

    let start = Instant::now();
    if let Err(e) = day_25::day_25::run_b() {
        eprintln!("Error running day 25 part B: {:?}", e);
    }
    let duration_b = start.elapsed();
    println!("run_b took: {:?}", duration_b);

    let duration_total = start_total.elapsed();
    println!("Total time: {:?}", duration_total);
    println!("Average time per part: {:?}", duration_total / 49);
}
