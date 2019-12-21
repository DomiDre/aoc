use std::collections::{HashMap, HashSet, VecDeque};

pub type Map = Vec<Vec<char>>;

pub struct DonutMaze {
    pub map: Map,
    pub idx_start: usize,
    pub idx_end: usize,
    adjacency_list: HashMap<usize, Vec<usize>>,
}

impl DonutMaze {
    pub fn new(map: Map) -> DonutMaze {
        let n_cols = map[0].len();
        let n_rows = map.len();

        // create mapping from points connected by teleporter
        let mut teleport_connections: HashMap<usize, usize> = HashMap::new();
        let mut found_teleports: HashMap<String, usize> = HashMap::new();
        for y in 1..n_rows - 1 {
            for x in 1..n_cols - 1 {
                // ignore outer edge of maze to avoid overflow checks
                let c = map[y][x];

                match c {
                    '.' => {
                        // position is a walkable position
                        // analyze & remember this position
                    }
                    'A'..='Z' => {
                        let dot_idx: usize;
                        let c_n;
                        if map[y][x + 1] == '.' && map[y][x - 1].is_ascii_uppercase() {
                            dot_idx = grid_to_idx(x + 1, y, n_cols);
                            c_n = map[y][x - 1];
                        } else if map[y][x - 1] == '.' && map[y][x + 1].is_ascii_uppercase() {
                            dot_idx = grid_to_idx(x - 1, y, n_cols);
                            c_n = map[y][x + 1];
                        } else if map[y - 1][x] == '.' && map[y + 1][x].is_ascii_uppercase() {
                            dot_idx = grid_to_idx(x, y - 1, n_cols);
                            c_n = map[y + 1][x];
                        } else if map[y + 1][x] == '.' && map[y - 1][x].is_ascii_uppercase() {
                            dot_idx = grid_to_idx(x, y + 1, n_cols);
                            c_n = map[y - 1][x];
                        } else {
                            continue;
                        }
                        // check if teleportation name is already known
                        if let Some(other_idx) =
                            found_teleports.get(&(c.to_string() + &c_n.to_string()))
                        {
                            teleport_connections.insert(dot_idx, *other_idx);
                            teleport_connections.insert(*other_idx, dot_idx);
                        } else if let Some(other_idx) =
                            found_teleports.get(&(c_n.to_string() + &c.to_string()))
                        {
                            teleport_connections.insert(dot_idx, *other_idx);
                            teleport_connections.insert(*other_idx, dot_idx);
                        } else {
                            found_teleports.insert(c.to_string() + &c_n.to_string(), dot_idx);
                        }
                    }
                    _ => {
                        // position is either a wall or nothing
                        // not of interest
                    }
                }
            }
        }

        // loop second time over each point, this time creating adjacency lists
        // including information from teleport connections
        let mut adjacency_list: HashMap<usize, Vec<usize>> = HashMap::new();
        for y in 2..n_rows - 2 {
            for x in 2..n_cols - 2 {
                // ignore outer edge of maze to avoid overflow checks
                let c = map[y][x];
                let idx = grid_to_idx(x, y, n_cols);
                if let '.' = c {
                    let mut adjacent_walkable_points = Vec::new();

                    // check each neighbour if it's walkable
                    let neighbours = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
                    for &(x_n, y_n) in neighbours.iter() {
                        let c_n = map[y_n][x_n];
                        if c_n == '.' {
                            let idx_n = grid_to_idx(x_n, y_n, n_cols);
                            adjacent_walkable_points.push(idx_n);
                        }
                    }

                    // check if point is a teleportation point
                    let opt_teleport = teleport_connections.get(&idx);
                    if opt_teleport.is_some() {
                        adjacent_walkable_points.push(*opt_teleport.unwrap());
                    }
                    adjacency_list.insert(idx, adjacent_walkable_points);
                }
            }
        }
        let idx_start = *found_teleports.get(&"AA".to_string()).unwrap();
        let idx_end = *found_teleports.get(&"ZZ".to_string()).unwrap();

        // for (name, teleport_idx) in adjacency_list.clone() {
        //     println!("{} {:?}", name, teleport_idx);
        // }
        DonutMaze {
            map,
            idx_start,
            idx_end,
            adjacency_list,
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
}

fn grid_to_idx(x: usize, y: usize, n_cols: usize) -> usize {
    x + y * n_cols
}
