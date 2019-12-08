use std::fs;
use std::io::Error;


/// Read file "input" and return content
fn read_input() -> Result<String, Error> {
    let content = fs::read_to_string("./input")
    .expect("Error while opening input file.");

    Ok(content)
}

/// Parse the input codes and determine what is at position 0
/// Once opcode 99 is reached
fn part1() {
    let intcode_string = read_input().unwrap();
    let mut intcodes = intcode_string.split(",")
    .map(|opcode| opcode.parse::<usize>().unwrap())
    .collect::<Vec<usize>>();
    println!("{}", intcodes[0]);

    intcodes[1] = 12;
    intcodes[2] = 2;
    let mut ip = 0; // instruction pointer
    loop {
        println!("Position: {}, Intcode: {}", ip, intcodes[ip]);
        let i = intcodes[ip+1];
        let j = intcodes[ip+2];
        let k = intcodes[ip+3];
        match intcodes[ip] {
            1 => {
                intcodes[k] = intcodes[i] + intcodes[j];
            },
            2 => {
                intcodes[k] = intcodes[i] * intcodes[j];
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
        ip += 4;
    }
    println!("Value at position 0: {}", intcodes[0]);
}

/// Modify noun (intcodes[1]) and verb (intcodes[2]) such that the output
/// (intcodes[0]) is equal to 19690720. The searched value is 100 * noun + verb
fn part2() {
    let intcode_string = read_input().unwrap();
    let initial_intcodes = intcode_string.split(",")
    .map(|opcode| opcode.parse::<usize>().unwrap())
    .collect::<Vec<usize>>();

    for noun in 0..99 {
        for verb in 0..99 {
            let mut intcodes = initial_intcodes.clone();
            intcodes[1] = noun;
            intcodes[2] = verb;
            let mut ip = 0; // instruction pointer
            loop {
                let i = intcodes[ip+1];
                let j = intcodes[ip+2];
                let k = intcodes[ip+3];
                match intcodes[ip] {
                    1 => {
                        intcodes[k] = intcodes[i] + intcodes[j];
                    },
                    2 => {
                        intcodes[k] = intcodes[i] * intcodes[j];
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
                ip += 4;
            }
            if intcodes[0] == 19690720 {
                println!("Found result: {}", 100*noun + verb);
                break;
            }
        }
    }
}

fn main() {
    // part1();
    part2();
}
