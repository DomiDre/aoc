use std::fs;
use std::io::{Error};

/// Extending day5 solution

/// Read file "input" and return content
fn read_input() -> Result<String, Error> {
    let content = fs::read_to_string("./input")
    .expect("Error while opening input file.");

    Ok(content)
}

/// Get either the value as position `parameter` in mode 0
/// or return parameter in mode 1
fn get_value(instructions: &[i32], parameter: i32, mode: u8) -> i32 {
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

fn add(instructions: &mut [i32], ip: usize, pmodes: [u8; 3]) {
    // get first two parameter values
    let p1 = get_value(&instructions, instructions[ip+1], pmodes[0]);
    let p2 = get_value(&instructions, instructions[ip+2], pmodes[1]);

    // store result at third position, must always be position mode
    if pmodes[2] != 0 {
        panic!("Write instruction is called in immedi_ate mode");
    }
    let p3 = instructions[ip+3];
    if p3 < 0 {
        panic!("Trying to store result of addition to negative memory.");
    }
    instructions[p3 as usize] = p1 + p2;
}

fn multiply(instructions: &mut [i32], ip: usize, pmodes: [u8; 3]) {
    // get first two parameter values
    let p1 = get_value(&instructions, instructions[ip+1], pmodes[0]);
    let p2 = get_value(&instructions, instructions[ip+2], pmodes[1]);

    // store result at third position, must always be position mode
    if pmodes[2] != 0 {
        panic!("Write instruction is called in immedi_ate mode");
    }
    let p3 = instructions[ip+3];
    if p3 < 0 {
        panic!("Trying to store result of multiplication to negative memory.");
    }
    instructions[p3 as usize] = p1 * p2;
}

fn less_than(instructions: &mut [i32], ip: usize, pmodes: [u8; 3]) {
    // get first two parameter values
    let p1 = get_value(&instructions, instructions[ip+1], pmodes[0]);
    let p2 = get_value(&instructions, instructions[ip+2], pmodes[1]);

    // store result at third position, must always be position mode
    if pmodes[2] != 0 {
        panic!("Write instruction is called in immedi_ate mode");
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

fn equals(instructions: &mut [i32], ip: usize, pmodes: [u8; 3]) {
    // get first two parameter values
    let p1 = get_value(&instructions, instructions[ip+1], pmodes[0]);
    let p2 = get_value(&instructions, instructions[ip+2], pmodes[1]);

    // store result at third position, must always be position mode
    if pmodes[2] != 0 {
        panic!("Write instruction is called in immedi_ate mode");
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

/// calculate output for given intcode table, with inputs and starting from ip
/// makes a break at every output to be restarted from ip
/// return tuple of current (ip, output). If opcode 99 is reached, ip i set to -1
/// to flag termination
fn calculate_output(intcodes: &mut [i32], inputs: [usize; 2], ip: usize) -> (i32, i32) {
    let mut ip = ip; // instruction pointer
    let mut input_counter = 0;
    let mut output = -1;
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
            // speci_al case of single digit opcode
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
                add(intcodes, ip, pmodes);
                ip += 4;
            },
            2 => {
                multiply(intcodes, ip, pmodes);
                ip += 4;
            },
            3 => {
                // input
                if pmodes[0] != 0 {
                    panic!("Input instruction is not called in position mode");
                }
                let p1 = intcodes[ip+1];
                if p1 < 0 {
                    panic!("Tried to input to negative memory.");
                }
                intcodes[p1 as usize] = inputs[input_counter] as i32;
                input_counter += 1;
                ip += 2;
            },
            4 => {
                //output
                let p1 = get_value(&intcodes, intcodes[ip+1], pmodes[0]);
                output = p1;
                ip += 2;
                println!("OUTPUT: {}", output);
                return (ip as i32, output)
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
                less_than(intcodes, ip, pmodes);
                ip += 4;
            },
            8 => {
                // equals
                equals(intcodes, ip, pmodes);
                ip += 4;
            },
            99 => {
                return (-1, output);
            },
            _ => {
                panic!(
                    "Error while reading intcodes. Unknown opcode {} at \
                    position: {}", intcodes[ip], ip);
            }
        }
    }
}






fn part1() {
    let intcode_string = read_input().unwrap();
    let intcodes = intcode_string.split(",")
    .map(|opcode| opcode.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();

    let mut highest_signal = 0;
    for i_a in 0..5 {
        for i_b in 0..5 {
            if i_b == i_a {
                continue
            }
            for i_c in 0..5 {
                if i_c == i_a || i_c == i_b {
                    continue
                }
                for i_d in 0..5 {
                    if i_d == i_a || i_d == i_b || i_d == i_c {
                        continue
                    }
                    for i_e in 0..5 {
                        if i_e == i_a || i_e == i_b || i_e == i_c || i_e == i_d {
                            continue
                        }
                        let (_, prev_output) = calculate_output(
                            intcodes.clone().as_mut_slice(),
                            [i_a, 0], 0);
                        let (_, prev_output) = calculate_output(
                            intcodes.clone().as_mut_slice(),
                            [i_b, prev_output as usize], 0);
                        let (_, prev_output) = calculate_output(
                            intcodes.clone().as_mut_slice(),
                            [i_c, prev_output as usize], 0);
                        let (_, prev_output) = calculate_output(
                            intcodes.clone().as_mut_slice(),
                            [i_d, prev_output as usize], 0);
                        let (_, output)      = calculate_output(
                            intcodes.clone().as_mut_slice(),
                            [i_e, prev_output as usize], 0);
                        if output > highest_signal {
                            highest_signal = output;
                        }
                    }
                }
            }
        }
    }
    println!("{}", highest_signal);
}



/// calculate output for given intcode table, with inputs and starting from ip
/// makes a break at every output to be restarted from ip
/// return tuple of current (ip, output). If opcode 99 is reached, ip i set to -1
/// to flag termination
fn part2_machine(intcodes: &mut [i32], input: i32, ip: usize) -> (i32, i32) {
    let mut ip = ip; // instruction pointer
    let mut input = input;
    let mut output = -1;

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
            // speci_al case of single digit opcode
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
                add(intcodes, ip, pmodes);
                ip += 4;
            },
            2 => {
                multiply(intcodes, ip, pmodes);
                ip += 4;
            },
            3 => {
                // input
                if input == -1 {
                    // previous input was already consumed, halt and wait for
                    // new input
                    return (ip as i32, output)
                }
                if pmodes[0] != 0 {
                    panic!("Input instruction is not called in position mode");
                }
                let p1 = intcodes[ip+1];
                if p1 < 0 {
                    panic!("Tried to input to negative memory.");
                }
                intcodes[p1 as usize] = input;
                ip += 2;
                input = -1; // consumed input
            },
            4 => {
                //output
                let p1 = get_value(&intcodes, intcodes[ip+1], pmodes[0]);
                output = p1;
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
                less_than(intcodes, ip, pmodes);
                ip += 4;
            },
            8 => {
                // equals
                equals(intcodes, ip, pmodes);
                ip += 4;
            },
            99 => {
                return (-1, output);
            },
            _ => {
                panic!(
                    "Error while reading intcodes. Unknown opcode {} at \
                    position: {}", intcodes[ip], ip);
            }
        }
    }
}

/// feedback loop where machine waits at every input request
fn part2() {
    let intcode_string = read_input().unwrap();
    let intcodes = intcode_string.split(",")
    .map(|opcode| opcode.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();

    let mut highest_signal = 0;
    for i_a in 5..10 {
        for i_b in 5..10 {
            if i_b == i_a {
                continue
            }
            for i_c in 5..10 {
                if i_c == i_a || i_c == i_b {
                    continue
                }
                for i_d in 5..10 {
                    if i_d == i_a || i_d == i_b || i_d == i_c {
                        continue
                    }
                    for i_e in 5..10 {
                        if i_e == i_a || i_e == i_b || i_e == i_c || i_e == i_d {
                            continue
                        }
                        // Checking new Phase Setting
                        // initialize intcode table for every
                        let mut intcodes_a = intcodes.clone();
                        let mut intcodes_b = intcodes.clone();
                        let mut intcodes_c = intcodes.clone();
                        let mut intcodes_d = intcodes.clone();
                        let mut intcodes_e = intcodes.clone();
                        let mut_slice_a = intcodes_a.as_mut_slice();
                        let mut_slice_b = intcodes_b.as_mut_slice();
                        let mut_slice_c = intcodes_c.as_mut_slice();
                        let mut_slice_d = intcodes_d.as_mut_slice();
                        let mut_slice_e = intcodes_e.as_mut_slice();

                        // init tuples for calc results (first is ip, second output)
                        let mut result_a: (i32, i32);
                        let mut result_b: (i32, i32);
                        let mut result_c: (i32, i32);
                        let mut result_d: (i32, i32);
                        let mut result_e: (i32, i32);
                        

                        result_a = part2_machine(mut_slice_a, i_a, 0); // initialize machine
                        result_a = part2_machine(mut_slice_a, 0, result_a.0 as usize); // add first output

                        result_b = part2_machine(mut_slice_b, i_b, 0);
                        result_b = part2_machine(mut_slice_b, result_a.1, result_b.0 as usize);

                        result_c = part2_machine(mut_slice_c, i_c, 0);
                        result_c = part2_machine(mut_slice_c, result_b.1, result_c.0 as usize);
                        
                        result_d = part2_machine(mut_slice_d, i_d, 0);
                        result_d = part2_machine(mut_slice_d, result_c.1, result_d.0 as usize);
                        
                        result_e = part2_machine(mut_slice_e, i_e, 0);
                        result_e = part2_machine(mut_slice_e, result_d.1, result_e.0 as usize);
                        
                        loop {
                            if result_e.0 != -1 {
                                result_a = part2_machine(mut_slice_a, result_e.1, result_a.0 as usize);
                                result_b = part2_machine(mut_slice_b, result_a.1, result_b.0 as usize);
                                result_c = part2_machine(mut_slice_c, result_b.1, result_c.0 as usize);
                                result_d = part2_machine(mut_slice_d, result_c.1, result_d.0 as usize);
                                result_e = part2_machine(mut_slice_e, result_d.1, result_e.0 as usize);
                            } else {
                                break
                            }
                        }
                        if result_e.1 > highest_signal {
                            highest_signal = result_e.1;
                        }
                    }
                }
            }
        }
    }
    println!("{}", highest_signal);
}


fn main() {
    // part1();
    part2();
}
