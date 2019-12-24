use std::fs::File;
use std::io::{Error, BufRead, BufReader};
use std::collections::HashSet;
use std::collections::VecDeque;

const GRID_SIZE: usize = 5;
type Map = [[char; GRID_SIZE]; GRID_SIZE];
type RecursiveMap = VecDeque<Map>;

mod bug_map;
use bug_map::BugMap;

mod recursive_bug_map;
use recursive_bug_map::RecursiveBugMap;

fn read_input(file_path: &str) -> Result<Map, Error> {
    let file = File::open(file_path)?;
    let buffer = BufReader::new(file);
    let mut map = [['.'; GRID_SIZE]; GRID_SIZE];

    for (j, read_line) in buffer.lines().enumerate() {
        if let Ok(line) = read_line {
            for (i, c) in line.chars().enumerate() {
                map[j][i] = c;
            }
        }
    }
    Ok(map)
}

fn part1() {
    let mut map = BugMap::new(
        read_input("./input").unwrap()
    );
    let mut observed_ratings = HashSet::new();
    loop {
        let new_value = observed_ratings.insert(map.biodiversity_rating());
        if !new_value {
            break;
        }
        map.evolve();
    }
    println!("First repeated rating: {}", map.biodiversity_rating());
}

fn part2() {
    let mut recursive_map = RecursiveBugMap::new(
        read_input("./input").unwrap()
    );
    for _i in 0..200 {
        recursive_map.evolve();
    }
    println!("Number of Bugs: {}", recursive_map.number_of_bugs());
}

fn main() {
    part1();
    part2()
}
