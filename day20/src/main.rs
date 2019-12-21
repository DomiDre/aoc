use std::fs::File;
use std::io::{BufRead, BufReader, Error};

mod donut_maze;
use donut_maze::{DonutMaze, Map};

mod recursive_donut_maze;
use recursive_donut_maze::RecursiveDonutMaze;

fn read_input(filepath: &str) -> Result<Map, Error> {
    let file = File::open(filepath)?;
    let buffered = BufReader::new(file);

    let mut map: Map = Vec::new();
    let mut idx_row = 0;
    for line in buffered.lines() {
        map.push(Vec::new());
        for c in line.unwrap().chars() {
            map[idx_row].push(c);
        }
        idx_row += 1;
    }
    Ok(map)
}

fn part1(maze_vec: Map) {
    println!("PART 1");
    let maze = DonutMaze::new(maze_vec);
    if let Some(path) = maze.find_shortest_connection(maze.idx_start, maze.idx_end) {
        println!(
            "Shortest connection from AA to ZZ takes {} steps",
            path.len() - 1
        );
    } else {
        println!("Couldnt find a connection");
    }
}

fn part2(maze_vec: Map) {
    println!("PART 2");
    let maze = RecursiveDonutMaze::new(maze_vec);
    if let Some(steps) = maze.find_shortest_connection(maze.idx_start, maze.idx_end) {
        println!(
            "Shortest connection from AA to ZZ takes {} steps",
            steps
        );
    } else {
        println!("Couldnt find a connection");
    }
}

fn main() {
    let maze_vec = read_input("./input").unwrap();
    part1(maze_vec.clone());
    part2(maze_vec);
}
