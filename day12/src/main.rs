use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[derive(Debug, Clone)]
struct Moon {
    position: [i64; 3],
    velocity: [i64; 3],
}

impl Moon {
    fn new(x: i64, y: i64, z: i64) -> Moon {
        Moon {
            position: [x, y, z],
            velocity: [0, 0, 0]
        }
    }

    fn potential_energy(&self) -> i64 {
        let mut energy = 0;
        for i in 0..3 {
            energy += self.position[i].abs()
        }
        energy
    }

    fn kinetic_energy(&self) -> i64 {
        let mut energy = 0;
        for i in 0..3 {
            energy += self.velocity[i].abs()
        }
        energy
    }
}


struct Simulation {
    moons: Vec<Moon>,
    time: u64
}

impl Simulation {
    fn new(moons: Vec<Moon>) -> Simulation {
        Simulation {
            moons,
            time: 0
        }
    }

    /// Pull every pair of moons closer together
    fn apply_gravity(&mut self, previous_state: &[Moon]) {
        for j in 1..self.moons.len() {
            // make a cut in vector between every pair of elements
            let (moons_before_j, moons_after_j) = self.moons.split_at_mut(j);
            // for one element always the last of the left slice
            // to (avoid double counting)
            let mut moon = moons_before_j.last_mut().unwrap();
            // for the other elements take all of the right side
            for (k, other_moon) in moons_after_j.iter_mut().enumerate() {
                // for every dimension
                for i in 0..3 {
                    // for comparison use moons in unedited state
                    if previous_state[j-1].position[i] < previous_state[j+k].position[i] {
                        moon.position[i] += 1;
                        other_moon.position[i] -= 1;
                    } else if previous_state[j-1].position[i] > previous_state[j+k].position[i] {
                        moon.position[i] -= 1;
                        other_moon.position[i] += 1;
                    }
                }
            }
        }
    }

    fn apply_velocity(&mut self) {
        for moon in self.moons.iter_mut() {
            for i in 0..3 {
                moon.position[i] += moon.velocity[i];
            }
        }
    }

    fn update_velocity(&mut self, previous_state: &[Moon]) {
        for (m, moon) in self.moons.iter_mut().enumerate() {
            for i in 0..3 {
                moon.velocity[i] = moon.position[i] - previous_state[m].position[i];
            }
        }
    }

    fn energy(&self) -> i64 {
        let mut total_energy = 0;
        for moon in self.moons.iter() {
            total_energy += moon.potential_energy() * moon.kinetic_energy();
        }
        total_energy
    }

    fn time_step(&mut self) {
        let previous_state = self.moons.clone();
        self.apply_gravity(&previous_state);
        self.apply_velocity();
        self.update_velocity(&previous_state);
        self.time += 1;
    }
}

// Read input file
fn read_input() -> Result<Vec<Moon>, Error> {
    let file = File::open("./input")?;
    let buffer = BufReader::new(file);

    let mut moons: Vec<Moon> = Vec::new();
    let re = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
    for line in buffer.lines() {
        let line_re = re.captures(Box::leak(line.unwrap().into_boxed_str()));
        if let Some(value) = line_re {
            // if motivated: at better error message in any case anything fails
            let x = value.get(1).map(|m| m.as_str()).unwrap().parse::<i64>().unwrap();
            let y = value.get(2).map(|m| m.as_str()).unwrap().parse::<i64>().unwrap();
            let z = value.get(3).map(|m| m.as_str()).unwrap().parse::<i64>().unwrap();
            moons.push( Moon::new(x,y,z) );
        }
    }

    Ok(moons)
}

fn euclid(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        euclid(b, a%b)
    }
}

fn part1() {
    let initial_moon_positions = read_input().unwrap();
    let mut sim = Simulation::new(initial_moon_positions);
    for _i in 0..1000 {
        sim.time_step();
    }
    println!("Part 1:\nEnergy after 1000 steps: {}\n", sim.energy());
    
}

fn part2() {
    let initial_state = read_input().unwrap();
    let mut sim = Simulation::new(initial_state.clone());
    
    let mut k: u64 = 0;
    let mut cycles = [0; 3];
    loop {
        // evolve system
        sim.time_step();    
        k += 1;
        // check for cycles
        for i in 0..3 {
            if cycles[i] > 0 {
                continue
            }
            let mut is_cycle = true;
            // check every moon as if it has the same position in dimension i, as initially
            for (m, moon) in sim.moons.iter().enumerate() {
                if moon.position[i] != initial_state[m].position[i] || moon.velocity[i] != initial_state[m].velocity[i] {
                    is_cycle = false
                }
            }
            if is_cycle {
                cycles[i] = k;
            }
        }
        if cycles[0] > 0 && cycles[1] > 0 && cycles[2] > 0 {
            break
        }
        if k % 1000000 == 0 {
        }
    }

    // Found cycles for seperate coordinates -> find now least common multiple
    let cycle_product = cycles[0]*cycles[1]*cycles[2];
    // divide product by greatest common divisor to get least common multiple
    println!("Part2:\n Cycle: {}", cycle_product/euclid(cycles[0], cycles[1])/euclid(cycles[0], cycles[2]));
}

fn main() {
    part1();
    part2();
}
