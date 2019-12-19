use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

pub const MAP_DIM_X: usize = 81;
pub const MAP_DIM_Y: usize = 81;
pub type Map = [[char; MAP_DIM_Y]; MAP_DIM_X];

/// Keep track of state in the maze
/// For final output also keep track of the order in that the keys are collected
#[derive(Clone, Debug)]
struct State {
    pub position: Vec<usize>,
    pub distance: usize,
    pub collected_keys: HashSet<char>,
    pub keys_in_order: Vec<char>,
}

impl State {
    pub fn new(
        position: Vec<usize>,
        distance: usize,
        collected_keys: HashSet<char>,
        keys_in_order: Vec<char>,
    ) -> State {
        State {
            position,
            distance,
            collected_keys,
            keys_in_order,
        }
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.collected_keys == other.collected_keys
    }
}

impl Eq for State {}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // states are considered equal if robot is on same position & owns the same
        // keys
        self.position.hash(state);
        // don't care for order of collecting keys
        // to get same hash every time hash them
        // in alphabetic order
        let mut sorted_keys = self.keys_in_order.clone();
        sorted_keys.sort();
        for key in sorted_keys.iter() {
            key.hash(state);
        }
    }
}

pub struct Maze {
    pub grid: Map,
    pub adjacency_list: HashMap<usize, Vec<usize>>,
    pub point_of_interests: HashMap<usize, char>,
    pub start_idx: Vec<usize>,
    pub keys: HashMap<char, usize>,
    pub doors: HashMap<char, usize>,
}

impl Maze {
    pub fn new(grid: Map) -> Maze {
        // create adjacency list
        let mut adjacency_list: HashMap<usize, Vec<usize>> = HashMap::new();
        // and while at it, note down all important positions on the map
        let mut keys = HashMap::new();
        let mut doors = HashMap::new();
        let mut point_of_interests = HashMap::new();
        let mut start_idx = Vec::new();
        for j in 0..MAP_DIM_Y {
            for i in 0..MAP_DIM_X {
                let c = grid[i][j];
                let idx = grid_to_idx(i, j);
                // check if position is even of interest
                // # wall points are uninteresting -> continue loop directly
                // . floor points are interesting for adjacency list
                // everything else is an important point: key, door or starting point
                match c {
                    '#' => continue,
                    '.' => (),
                    _ => {
                        if c.is_lowercase() {
                            keys.insert(c, idx);
                            point_of_interests.insert(idx, c);
                        } else if c.is_uppercase() {
                            doors.insert(c, idx);
                            point_of_interests.insert(idx, c);
                        } else if c == '@' {
                            if start_idx.len() > 9 {
                                panic!("Maze only supports 10 start positions (for now).");
                            }
                            // replace @ symbol by chars '0'..'9'
                            point_of_interests.insert(idx, digit_to_char(start_idx.len()));
                            start_idx.push(idx);
                        }
                    }
                };

                let mut adjacenct_walkable_points = Vec::new();
                let neighbours = [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)];
                for neighbour in neighbours.iter() {
                    // check whether points are out of grid not necessary
                    // as maze is surrounded by '#', which are ignored

                    // check if neighbour is of wall type or walkable
                    // doors are included as they are technically walkable after key
                    if grid[neighbour.0][neighbour.1] != '#' {
                        adjacenct_walkable_points.push(grid_to_idx(neighbour.0, neighbour.1));
                    }
                }
                adjacency_list.insert(idx, adjacenct_walkable_points);
            }
        }

        assert_ne!(start_idx.len(), 0, "No start position defined in maze");

        Maze {
            grid,
            adjacency_list,
            point_of_interests,
            start_idx,
            keys,
            doors,
        }
    }

    /// For two points start and end, this function determines the shortest connection
    /// as vector if floor indexes if there is no possible connection, it returns None
    pub fn find_shortest_connection(&self, start: usize, end: usize) -> Option<Vec<usize>> {
        let mut queue = VecDeque::new();
        queue.push_back(start);

        // keep track which nodes have already been visited
        let mut visited: HashSet<usize> = HashSet::new();
        // store for every visited node how one got here starting from the start point
        let mut parent: HashMap<usize, usize> = HashMap::new();

        // append
        loop {
            let node_opt = queue.pop_front();
            if node_opt.is_none() {
                // the queue is empty, there is no way to reach end
                return None;
            }

            let node = node_opt.unwrap();
            visited.insert(node);
            if node == end {
                // found the searched node
                break;
            }

            // by definition neighbours could not be empty
            // or the adjacency list is uncomplete

            let neighbours = self.adjacency_list.get(&node).unwrap();

            for neighbour in neighbours.iter() {
                if visited.get(neighbour).is_some() {
                    // if neighbour has already been visited -> ignore
                    continue;
                }

                // push neighbour into the queue of paths to check
                queue.push_back(*neighbour);

                // note that neighbour was visited coming from node
                parent.insert(*neighbour, node);
            }
        }
        // parent is filled with a path. now just construct it
        let mut path = Vec::new();
        let mut node = end;
        path.push(node);
        loop {
            let node_opt = parent.get(&node);
            if node_opt.is_none() {
                return None;
            }
            node = *node_opt.unwrap();
            path.push(node);
            if node == start {
                break;
            }
        }
        Some(path)
    }

    // /// get all positions that are reachable and how many steps to get there
    // fn get_reachable_keys(&self, start_idx: usize, collected_keys: &HashSet<char>) -> Vec<(char, usize)> {
    //     let mut reachable_keys = Vec::new();
    //     'for_keys: for (i, (key, key_pos)) in self.keys.iter().enumerate() {
    //         let path_opt = self
    //         .find_shortest_connection(self.start_idx[0], *key_pos);
    //         // key is unreachable if path is none
    //         if path_opt.is_none() {
    //             continue
    //         }
    //         let path = path_opt.unwrap();
    //         let steps = path.len() - 1;

    //         // check for doors on path
    //         for i in 1..path.len() {
    //             // is path a point of interest?
    //             let opt_poi = self.point_of_interests.get(&path[i]);
    //             if opt_poi.is_some() {
    //                 let poi = opt_poi.unwrap();
    //                 // is poi a door?
    //                 if poi.is_ascii_uppercase() {
    //                     // if key for door is not collected -> this key is not reachable
    //                     if collected_keys.get(&poi.to_ascii_lowercase()).is_none() {
    //                         continue 'for_keys;
    //                     }
    //                 }
    //                 // possibly add here if anything should be done if another poi is on path
    //             }
    //         }
    //         reachable_keys.push((*key, steps));
    //     }
    //     // return trsvhsblr keys in order of shortest distance
    //     reachable_keys.sort_by_key(|k| k.1);
    //     reachable_keys
    // }

    /// for all keys determine distance to every other key (that has a reachable path)
    /// and which doors are along the path
    pub fn get_distances_and_blocking_doors(
        &self,
    ) -> (
        HashMap<char, Vec<(char, usize)>>,
        HashMap<(char, char), Vec<char>>,
    ) {
        let mut distances: HashMap<char, Vec<(char, usize)>> = HashMap::new();
        let mut blocking_doors: HashMap<(char, char), Vec<char>> = HashMap::new();
        
        // check every robot
        for idx_robot in 0..self.start_idx.len() {
            for (i, (key, key_pos)) in self.keys.iter().enumerate() {
                // check distance from start
                let path_opt = self
                    .find_shortest_connection(self.start_idx[idx_robot], *key_pos);
                // ignore generally unreachable keys
                if path_opt.is_none() {
                    continue
                }
                let path = path_opt.unwrap();
                let steps = path.len() - 1;
                distances
                    .entry(digit_to_char(idx_robot))
                    .or_insert(Vec::new())
                    .push((*key, steps));
                // check if on path, blocking doors are present
                let mut door_on_path = Vec::new();
                for i in 1..path.len() {
                    // is path a point of interest?
                    let opt_poi = self.point_of_interests.get(&path[i]);
                    if opt_poi.is_some() {
                        let poi = opt_poi.unwrap();
                        if self.doors.get(poi).is_some() {
                            door_on_path.push(*poi);
                        }
                    }
                }
                if door_on_path.len() > 0 {
                    blocking_doors.insert((digit_to_char(idx_robot), *key), door_on_path);
                }
    
                // check distance between keys
                for (other_key, other_key_pos) in self.keys.iter().skip(i + 1) {
                    let path_opt = self
                        .find_shortest_connection(*key_pos, *other_key_pos);
                    if path_opt.is_none() {
                        continue
                    }
                    let path = path_opt.unwrap();
                    let steps = path.len() - 1;
                    distances
                        .entry(*key)
                        .or_insert(Vec::new())
                        .push((*other_key, steps));
                    distances
                        .entry(*other_key)
                        .or_insert(Vec::new())
                        .push((*key, steps));
    
                    // check if on path, blocking doors are present
                    let mut door_on_path = Vec::new();
                    for i in 1..path.len() {
                        // is path a point of interest?
                        let opt_poi = self.point_of_interests.get(&path[i]);
                        if opt_poi.is_some() {
                            let poi = opt_poi.unwrap();
                            if self.doors.get(poi).is_some() {
                                door_on_path.push(*poi);
                            }
                        }
                    }
                    if door_on_path.len() > 0 {
                        blocking_doors.insert((*key, *other_key), door_on_path.clone());
                        blocking_doors.insert((*other_key, *key), door_on_path);
                    }
                }
            }
        }
        for distance_vec in distances.values_mut() {
            distance_vec.sort_by_key(|k| k.1);
        }
        (distances, blocking_doors)
    }

    pub fn collect_all_keys(&self) {
        let (distances, blocking_doors) = self.get_distances_and_blocking_doors();
        // for (start_char, distances) in distances.iter() {
        //     println!("{} -> {:?}", start_char, distances);
        // }
        let num_all_keys = self.keys.len(); // number of keys that need to be collected
        let num_robots = self.start_idx.len(); // number of robots that can move
        
        let mut queue: Vec<State> = Vec::new(); // sorted queue of states 
        let initial_state = State::new(self.start_idx.clone(), 0, HashSet::new(), Vec::new());
        queue.push(initial_state);

        let mut known_states: HashSet<State> = HashSet::new(); // keep track of visited states
        let mut counter = 0; // state counter for debugging
        loop {
            let state = queue.remove(0);

            // check if state is already known
            if known_states.get(&state).is_some() {
                continue;
            }
            if state.collected_keys.len() == num_all_keys {
                println!("Found a shortest path that collects all keys:");
                println!("{:?}", state);
                break;
            }

            // test to move every robot
            for robot_idx in 0..num_robots {
                // println!("{}", state.position[robot_idx]);
                // test every possible move to other keys from current position
                let state_key_opt = self.point_of_interests.get(&state.position[robot_idx]);

                let state_key = state_key_opt.unwrap();
                //.unwrap();
                'node_neighbour_loop: for (other_key, distance) in
                    distances.get(state_key).unwrap().iter()
                {
                    // check if key has already been collected
                    if state.collected_keys.get(other_key).is_some() {
                        continue;
                    }
                    // first check if this key is even reachable
                    let blockage_opt = blocking_doors.get(&(*state_key, *other_key));
                    if blockage_opt.is_some() {
                        let blocked = blockage_opt.unwrap();
                        for blocked_door in blocked.iter() {
                            // check if key for blocked_door was collected
                            if state
                                .collected_keys
                                .get(&blocked_door.to_ascii_lowercase())
                                .is_none()
                            {
                                continue 'node_neighbour_loop;
                            }
                        }
                    }
                    // it's valid to collect the key, add new state
                    let mut new_state = state.clone();
                    new_state.position[robot_idx] = *self.keys.get(other_key).unwrap();
                    new_state.distance += distance;
                    new_state.collected_keys.insert(*other_key);
                    new_state.keys_in_order.push(*other_key);
                    queue.push(new_state)
                }
            }
            queue.sort_by_key(|k| k.distance);
            // bottle neck is sort if queue becomes too long
            // if queue is > 50_000, drop the last states
            // ... this is not generally correct and may result in wrong results
            // gotta optimize the code otherwise...
            if counter % 10000 == 0  && queue.len() > 50000 {
                queue.drain(50000..);
            }
            counter += 1;
            if counter % 10000 == 0 {
                println!(
                    "Iteration: {}, Shortest Distance Travelled: {}, Number of candidate states: {} ... Still Calculating...",
                    counter,
                    state.distance,
                    queue.len()
                );
            }
            known_states.insert(state);
        }
    }
}

fn grid_to_idx(x: usize, y: usize) -> usize {
    x + y * MAP_DIM_X
}

fn digit_to_char(digit: usize) -> char {
    ((digit as u8) + b'0') as char
}