use std::fs;
use std::io::{Error, stdin};

/// Extending day2 solution
/// Add opcodes:
/// 3 (input): request user input and store to parameter
/// 4 (output): output value at parameter

/// Add parameter modes 0 (parameter interpret as address), 1 (parameter interpret as value)
/// parameter mode passed with the opcodethread::spawn(move || {
/// opcode is right-most two digits
/// from right to left one opcode for each parameter, 0 if missing

/// Read file "input" and return content
fn read_input() -> Result<String, Error> {
    let content = fs::read_to_string("./input")
    .expect("Error while opening input file.");

    Ok(content)
}

/// Get either the value as position `parameter` in mode 0
/// or return parameter in mode 1
fn get_value(instructions: &Vec<i32>, parameter: i32, mode: u8) -> i32 {
    match mode {
        0 => {
            if parameter < 0 {
                panic!("Tried to access negative memory.");
            }
            instructions[parameter as usize]
        },
        1 => {
            parameter
        },
        _ => {
            panic!("Passed unknown parameter mode: {}", mode);
        }
    }
}

fn add(instructions: &mut Vec<i32>, ip: usize, pmodes: [u8; 3]) {
    // get first two parameter values
    let p1 = get_value(&instructions, instructions[ip+1], pmodes[0]);
    let p2 = get_value(&instructions, instructions[ip+2], pmodes[1]);

    // store result at third position, must always be position mode
    if pmodes[2] != 0 {
        panic!("Write instruction is called in immediate mode");
    }
    let p3 = instructions[ip+3];
    if p3 < 0 {
        panic!("Trying to store result of addition to negative memory.");
    }
    instructions[p3 as usize] = p1 + p2;
}

fn multiply(instructions: &mut Vec<i32>, ip: usize, pmodes: [u8; 3]) {
    // get first two parameter values
    let p1 = get_value(&instructions, instructions[ip+1], pmodes[0]);
    let p2 = get_value(&instructions, instructions[ip+2], pmodes[1]);

    // store result at third position, must always be position mode
    if pmodes[2] != 0 {
        panic!("Write instruction is called in immediate mode");
    }
    let p3 = instructions[ip+3];
    if p3 < 0 {
        panic!("Trying to store result of multiplication to negative memory.");
    }
    instructions[p3 as usize] = p1 * p2;
}

fn input(instructions: &mut Vec<i32>, ip: usize, pmodes: [u8; 3]) {
    if pmodes[0] != 0 {
        panic!("Input instruction is called in immediate mode");
    }
    let p1 = instructions[ip+1];
    if p1 < 0 {
        panic!("Tried to output from negative memory.");
    }
    let mut input = String::new();
    println!("Requesting input to store at position {} :", p1);

    stdin().read_line(&mut input)
    .expect("Error occured trying to read user input.");
    
    let input_value: i32 = input.trim().parse()
    .expect("Did not pass number.");

    instructions[p1 as usize] = input_value;
}

fn output(instructions: &Vec<i32>, ip: usize, pmodes: [u8; 3]) {
    let p1 = get_value(&instructions, instructions[ip+1], pmodes[0]);
    println!("Output: {}", p1)
}

fn part1() {
    let intcode_string = read_input().unwrap();
    let mut intcodes = intcode_string.split(",")
    .map(|opcode| opcode.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();

    // let mut intcodes = vec![1101, 100, -1, 4, 0];
    let mut ip = 0; // instruction pointer
    loop {
        let instruction = intcodes[ip];
        // read instruction, first seperate by digits and transform to vec of digits
        let instruct_digits: Vec<u8> = instruction.to_string().chars()
        .map(|d| d.to_digit(10).unwrap() as u8)
        .collect();

        let n_digits = instruct_digits.len();
        let opcode: u8;
        let mut pmodes: [u8; 3] = [0, 0, 0];
        if n_digits == 1 {
            // special case of single digit opcode
            opcode = instruct_digits[0];
        } else {
            // last two digits are opcode
            opcode = instruct_digits[n_digits-2]*10+instruct_digits[n_digits-1];
            for i in 0..n_digits-2 {
                pmodes[i] = instruct_digits[n_digits-3 - i];
            }
        }
        match opcode {
            1 => {
                add(&mut intcodes, ip, pmodes);
                ip += 4;
            },
            2 => {
                multiply(&mut intcodes, ip, pmodes);
                ip += 4;
            },
            3 => {
                input(&mut intcodes, ip, pmodes);
                ip += 2;
            },
            4 => {
                output(&intcodes, ip, pmodes);
                ip += 2;
            },
            99 => {
                break;
            },
            _ => {
                panic!(
                    "Error while reading intcodes. Unknown opcode {} at \
                    position: {}", intcodes[ip], ip);
            }
        }
    }
}




fn less_than(instructions: &mut Vec<i32>, ip: usize, pmodes: [u8; 3]) {
    // get first two parameter values
    let p1 = get_value(&instructions, instructions[ip+1], pmodes[0]);
    let p2 = get_value(&instructions, instructions[ip+2], pmodes[1]);

    // store result at third position, must always be position mode
    if pmodes[2] != 0 {
        panic!("Write instruction is called in immediate mode");
    }
    let p3 = instructions[ip+3];
    if p3 < 0 {
        panic!("Trying to set negative memory.");
    }
    if p1 < p2 {
        instructions[p3 as usize] = 1;
    } else {
        instructions[p3 as usize] = 0;
    }
}

fn equals(instructions: &mut Vec<i32>, ip: usize, pmodes: [u8; 3]) {
    // get first two parameter values
    let p1 = get_value(&instructions, instructions[ip+1], pmodes[0]);
    let p2 = get_value(&instructions, instructions[ip+2], pmodes[1]);

    // store result at third position, must always be position mode
    if pmodes[2] != 0 {
        panic!("Write instruction is called in immediate mode");
    }
    let p3 = instructions[ip+3];
    if p3 < 0 {
        panic!("Trying to set negative memory.");
    }
    if p1 == p2 {
        instructions[p3 as usize] = 1;
    } else {
        instructions[p3 as usize] = 0;
    }
}

/// Add opcodes, 5, 6, 7, 8
fn part2() {
    let intcode_string = read_input().unwrap();
    let mut intcodes = intcode_string.split(",")
    .map(|opcode| opcode.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();

    // let mut intcodes = vec![1101, 100, -1, 4, 0];
    let mut ip = 0; // instruction pointer
    loop {
        let instruction = intcodes[ip];
        // read instruction, first seperate by digits and transform to vec of digits
        let instruct_digits: Vec<u8> = instruction.to_string().chars()
        .map(|d| d.to_digit(10).unwrap() as u8)
        .collect();

        let n_digits = instruct_digits.len();
        let opcode: u8;
        let mut pmodes: [u8; 3] = [0, 0, 0];
        if n_digits == 1 {
            // special case of single digit opcode
            opcode = instruct_digits[0];
        } else {
            // last two digits are opcode
            opcode = instruct_digits[n_digits-2]*10+instruct_digits[n_digits-1];
            for i in 0..n_digits-2 {
                pmodes[i] = instruct_digits[n_digits-3 - i];
            }
        }
        match opcode {
            1 => {
                add(&mut intcodes, ip, pmodes);
                ip += 4;
            },
            2 => {
                multiply(&mut intcodes, ip, pmodes);
                ip += 4;
            },
            3 => {
                input(&mut intcodes, ip, pmodes);
                ip += 2;
            },
            4 => {
                output(&intcodes, ip, pmodes);
                ip += 2;
            },
            5 => {
                // jump-if-true
                let p1 = get_value(&intcodes, intcodes[ip+1], pmodes[0]);
                let p2 = get_value(&intcodes, intcodes[ip+2], pmodes[1]);
                if p1 != 0 {
                    if p2 < 0 {
                        panic!("Trying to jump to negative memory");
                    }
                    ip = p2 as usize;
                } else {
                    ip += 3;
                }
            },
            6 => {
                // jump-if-false
                let p1 = get_value(&intcodes, intcodes[ip+1], pmodes[0]);
                let p2 = get_value(&intcodes, intcodes[ip+2], pmodes[1]);
                if p1 == 0 {
                    if p2 < 0 {
                        panic!("Trying to jump to negative memory");
                    }
                    ip = p2 as usize;
                } else {
                    ip += 3;
                }
            },
            7 => {
                // less than
                less_than(&mut intcodes, ip, pmodes);
                ip += 4;
            },
            8 => {
                // equals
                equals(&mut intcodes, ip, pmodes);
                ip += 4;
            },
            99 => {
                break;
            },
            _ => {
                panic!(
                    "Error while reading intcodes. Unknown opcode {} at \
                    position: {}", intcodes[ip], ip);
            }
        }
    }
}


fn main() {
    // part1();
    part2();
}
