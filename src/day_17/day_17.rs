use std::str::FromStr;
use std::fs;
use std::path::Path;

#[derive(Debug)]
struct StrangeDevice {
    register_a: i64,
    register_b: i64,
    register_c: i64,
    instruction_pointer: usize,
    program: Vec<i64>,
    output: Vec<i64>,
}

#[derive(Debug)]
pub enum ParseError {
    InvalidFormat,
    InvalidNumber,
    FileError(std::io::Error),
}

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> ParseError {
        ParseError::FileError(err)
    }
}

impl FromStr for StrangeDevice {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().filter(|l| !l.is_empty()).collect();
        
        println!("Debug: Found {} non-empty lines", lines.len());
        for (i, line) in lines.iter().enumerate() {
            println!("Debug: Line {}: {}", i, line);
        }
        
        if lines.len() < 4 {
            return Err(ParseError::InvalidFormat);
        }
        
        // Parse registers
        let reg_a = lines[0].strip_prefix("Register A: ")
            .ok_or(ParseError::InvalidFormat)?
            .parse::<i64>()
            .map_err(|_| ParseError::InvalidNumber)?;
            
        let reg_b = lines[1].strip_prefix("Register B: ")
            .ok_or(ParseError::InvalidFormat)?
            .parse::<i64>()
            .map_err(|_| ParseError::InvalidNumber)?;
            
        let reg_c = lines[2].strip_prefix("Register C: ")
            .ok_or(ParseError::InvalidFormat)?
            .parse::<i64>()
            .map_err(|_| ParseError::InvalidNumber)?;

        // Parse program - now using lines[3] instead of lines[4]
        let program = lines[3].strip_prefix("Program: ")
            .ok_or(ParseError::InvalidFormat)?
            .split(',')
            .map(|n| n.trim().parse::<i64>())
            .collect::<Result<Vec<i64>, _>>()
            .map_err(|_| ParseError::InvalidNumber)?;

        Ok(StrangeDevice {
            register_a: reg_a,
            register_b: reg_b,
            register_c: reg_c,
            instruction_pointer: 0,
            program,
            output: Vec::new(),
        })
    }
}

impl StrangeDevice {
    fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ParseError> {
        let contents = fs::read_to_string(path)?;
        Self::from_str(&contents)
    }

    fn get_combo_operand_value(&self, operand: i64) -> i64 {
        match operand {
            0..=3 => operand,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            7 => panic!("Invalid combo operand 7"),
            _ => panic!("Invalid combo operand"),
        }
    }

    fn execute_instruction(&mut self) -> bool {
        if self.instruction_pointer >= self.program.len() {
            return false;
        }

        let opcode = self.program[self.instruction_pointer];
        let operand = self.program[self.instruction_pointer + 1];

        match opcode {
            0 => { // adv
                let power = match operand {
                    0..=3 => operand,
                    4 => self.register_a,
                    5 => self.register_b,
                    6 => self.register_c,
                    _ => panic!("Invalid operand")
                } % 8;
                let divisor = 2_i64.pow(power as u32);
                self.register_a = self.register_a / divisor;
                self.instruction_pointer += 2;
            },
            1 => { // bxl
                self.register_b ^= operand;
                self.instruction_pointer += 2;
            },
            2 => { // bst
                self.register_b = self.get_combo_operand_value(operand) % 8;
                self.instruction_pointer += 2;
            },
            3 => { // jnz
                if self.register_a != 0 {
                    self.instruction_pointer = operand as usize;
                } else {
                    self.instruction_pointer += 2;
                }
            },
            4 => { // bxc
                self.register_b ^= self.register_c;
                self.instruction_pointer += 2;
            },
            5 => { // out
                let value = self.get_combo_operand_value(operand) % 8;
                self.output.push(value);
                self.instruction_pointer += 2;
            },
            6 => { // bdv
                let power = self.get_combo_operand_value(operand);
                let divisor = 2_i64.pow(power as u32);
                self.register_b = self.register_a / divisor;
                self.instruction_pointer += 2;
            },
            7 => { // cdv
                let power = self.get_combo_operand_value(operand);
                let divisor = 2_i64.pow(power as u32);
                self.register_c = self.register_a / divisor;
                self.instruction_pointer += 2;
            },
            _ => panic!("Invalid opcode"),
        }

        println!("After - A: {}, B: {}, C: {}", self.register_a, self.register_b, self.register_c);
        true
    }

    fn get_program_output(&self) -> String {
        self.output.iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn run(&mut self) -> String {
        while self.execute_instruction() {}
        self.get_program_output()
    }
}

pub fn run_a() -> Result<(), ParseError> {
    let mut strange_device = StrangeDevice::from_file("src/day_17/input.txt")?;

    let output = strange_device.run();

    // print out the comma joined string
    println!("{}", output.replace(",", ""));

    Ok(())
}

pub fn run_b() -> Result<(), ParseError> {
    // Read and parse the program
    let device = StrangeDevice::from_file("src/day_17/input.txt")?;
    let program = device.program;
    
    // Find the minimum value for register A
    match get_register_a(&program, 0) {
        Some(a_value) => println!("Minimum register A value: {}", a_value),
        None => println!("No solution found"),
    }
    
    Ok(())
}

type Num = i64;

fn get_register_a(program: &[Num], mut a_prev: Num) -> Option<Num> {
    if program.is_empty() {
        return Some(a_prev);
    }
    
    let mut program = Vec::from(program);
    let b_out = program.pop().unwrap();
    a_prev <<= 3;
    
    let mut options = Vec::new();
    for mut b in 0..8 {
        let b_prev = b ^ (a_prev >> b);
        let b_prev = b_prev - (b_prev & 7) + b_out;
        let c = b ^ 5 ^ b_prev;
        let a_min = c << b;
        let a_max = (c + 1) << b;
        b ^= 3;
        let a_next = a_prev + b;
        
        if a_min < a_prev + 8 && a_prev < a_max && a_min <= a_next && a_next < a_max {
            options.push(a_next);
        }
    }
    
    options.sort();
    for a_next in options {
        let a_option = get_register_a(&program, a_next);
        if a_option.is_some() {
            return a_option;
        }
    }
    None
}

