use std::fs::File;
use std::io::{BufRead, BufReader, Error};

mod maze;
use maze::{Map, Maze, MAP_DIM_X, MAP_DIM_Y};

fn read_input(filepath: &str) -> Result<Map, Error> {
    let file = File::open(filepath)?;
    let buffered = BufReader::new(file);

    let mut map: Map = [['?'; MAP_DIM_Y]; MAP_DIM_X];
    let mut idx_row = 0;
    for line in buffered.lines() {
        let mut idx_col = 0;
        for c in line.unwrap().chars() {
            map[idx_col][idx_row] = c;
            idx_col += 1;
        }
        idx_row += 1;
    }
    Ok(map)
}

fn part1() {
    let maze = Maze::new(read_input("./input").unwrap());
    maze.collect_all_keys();
}

fn part2() {
    let maze = Maze::new(read_input("./updated_input").unwrap());
    maze.collect_all_keys();
}

fn main() {
    part1();
    part2();
}
