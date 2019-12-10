use std::fs::{File};
use std::io::{Error, BufRead, BufReader};

type PlanarVector = (f64, f64);

const EPSILON: f64 = 0.00001;

#[derive(Debug)]
struct Map {
    asteroids: Vec<Asteroid>
}

impl Map {
    fn new() -> Map {
        Map {
            asteroids: Vec::new()
        }
    }

    fn add_asteroid(&mut self, x: i64, y: i64) {
        self.asteroids.push(Asteroid {
            x,
            y
        });
    }
}

#[derive(Debug)]
struct Asteroid {
    x: i64,
    y: i64
}

impl Asteroid {
    pub fn count_other_visible_asteroids(&self, map: &Map) -> usize {
        let mut unique_directions: Vec<PlanarVector> = Vec::new();

        'loop_others: for other_asteroid in map.asteroids.iter() {
            // generate line connecting asteroid with other asteroid
            let direction = self.get_direction(&other_asteroid);
            
            // ignore comparison with itself
            if direction.0.abs() < EPSILON && direction.1.abs() < EPSILON {
                continue 'loop_others;
            }

            // check if direction is already in list
            for other_direction in unique_directions.iter() {
                let s: f64 = dot_product(&direction, other_direction) / 
                        dot_product(&direction, &direction);
                if s > 0.0 && 
                   (s*direction.0 - other_direction.0).abs() < EPSILON &&
                   (s*direction.1 - other_direction.1).abs() < EPSILON {
                    // parallel vectors
                    continue 'loop_others
                }
            }
            unique_directions.push(direction);
        }
        unique_directions.len()
    }

    fn get_direction(&self, other_asteroid: &Asteroid) -> PlanarVector {
        (other_asteroid.x as f64 - self.x as f64,
         other_asteroid.y as f64 - self.y as f64)
    }
}

fn dot_product(a: &PlanarVector, b: &PlanarVector) -> f64 {
    return a.0*b.0 + a.1*b.1
}

fn cross_product(a: &PlanarVector, b: &PlanarVector) -> f64 {
    return a.0*b.1 - a.1*b.0
}

fn read_input() -> Result<Map, Error> {
    let file = File::open("./input")?;
    let buffered = BufReader::new(file);

    let mut map: Map = Map::new();
    let mut idx_row = 0;
    for line in buffered.lines() {
        let mut idx_col = 0;
        for c in line.unwrap().chars() {
            match c {
                '#' => map.add_asteroid(idx_col, idx_row),
                _ => {}
            };
            idx_col += 1;
        }
        idx_row += 1;
    }

    Ok(map)
}

/// for every asteroid, find the smallest number of unique directions pointing
/// to other asteroids, find the one with the highest count
fn part1(map: &Map) {
    let mut highest_count = 0;
    let mut highest_counting_asteroid: &Asteroid = &map.asteroids[0];

    for asteroid in map.asteroids.iter() {
        let count = asteroid.count_other_visible_asteroids(&map);
        if count > highest_count {
            highest_count = count;
            highest_counting_asteroid = asteroid;
        }
    }
    println!("Highest Count: {} for {:?}", highest_count, highest_counting_asteroid);
}


/// having a station positioned, loop over other asteroids, find the one with
/// the smallest negative angle (laser rotating clockwise) to current laser
/// if multiple solutions are found, take the one with smallest distance
/// then store that laser direction
fn part2(mut map: Map, station_asteroid: Asteroid) {
    println!("Station at x: {} y: {}", station_asteroid.x, station_asteroid.y);
    println!("Seeing {} asteroids", map.asteroids.len() - 1);

    // remove station from map
    for (i, asteroid) in map.asteroids.iter().enumerate() {
        if (asteroid.x == station_asteroid.x) && (asteroid.y == station_asteroid.y) {
            map.asteroids.remove(i);
            println!("Removed space station from laser map");
            break;
        }
    }
    // first destruction should be at 0, 1
    let mut laser_direction: PlanarVector = (0.0-10.0*EPSILON, -1.0);
    let mut removed_count = 0;
    loop {
        // loop over map over and over, determining and removing next asteroid

        // find idx with smallest angle & distance in clockwise direction
        let mut idx_to_remove = 0;
        // initialize with large values
        let mut found_cross_z = 0.0;
        let mut found_angle_change = 3.1415;
        let mut smallest_distance: f64 = 10000.0;
        let norm_laser = dot_product(&laser_direction, &laser_direction).sqrt();
        for (i, other_asteroid) in map.asteroids.iter().enumerate() {
            // get vector pointing from station to checked asteroid
            let direction = station_asteroid.get_direction(&other_asteroid);
            // calculate distance
            let distance = dot_product(&direction, &direction).sqrt();

            // calculate angle between station and asteroid
            let scalar = dot_product(&laser_direction, &direction);
            let cosine_angle = scalar / norm_laser / distance;
            let angle = cosine_angle.acos();
            // angle gives magnitude but not direction (is asteroid left or
            // right of the laser direction)
          
            // quick check: did laser even rotate? if no -> ignore this asteroid
            // would be errorous if in the end all asteroids would be all in one
            // line... not in this case
            if angle.abs() < EPSILON {
                continue
            }

            // sign of cross product tells us whether direction is in the 
            // right (positive) or left (negative) half plane relative to the
            // laser direction
            let cross_z =  cross_product(&laser_direction, &direction);

            // differentiate the cases where cross_z is positive or negative
            if cross_z > 0.0 {
                // if previous cross_z was negative, prefer this one and continue
                if found_cross_z < 0.0 {
                    idx_to_remove = i;
                    found_angle_change = angle;
                    smallest_distance = distance;
                    found_cross_z = cross_z;
                    continue;
                } else {
                    // both were positive -> compare angle values
                    // prefer smaller angle when both cross_z are positive
                    if angle < found_angle_change {
                        idx_to_remove = i;
                        found_angle_change = angle;
                        smallest_distance = distance;
                        found_cross_z = cross_z;
                        continue;
                    } else if (angle - found_angle_change).abs() < EPSILON {
                        // for same angle values -> compare distance
                        if distance < smallest_distance {
                            idx_to_remove = i;
                            found_angle_change = angle;
                            smallest_distance = distance;
                            found_cross_z = cross_z;
                            continue;
                        }
                    } else {
                        // otherwise dont store
                        continue
                    }
                }
            } else {
                // cross_z is negative (on left plane...)
                if found_cross_z > 0.0 {
                    // stored cross_z is on right plane -> ignore
                    continue
                } else {
                    // both are left plane, compare angles, this time
                    // the larger angle is better
                    if angle > found_angle_change {
                        idx_to_remove = i;
                        found_angle_change = angle;
                        smallest_distance = distance;
                        found_cross_z = cross_z;
                        continue;
                    } else if (angle - found_angle_change).abs() < EPSILON {
                        // again for same angle values -> compare distance
                        if distance < smallest_distance {
                            idx_to_remove = i;
                            found_angle_change = angle;
                            smallest_distance = distance;
                            found_cross_z = cross_z;
                            continue;
                        }
                    } else {
                        // otherwise dont store
                        continue
                    }
                }

            }
        }
        let destroyed = map.asteroids.remove(idx_to_remove);
        laser_direction = station_asteroid.get_direction(&destroyed);
        removed_count += 1;
        if removed_count == 200 {
            println!("Destroyed #{}: {:?}", removed_count, destroyed);
            println!("Solution: {}", destroyed.x*100 + destroyed.y);
            break
        }
    }
}

fn main() {
    let map = read_input().unwrap();
    part1(&map);
    part2(map, Asteroid {x:26, y:29});
}
