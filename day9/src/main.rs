use std::fs;
use std::io::{Error};

pub mod computer;
use computer::IntcodeComputer;

/// Extending day5 solution

/// Read file "input" and return content
fn read_input() -> Result<Vec<i64>, Error> {
    let content = fs::read_to_string("./input")
    .expect("Error while opening input file.");

    let intcodes = content.split(",")
    .map(|opcode| opcode.parse::<i64>().unwrap())
    .collect::<Vec<i64>>();

    Ok(intcodes)
}

fn part1(_intcodes: Vec<i64>) {
    let mut comp = IntcodeComputer::new(_intcodes);
    // comp.debug_mode = true;
    comp.run();
    comp.set_memory_input(1);
    comp.run();
}

fn part2(_intcodes: Vec<i64>) {
    let mut comp = IntcodeComputer::new(_intcodes);
    // comp.debug_mode = true;
    comp.run();
    comp.set_memory_input(2);
    comp.run();

}

fn main() {
    let intcodes = read_input().unwrap();
    part1(intcodes.clone());
    part2(intcodes.clone());
}
