use crate::utils::read_lines::read_lines;

// create a type with 'solution', i64 and 'numbers', vec i64
struct Solution {
    solution: i64,
    numbers: Vec<i64>,
}

pub fn run_a() -> std::io::Result<()> {
    return run_for_given_operators(&["+", "*"]);
}

pub fn run_b() -> std::io::Result<()> {
    return run_for_given_operators(&["+", "*", "||"]);
}

fn run_for_given_operators(operators: &[&str]) -> std::io::Result<()> {
    let lines = read_lines("src/day_7/input.txt")?;
    let mut solutions: Vec<Solution> = Vec::new();

    for line in lines {
        let input: Vec<&str> = line.split(":").collect();
        let solution = input[0].parse::<i64>().unwrap();
        let numbers: Vec<i64> = input[1].trim().split(" ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        solutions.push(Solution {
            solution: solution,
            numbers: numbers,
        });
    }

    let mut true_solutions: Vec<i64> = Vec::new();
    
    for solution in solutions {
        let num_operators = solution.numbers.len() - 1;
        let operator_combinations = generate_all_operator_combinations(&operators, num_operators);
        let mut found = false;

        // println!("\nTrying to find: {} with numbers {:?}", solution.solution, solution.numbers);
        
        for op_combo in operator_combinations {
            let (result, expression) = evaluate_expression(&solution.numbers, &op_combo);
            
            if result == solution.solution {
                // println!("Found: {} = {}", expression, result);
                true_solutions.push(solution.solution);  // Note: pushing solution.solution, not result
                found = true;
                break;
            }
        }
        
        if !found {
            // println!("NO SOLUTION FOUND for {}: {:?}", solution.solution, solution.numbers);
        }
    }

    let sum = true_solutions.iter().sum::<i64>();
    println!("\nSum of true solutions: {}", sum);
    Ok(())
}

fn generate_all_operator_combinations<'a>(operators: &[&'a str], length: usize) -> Vec<Vec<&'a str>> {
    let mut result = Vec::new();
    
    fn generate_recursive<'a>(
        current: &mut Vec<&'a str>,
        operators: &[&'a str],
        length: usize,
        result: &mut Vec<Vec<&'a str>>
    ) {
        if current.len() == length {
            result.push(current.clone());
            return;
        }
        
        for &op in operators {
            current.push(op);
            generate_recursive(current, operators, length, result);
            current.pop();
        }
    }
    
    generate_recursive(&mut Vec::new(), operators, length, &mut result);
    result
}

fn evaluate_expression(numbers: &[i64], operators: &[&str]) -> (i64, String) {
    let mut result = numbers[0];
    let mut expression = format!("{}", numbers[0]);
    
    for i in 0..operators.len() {
        match operators[i] {
            "+" => {
                result += numbers[i + 1];
                expression.push_str(&format!(" + {}", numbers[i + 1]));
            },
            "*" => {
                result *= numbers[i + 1];
                expression.push_str(&format!(" * {}", numbers[i + 1]));
            },
            "||" => {
                // Convert both numbers to strings, concatenate, then parse back to i64
                let concat_str = format!("{}{}", result, numbers[i + 1]);
                result = concat_str.parse::<i64>().unwrap();
                expression.push_str(&format!(" || {}", numbers[i + 1]));
            },
            _ => unreachable!("Unknown operator")
        }
    }
    
    (result, expression)
}