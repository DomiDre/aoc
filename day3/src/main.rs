use std::fs;
use std::io::Error;

#[derive(Debug)]
struct Line {
    start: (i32, i32),
    end: (i32, i32)
}

fn read_input() -> Result<String, Error> {
    let content = fs::read_to_string("./input")
    .expect("Error while reading file");

    Ok(content)
}

fn read_instruction(instr: &str) -> (&str, i32) {
    let direction = &instr[..1];
    let steps: i32 = (&instr[1..]).parse().unwrap();
    (direction, steps)
}

fn shift_position(start: (i32, i32), direction: &str, steps: i32) -> (i32, i32){
    let mut end = start;
    match direction {
        "R" => {
            end.0 += steps;
        },
        "L" => {
            end.0 -= steps;
        },
        "U" => {
            end.1 += steps;
        },
        "D" => {
            end.1 -= steps;
        },
        _ => {
            panic!("Instruction contained unknown direction: {}", direction);
        }
    }
    end
}

/// Read input file and first read the set of instructions
/// for the first line to get a vector of start & end points
fn part1() {
    //R
    let file_content = read_input().unwrap();

    let instructions: Vec<_>= file_content.split("\n").collect();
    let instructions1: Vec<_> = instructions[0].split(",").collect();
    let instructions2: Vec<_> = instructions[1].split(",").collect();

    // Read instructions1 and create vector containing all lines
    // defined by their start and endpoints as two tuples (x,y)
    let mut lines1: Vec<Line> = Vec::new();
    let mut position = (0, 0);
    for instr in instructions1 {
        let (direction, steps) = read_instruction(instr);

        let start = position;
        let end = shift_position(start, direction, steps);
        lines1.push(Line {
            start,
            end
        });
        position = end;
    }

    // Read instructions2, also create start and end point
    // but also check 
    for instr in instructions2 {
        let (direction, steps) = read_instruction(instr);
        let start = position;
        let end = shift_position(start, direction, steps);

    }
    println!("{:?}", lines1);
}

fn main() {
    part1();
}
