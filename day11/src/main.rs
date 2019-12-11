use std::fs;
use std::io::Error;
mod computer;
use computer::IntcodeComputer;

mod robot;
use robot::PaintingRobot;

fn read_input() -> Result<Vec<i64>, Error> {
    let content = fs::read_to_string("./input")?;

    let digits: Vec<i64> = content
        .split(',')
        .map(|operation| operation.parse().unwrap())
        .collect();
    Ok(digits)
}

fn part1(intcodes: Vec<i64>) {
    let mut comp = IntcodeComputer::new(intcodes);
    let mut robot = PaintingRobot::new();
    comp.show_stdinout = false;
    loop {
        comp.run();
        // check camera
        comp.set_memory_input(robot.get_color() as i64);
        comp.run();
        // brain outputs color to be painted
        robot.color(comp.memory_output);
        comp.run();
        // brain outputs tells which direction to turn
        robot.turn_robot(comp.memory_output);
        robot.move_robot();
        if comp.terminated {
            break;
        }
    }
    println!("{:?}", robot.get_individually_painted_fields());
}

fn part2(intcodes: Vec<i64>) {
    let mut comp = IntcodeComputer::new(intcodes);
    let mut robot = PaintingRobot::new();
    comp.show_stdinout = false;
    robot.color(1);
    loop {
        comp.run();
        // check camera
        comp.set_memory_input(robot.get_color() as i64);
        comp.run();
        // brain outputs color to be painted
        robot.color(comp.memory_output);
        comp.run();
        // brain outputs tells which direction to turn
        robot.turn_robot(comp.memory_output);
        robot.move_robot();
        if comp.terminated {
            break;
        }
    }
    robot.print_painted_fields();
}

fn main() {
    let intcodes = read_input().unwrap();
    part1(intcodes.clone());
    part2(intcodes);
}
