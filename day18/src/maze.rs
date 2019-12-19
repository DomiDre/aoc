use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

pub const MAP_DIM_X: usize = 81;
pub const MAP_DIM_Y: usize = 81;
pub type Map = [[char; MAP_DIM_Y]; MAP_DIM_X];

#[derive(Clone, Debug)]
struct State {
    pub key: char,
    pub distance: usize,
    pub collected_keys: HashSet<char>,
    pub keys_in_order: Vec<char>,
}

impl State {
    pub fn new(
        key: char,
        distance: usize,
        collected_keys: HashSet<char>,
        keys_in_order: Vec<char>,
    ) -> State {
        State {
            key,
            distance,
            collected_keys,
            keys_in_order,
        }
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.collected_keys == other.collected_keys
    }
}

impl Eq for State {}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
        let mut sorted_keys: Vec<_> = self.collected_keys.iter().collect();
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
    pub start_idx: usize,
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
        let mut start_idx = 0;
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
                        } else if c.is_uppercase() {
                            doors.insert(c, idx);
                        } else if c == '@' {
                            start_idx = idx;
                        }
                        point_of_interests.insert(idx, c);
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

        assert_ne!(start_idx, 0, "Start position not defined in maze");

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

    pub fn get_distances_and_blocking_doors(
        &self,
    ) -> (
        HashMap<char, Vec<(char, usize)>>,
        HashMap<(char, char), Vec<char>>,
    ) {
        let mut distances: HashMap<char, Vec<(char, usize)>> = HashMap::new();
        let mut blocking_doors: HashMap<(char, char), Vec<char>> = HashMap::new();

        for (i, (key, key_pos)) in self.keys.iter().enumerate() {
            // check distance from start
            let path = self
                .find_shortest_connection(self.start_idx, *key_pos)
                .unwrap();
            let steps = path.len() - 1;
            distances
                .entry('@')
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
                blocking_doors.insert(('@', *key), door_on_path);
            }

            // check distance between keys
            for (other_key, other_key_pos) in self.keys.iter().skip(i + 1) {
                let path = self
                    .find_shortest_connection(*key_pos, *other_key_pos)
                    .unwrap();
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
        for distance_vec in distances.values_mut() {
            distance_vec.sort_by_key(|k| k.1);
        }
        (distances, blocking_doors)
    }

    pub fn collect_all_keys(&self) {
        let (distances, blocking_doors) = self.get_distances_and_blocking_doors();
        println!("Distances: {:?}", distances);
        println!("Distances from start: {:?}", distances.get(&'@').unwrap());
        println!("Blocked paths: {:?}", blocking_doors);

        let mut queue: Vec<State> = Vec::new();
        let initial_state = State::new('@', 0, HashSet::new(), Vec::new());
        let mut known_states: HashSet<State> = HashSet::new();
        queue.push(initial_state);

        let num_all_keys = self.keys.len();
        let mut counter = 0;
        loop {
            let node = queue.remove(0);
            if known_states.get(&node).is_some() {
                continue;
            }
            if node.collected_keys.len() == num_all_keys {
                println!("{:?}", node);
                break;
            }

            'node_neighbour_loop: for (other_key, distance) in
                distances.get(&node.key).unwrap().iter()
            {
                // check if key has already been collected
                if node.collected_keys.get(other_key).is_some() {
                    continue;
                }
                // first check if this key is even reachable
                let blockage_opt = blocking_doors.get(&(node.key, *other_key));
                if blockage_opt.is_some() {
                    let blocked = blockage_opt.unwrap();
                    for blocked_door in blocked.iter() {
                        // check if key for blocked_door was collected
                        if node
                            .collected_keys
                            .get(&blocked_door.to_ascii_lowercase())
                            .is_none()
                        {
                            continue 'node_neighbour_loop;
                        }
                    }
                }
                let mut collected_keys = node.collected_keys.clone();
                collected_keys.insert(*other_key);
                let mut keys_in_order = node.keys_in_order.clone();
                keys_in_order.push(*other_key);
                let new_node = State::new(
                    *other_key,
                    node.distance + distance,
                    collected_keys,
                    keys_in_order,
                );

                queue.push(new_node)
            }
            queue.sort_by_key(|k| k.distance);
            counter += 1;
            if counter % 1000 == 0 {
                println!("---");
                for path in queue.iter().take(10) {
                    println!("{:?}", path);
                }
            }
            known_states.insert(node);
        }
    }
}

fn grid_to_idx(x: usize, y: usize) -> usize {
    x + y * MAP_DIM_X
}

// fn idx_to_grid(idx: usize) -> (usize, usize) {
//     let x = idx % MAP_DIM_X;
//     let y = (idx - x) / MAP_DIM_X;
//     (x, y)
// }
