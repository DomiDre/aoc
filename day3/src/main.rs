use std::fs;
use std::io::Error;

#[derive(Debug)]
struct Line {
    start: (i32, i32),
    end: (i32, i32),
    direction: (i32, i32)
}

impl Line {
    fn new(start: (i32, i32), end: (i32, i32)) -> Line {
        Line {
            start,
            end,
            direction: (end.0 - start.0, end.1 - start.1)
        }
    }
}

/// Read input file to String
fn read_input() -> Result<String, Error> {
    let content = fs::read_to_string("./input")
    .expect("Error while reading file");

    Ok(content)
}

/// translate vector of instructions to a vector of lines
/// containing start, end and direction of every line
fn instructions_to_lines(instructions: Vec<&str>) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();
    let mut position = (0, 0);
    for instr in instructions {
        let direction = &instr[..1];
        let steps: i32 = (&instr[1..]).parse().unwrap();

        let start = position;
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
        lines.push(Line::new(start, end));
        position = end;
    }
    lines
}

/// Dot production between direction vector of two lines
fn dot_product(line1: &Line, line2: &Line) -> i32 {
    line1.direction.0*line2.direction.0 + line1.direction.1*line2.direction.1
}

/// Check two lines if they cross and return crossing point in this case
fn check_cross(line1: &Line, line2: &Line) -> Option<(i32, i32)> {
    match dot_product(&line1, &line2) {
        0 => {
            // vectors are perpendicular
            let line_horizontal: &Line;
            let line_vertical: &Line;
            if line1.direction.0 == 0 {
                line_vertical = line1;
                line_horizontal = line2;
            } else {
                line_vertical = line2;
                line_horizontal = line1;
            }
            
            // x_hori = x_start^h + t * Dx^h
            // y_hori = y_start^h
            // x_verti = x_start^v
            // y_verti = y_start^v + s * Dy^h
            // check if x_start^v in x_hori and solve for t.
            // If 0<=t<=1 the crossing is in between start and end for x_hori
            // do same for y_start^h in y_verti, solving for s
            let cross_param_hor: f32 = (line_vertical.start.0 as f32 - line_horizontal.start.0 as f32)/
                                        (line_horizontal.direction.0 as f32);
            if cross_param_hor >= 0.0 && cross_param_hor <= 1.0 {
                let cross_param_ver = (line_horizontal.start.1 as f32 - line_vertical.start.1 as f32)/
                                        (line_vertical.direction.1 as f32);
                if cross_param_ver >= 0.0 && cross_param_ver <= 1.0 {
                    // lines cross, cross point is x0^v, y0^h
                    Some((line_vertical.start.0, line_horizontal.start.1))
                } else {
                    None
                }
            } else {
                None
            }
        },
        _ => {
            // parallel vectors... ignore case of overlap
            None
        }
    }
    
}

/// Read input file and first read the set of instructions
/// for the first line to get a vector of start & end points
fn part1() {
    //R
    let file_content = read_input().unwrap();

    let instructions: Vec<_>= file_content.split("\n").collect();
    let instructions1: Vec<_> = instructions[0].split(",").collect();
    let instructions2: Vec<_> = instructions[1].split(",").collect();

    // Read instructions and create vector containing all lines
    // including start & end point as well as direction
    let lines1 = instructions_to_lines(instructions1);
    let lines2 = instructions_to_lines(instructions2);
    
    // check each line with each line if they cross and store
    // crossing point distance from center
    let mut crossings: Vec<i32> = Vec::new();
    for line2 in lines2.iter() {
        for line1 in lines1.iter() {
            let cross = check_cross(&line1, &line2);
            if cross.is_some() {
                let point = cross.unwrap();
                let manhattan_distance = point.0.abs() + point.1.abs();
                crossings.push(manhattan_distance);
            }
        }
    }
    println!("{:?}", crossings.iter().min());
}

/// Instead of manhattan distance, now the number of steps is of interest
fn part2() {
    //R
    let file_content = read_input().unwrap();

    let instructions: Vec<_>= file_content.split("\n").collect();
    let instructions1: Vec<_> = instructions[0].split(",").collect();
    let instructions2: Vec<_> = instructions[1].split(",").collect();

    // Read instructions and create vector containing all lines
    // including start & end point as well as direction
    let lines1 = instructions_to_lines(instructions1);
    let lines2 = instructions_to_lines(instructions2);
    
    // determine again the crossing points, but this time extract the total
    // number of steps
    // crossing point distance from center
    let mut number_of_steps: Vec<i32> = Vec::new();
    let mut steps_wire_1: i32 = 0;
    let mut steps_wire_2: i32 = 0;
    for line2 in lines2.iter() {
        steps_wire_1 = 0; // reset wire1 before each for loop
        for line1 in lines1.iter() {
            let cross = check_cross(&line1, &line2);
            if cross.is_some() {
                let point = cross.unwrap();
                number_of_steps.push(steps_wire_1 + steps_wire_2 +
                    (point.0 - line1.start.0).abs() + (point.1 - line1.start.1).abs() +
                    (point.0 - line2.start.0).abs() + (point.1 - line2.start.1).abs()
                );
            }
            // add direction steps before checking next line
            steps_wire_1 += line1.direction.0.abs() + line1.direction.1.abs();
        }

        // add direction steps before checking next line
        steps_wire_2 += line2.direction.0.abs() + line2.direction.1.abs();
    }
    println!("{:?}", number_of_steps.iter().min());
}

fn main() {
    // part1();
    part2();
}
