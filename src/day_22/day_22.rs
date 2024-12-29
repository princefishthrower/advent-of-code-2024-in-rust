use crate::utils::read_lines_as_int_arrays::read_lines_as_int_arrays;
use bitvec::prelude::*;

pub fn run_a() -> std::io::Result<()> {
    let input = read_lines_as_int_arrays("src/day_22/input.txt", " ");

    // each line is just a single number, so put it into a big vec of i64
    let starting_numbers = input.unwrap().into_iter().flatten().map(|x| x as u64).collect::<Vec<u64>>();

    // print and collect the results
    let mut final_secret_numbers = Vec::new();
    for starting_number in starting_numbers {
        let final_secret_number = apply_secret_number_rules_for_n_iterations(starting_number, 2000);
        // println!("Final secret number for starting number {}: {}", starting_number, final_secret_number);
        final_secret_numbers.push(final_secret_number);
    }

    // print sum of all final secret numbers
    let sum: u64 = final_secret_numbers.iter().sum();

    println!("Sum of all final secret numbers: {}", sum);

    Ok(())
}

pub fn run_b() -> std::io::Result<()> {
    let input = read_lines_as_int_arrays("src/day_22/input.txt", " ");

    // each line is just a single number, so put it into a big vec of i64
    let starting_numbers = input.unwrap().into_iter().flatten().map(|x| x as i64).collect::<Vec<i64>>();

    // Map to store total bananas for each possible sequence of 4 deltas
    // Using a vector indexed by delta sequences instead of a HashMap for performance
    let mut map = vec![0i16; 19usize.pow(4)];

    for &start in &starting_numbers {
        // Track seen sequences for this monkey to avoid double-counting
        let mut seen = bitvec![0; 19usize.pow(4)];
        
        let mut secret = start as u64;
        let mut old_price = (secret % 10) as i8;
        let mut deltas = Vec::new();

        // Generate 2000 prices and their deltas
        for _ in 0..2000 {
            secret = get_next_secret_number(secret);
            let price = (secret % 10) as i8;
            let delta = price - old_price;
            deltas.push(delta);
            old_price = price;

            // Need at least 4 deltas to form a sequence
            if deltas.len() < 4 {
                continue;
            }

            // Get the last 4 deltas
            let n = deltas.len();
            let sequence = (
                deltas[n - 4],
                deltas[n - 3],
                deltas[n - 2],
                deltas[n - 1]
            );

            // Calculate index for this sequence
            let idx = index_from_deltas(sequence);
            
            // Skip if we've already seen this sequence for this monkey
            if seen[idx] {
                continue;
            }

            seen.set(idx, true);
            map[idx] += price as i16;
        }
    }

    // Find the maximum total bananas
    let result = map.into_iter().max().unwrap();
    println!("Maximum total bananas: {}", result);
    
    Ok(())
}

fn apply_secret_number_rules_for_n_iterations(input: u64, n: u64) -> u64 {
    let mut secret_number = input;
    for _ in 0..n {
        secret_number = get_next_secret_number(secret_number);
    }
    secret_number
}

fn get_next_secret_number(mut secret: u64) -> u64 {
    // Multiply by 64
    let mult_64_result = secret * 64;
    secret = mix(mult_64_result, secret);
    secret = prune(secret);

    // Divide by 32
    let div_result = secret / 32;
    secret = mix(div_result, secret);
    secret = prune(secret);

    // Multiply by 2048
    let mult_2048_result = secret * 2048;
    secret = mix(mult_2048_result, secret);
    secret = prune(secret);

    secret
}

fn mix(value: u64, secret: u64) -> u64 {
    value ^ secret
}

fn prune(secret: u64) -> u64 {
    secret % 16777216
}
// Convert a sequence of 4 deltas into an index
fn index_from_deltas(deltas: (i8, i8, i8, i8)) -> usize {
    // Shift deltas to be 0-based indices (from -9..=9 to 0..=18)
    let (d1, d2, d3, d4) = deltas;
    let i1 = (d1 + 9) as usize;
    let i2 = (d2 + 9) as usize;
    let i3 = (d3 + 9) as usize;
    let i4 = (d4 + 9) as usize;
    
    // Calculate unique index using base-19 system
    i1 * 19 * 19 * 19 + i2 * 19 * 19 + i3 * 19 + i4
}