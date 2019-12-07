use std::fs::File;
use std::io::{Error, BufReader, BufRead};

///Calculates needed fuel depending on given mass
/// By dividing by three, rounding down and subtracting 2
fn calc_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

//Read input file line by line, calculate mass for each line
//and get sum
fn part1() -> Result<(), Error> {
    let file = File::open("input")?;
    let buffered = BufReader::new(file);
    
    let mut total_fuel = 0;
    for line in buffered.lines() {
        let mass: i32 = line?.parse().unwrap();
        let fuel = calc_fuel(mass);
        total_fuel += fuel;
    }
    println!("{}", total_fuel);

    Ok(())
}

//Read input file again line by line but this time for each
//line do a loop for additional fuel calculation
fn part2() -> Result<(), Error> {
    let file = File::open("input")?;
    let buffered = BufReader::new(file);
    
    let mut total_fuel = 0;
    for line in buffered.lines() {
        let mass: i32 = line?.parse().unwrap();
        let mut fuel = calc_fuel(mass);
        total_fuel += fuel;
        loop {
            fuel = calc_fuel(fuel);
            if fuel > 0 {
                total_fuel += fuel;
            } else {
                break;
            }
        }
    }
    println!("{}", total_fuel);

    Ok(())
}

fn main() {
    // part1().expect("Error while reading input");
    part2().expect("Error while reading input");
}