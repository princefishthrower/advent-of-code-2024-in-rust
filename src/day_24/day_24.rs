use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use crate::utils::read_lines::read_lines;
use std::cmp::Ordering;

// We'll mimic the Python string constants
const XOR: &str = "XOR";
const AND: &str = "AND";
const OR: &str = "OR";

#[derive(Debug)]
struct SimulationError {
    message: String,
}

impl fmt::Display for SimulationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Simulation error: {}", self.message)
    }
}

impl Error for SimulationError {}

#[derive(Clone)]
struct Wire {
    name: String,
    value: RefCell<Option<bool>>,
}

impl Wire {
    fn new(name: String, initial_value: Option<bool>) -> Self {
        Wire {
            name,
            value: RefCell::new(initial_value),
        }
    }
}

struct WireLogicGate {
    input_one_name: String,
    input_two_name: String,
    operation: String,
    output_name: String,
}

impl WireLogicGate {
    fn execute(&self, wires: &HashMap<String, Wire>) -> Result<bool, Box<dyn Error>> {
        let input_one = wires.get(&self.input_one_name)
            .ok_or_else(|| SimulationError { message: format!("Wire {} not found", self.input_one_name) })?;
        let input_two = wires.get(&self.input_two_name)
            .ok_or_else(|| SimulationError { message: format!("Wire {} not found", self.input_two_name) })?;
        
        let input_one_val = input_one.value.borrow()
            .ok_or_else(|| SimulationError { message: format!("Wire {} has no value", self.input_one_name) })?;
        let input_two_val = input_two.value.borrow()
            .ok_or_else(|| SimulationError { message: format!("Wire {} has no value", self.input_two_name) })?;

        Ok(match self.operation.as_str() {
            "AND" => input_one_val && input_two_val,
            "OR" => input_one_val || input_two_val,
            "XOR" => input_one_val ^ input_two_val,
            _ => return Err(Box::new(SimulationError { 
                message: format!("Unknown operation: {}", self.operation) 
            })),
        })
    }

    fn can_execute(&self, wires: &HashMap<String, Wire>) -> bool {
        let input_one = wires.get(&self.input_one_name);
        let input_two = wires.get(&self.input_two_name);

        matches!((input_one, input_two), (Some(w1), Some(w2)) 
            if w1.value.borrow().is_some() && w2.value.borrow().is_some())
    }
}

pub fn run_a() -> Result<(), Box<dyn Error>> {
    let (wires, wire_logic_gates) = parse_wire_input("src/day_24/input.txt")?;
    let result = simulate_system(&wires, &wire_logic_gates)?;
    println!("Decimal numbers of z wires: {}", result);
    Ok(())
}

pub fn run_b() -> std::io::Result<()> {
    
    println!("Stole a rather elegatn python script for this one: https://github.com/guohao/advent-of-code/blob/main/2024/day24/part2.py");

    Ok(())
}

fn parse_wire_input(filename: &str) -> Result<(HashMap<String, Wire>, Vec<WireLogicGate>), Box<dyn Error>> {
    let lines: Vec<String> = read_lines(filename)?;
    let mut wires = HashMap::new();
    let mut wire_logic_gates = Vec::new();
    let mut reading_gates = false;

    for line in lines {
        if line.is_empty() {
            reading_gates = true;
            continue;
        }

        if !reading_gates {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 2 {
                return Err(Box::new(SimulationError { 
                    message: format!("Invalid wire definition: {}", line) 
                }));
            }
            let name = parts[0].trim_end_matches(':').to_string();
            let value = match parts[1] {
                "1" => Some(true),
                "0" => Some(false),
                _ => None,
            };
            wires.insert(name.clone(), Wire::new(name, value));
        } else {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 5 || parts[3] != "->" {
                return Err(Box::new(SimulationError { 
                    message: format!("Invalid gate definition: {}", line) 
                }));
            }
            
            // Create output wire if it doesn't exist
            let output_name = parts[4].to_string();
            if !wires.contains_key(&output_name) {
                wires.insert(output_name.clone(), Wire::new(output_name.clone(), None));
            }

            wire_logic_gates.push(WireLogicGate {
                input_one_name: parts[0].to_string(),
                operation: parts[1].to_string(),
                input_two_name: parts[2].to_string(),
                output_name,
            });
        }
    }

    Ok((wires, wire_logic_gates))
}

fn simulate_system(wires: &HashMap<String, Wire>, wire_logic_gates: &[WireLogicGate]) -> Result<i64, Box<dyn Error>> {
    const MAX_ITERATIONS: usize = 1000;
    let mut iteration = 0;
    let mut executed_gates = HashSet::new();

    while executed_gates.len() < wire_logic_gates.len() && iteration < MAX_ITERATIONS {
        let mut progress = false;

        for (gate_idx, gate) in wire_logic_gates.iter().enumerate() {
            if executed_gates.contains(&gate_idx) || !gate.can_execute(wires) {
                continue;
            }

            let output_value = gate.execute(wires)?;
            if let Some(output_wire) = wires.get(&gate.output_name) {
                output_wire.value.replace(Some(output_value));
                executed_gates.insert(gate_idx);
                progress = true;
            }
        }

        if !progress {
            // Debug output of wire values when we get stuck
            println!("\nCurrent wire values when stuck:");
            for (name, wire) in wires {
                println!("{}: {:?}", name, wire.value.borrow());
            }
            return Err(Box::new(SimulationError { 
                message: "Simulation stuck - possible cycle detected".to_string() 
            }));
        }

        iteration += 1;
    }

    if iteration >= MAX_ITERATIONS {
        return Err(Box::new(SimulationError { 
            message: "Maximum iteration limit reached".to_string() 
        }));
    }

    get_decimal_numbers_of_z_wires(wires)
}

fn get_decimal_numbers_of_z_wires(wires: &HashMap<String, Wire>) -> Result<i64, Box<dyn Error>> {
    let mut z_wires: Vec<_> = wires.iter()
        .filter(|(name, _)| name.starts_with('z'))
        .collect();
    
    // Sort by the numeric portion of the wire name, with smaller numbers being less significant
    z_wires.sort_by(|(name1, _), (name2, _)| {
        let num1 = name1[1..].parse::<usize>().unwrap_or(0);
        let num2 = name2[1..].parse::<usize>().unwrap_or(0);
        num2.cmp(&num1)  // Reversed comparison
    });

    let binary_number = z_wires.iter()
        .map(|(name, wire)| {
            wire.value.borrow()
                .ok_or_else(|| SimulationError { 
                    message: format!("Wire {} has no value", name) 
                })
                .map(|v| if v { "1" } else { "0" })
        })
        .collect::<Result<String, _>>()?;

    let decimal = i64::from_str_radix(&binary_number, 2)?;
    Ok(decimal)
}

// additional helpers for part B
fn calculate_expected_sum(x_values: &[usize], y_values: &[usize]) -> i64 {
    // Convert x_values binary array to decimal
    let x_decimal = x_values.iter()
        .rev()  // Reverse since [0] is least significant bit
        .fold(0, |acc, &bit| (acc << 1) | bit);

    // Convert y_values binary array to decimal
    let y_decimal = y_values.iter()
        .rev()  // Reverse since [0] is least significant bit
        .fold(0, |acc, &bit| (acc << 1) | bit);

    // Return sum
    (x_decimal + y_decimal) as i64
}

fn compare_results(actual: i64, expected: i64) -> Vec<String> {
    let mut differences = Vec::new();
    let mut actual_bits = actual;
    let mut expected_bits = expected;
    let mut bit_pos = 0;

    // Compare each bit position
    while actual_bits > 0 || expected_bits > 0 {
        let actual_bit = actual_bits & 1;
        let expected_bit = expected_bits & 1;

        if actual_bit != expected_bit {
            // When we find a difference, we need to track which wire position had it
            // This could be a z wire or any other wire in the circuit
            differences.push(format!("z{:02}", bit_pos));
        }

        actual_bits >>= 1;
        expected_bits >>= 1;
        bit_pos += 1;
    }

    differences
}