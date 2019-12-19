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
}

fn main() {
    part1();
    // part2();
    // // determine all paths to finish the maze
    // // init with no keys collected
    // let collected_keys = Vec::new();
    // // and all doors closed
    // let closed_doors = maze.doors.clone();
    // let paths = maze.get_paths_length_dfs(maze.start_idx, collected_keys, closed_doors, 0);
    // println!("There are {} possible paths", paths.len());
    // let mut shortest_path = &paths[0].0;
    // let mut shortest_length = paths[0].1;
    // for path in paths.iter() {
    //     if path.1 < shortest_length {
    //         shortest_path = &path.0;
    //         shortest_length = path.1;
    //     }
    // }
    // println!("The shortest path is {:?} with length {}", shortest_path, shortest_length);

    // maze.get_paths_length_dijkstra();

    // println!("{:?}", maze.paths_collect_all_keys());
    // let path = maze.find_shortest_connection(
    //     maze.start_idx, *maze.keys.get(&'a').unwrap()
    // );
}
