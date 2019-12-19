use std::fs;
use std::io::Error;

mod computer;
mod drone_system;
use drone_system::{DroneState, DroneSystem};

const SCAN_RANGE: usize = 50;
fn read_input() -> Result<Vec<i64>, Error> {
    let content = fs::read_to_string("./input")?;
    let digits: Vec<i64> = content
        .split(',')
        .map(|d| d.parse::<i64>().unwrap())
        .collect();

    Ok(digits)
}

fn print_map(map: &[[char; SCAN_RANGE]; SCAN_RANGE]) {
    for j in 0..SCAN_RANGE {
        for i in 0..SCAN_RANGE {
            print!("{}", map[i][j]);
        }
        println!();
    }
}

fn part1(intcodes: Vec<i64>) {
    let mut drone_system = DroneSystem::new(intcodes);
    let mut tractor_beam_counter = 0;
    let mut map: [[char; SCAN_RANGE]; SCAN_RANGE] = [['.'; SCAN_RANGE]; SCAN_RANGE];
    for i in 0..SCAN_RANGE {
        for j in 0..SCAN_RANGE {
            let state = drone_system.deploy_drone(i, j);
            if let DroneState::Pulled = state {
                map[i][j] = '#';
                tractor_beam_counter += 1;
            }
        }
    }
    print_map(&map);
    println!("Points affected in scanned area: {}", tractor_beam_counter);
}

fn part2(intcodes: Vec<i64>) {
    // looking at map -> tractor beam left and right edge only move rightward
    // move along right edge of tractor beam and always check if point x-100,y and point x-100,y+100
    // are in the beam, at first occurence -> ship fits

    let mut drone_system = DroneSystem::new(intcodes);
    let mut y = 40; // start a lower y as there are some edge case lines at beginning without
                    // any tractor beam point
                    // find right edge of tractor beam
    let mut x_right = 0;
    // go right until x is in beam
    loop {
        let state = drone_system.deploy_drone(x_right, y);
        if let DroneState::Pulled = state {
            break;
        }
        x_right += 1;
    }

    // go further until beam leaves again
    loop {
        let state = drone_system.deploy_drone(x_right, y);
        if let DroneState::Stationary = state {
            x_right -= 1;
            break;
        }
        x_right += 1;
    }
    // go down the beam, determine width of it
    loop {
        y += 1;
        // find new right edge position
        loop {
            if let DroneState::Pulled = drone_system.deploy_drone(x_right, y) {
                x_right += 1;
            } else {
                x_right -= 1;
                break;
            }
        }
        // check if upper left point of ship is in tractor
        if x_right < 100 {
            // ignore out of bounds conditions
            continue;
        }
        if let DroneState::Stationary = drone_system.deploy_drone(x_right - 99, y) {
            continue;
        }
        // and check if lower left point of ship in tractor
        if let DroneState::Stationary = drone_system.deploy_drone(x_right - 99, y + 99) {
            continue;
        }

        // if this point is reached, ship fits
        break;
    }
    println!("{} {}", x_right, y);
    println!("Answer: {}", (x_right - 99) * 10000 + y);
}
fn main() {
    let intcodes = read_input().unwrap();
    part1(intcodes.clone());
    part2(intcodes);
}
