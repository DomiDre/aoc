use crate::Map;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

// to keep track of state in maze, one also needs to also account for the level
#[derive(Debug, Copy, Clone)]
pub struct State {
    position: usize,
    level: usize,
    steps: usize
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.level == other.level
    }
}

impl Eq for State {}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.level.hash(state);
    }
}

pub struct RecursiveDonutMaze {
    pub map: Map,
    pub idx_start: usize,
    pub idx_end: usize,
    adjacency_list: HashMap<usize, Vec<usize>>,
    teleport_connections: HashMap<usize, usize>,
    teleporter_at_outer_edge: HashMap<usize, bool>,
}

impl RecursiveDonutMaze {
    pub fn new(map: Map) -> RecursiveDonutMaze {
        let n_cols = map[0].len();
        let n_rows = map.len();

        // create mapping from points connected by teleporter
        let mut teleport_connections = HashMap::new();
        let mut found_teleports = HashMap::new();
        let mut teleporter_at_outer_edge = HashMap::new();
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
                        let is_outer_edge;
                        if map[y][x + 1] == '.' && map[y][x - 1].is_ascii_uppercase() {
                            dot_idx = grid_to_idx(x + 1, y, n_cols);
                            c_n = map[y][x - 1];
                            is_outer_edge = x <= 2 || x >= n_cols - 3;
                        } else if map[y][x - 1] == '.' && map[y][x + 1].is_ascii_uppercase() {

                            dot_idx = grid_to_idx(x - 1, y, n_cols);
                            c_n = map[y][x + 1];
                            is_outer_edge = x <= 2 || x >= n_cols - 3;
                        } else if map[y - 1][x] == '.' && map[y + 1][x].is_ascii_uppercase() {
                            dot_idx = grid_to_idx(x, y - 1, n_cols);
                            c_n = map[y + 1][x];
                            is_outer_edge = y <= 2 || y >= n_rows - 3;
                        } else if map[y + 1][x] == '.' && map[y - 1][x].is_ascii_uppercase() {
                            dot_idx = grid_to_idx(x, y + 1, n_cols);
                            c_n = map[y - 1][x];
                            is_outer_edge = y <= 2 || y >= n_rows - 3;
                        } else {
                            continue;
                        }
                        // take not of teleporting point and whether it's an outer edge
                        // or inner edge one to know if it takes you a level up or down
                        teleporter_at_outer_edge.insert(dot_idx, is_outer_edge);
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
                    if let Some(teleport_point) = teleport_connections.get(&idx) {
                        adjacent_walkable_points.push(*teleport_point);
                    }
                    adjacency_list.insert(idx, adjacent_walkable_points);
                }
            }
        }
        let idx_start = *found_teleports.get(&"AA".to_string()).unwrap();
        let idx_end = *found_teleports.get(&"ZZ".to_string()).unwrap();

        RecursiveDonutMaze {
            map,
            idx_start,
            idx_end,
            adjacency_list,
            teleport_connections,
            teleporter_at_outer_edge,
        }
    }

    /// For two points start and end, this function determines the number of steps along
    /// the shortest path
    pub fn find_shortest_connection(&self, start: usize, end: usize) -> Option<usize> {
        let mut queue = VecDeque::new();
        queue.push_back(State {
            position: start,
            level: 0,
            steps: 0
        });

        // keep track which nodes have already been visited
        let mut visited: HashSet<State> = HashSet::new();
        // store for every visited node how one got here starting from the start point
        let mut parent: HashMap<State, State> = HashMap::new();

        loop {
            if let Some(node) = queue.pop_front() {
                if node.position == end && node.level == 0 {
                    // found the searched node
                    return Some(node.steps);
                }
                let mut neighbour_level = node.level;
                // check if position is potential teleport position for extra checks
                let is_teleport_point = self.teleport_connections.get(&node.position).is_some();

                // by definition neighbours could not be empty
                // or the adjacency list is uncomplete
                let neighbours = self.adjacency_list.get(&node.position).unwrap();
                for neighbour in neighbours.iter() {
                    if is_teleport_point
                        && self.teleport_connections.get(&node.position).unwrap() == neighbour
                    {
                        // this is an teleportation path
                        if let Some(true) = self.teleporter_at_outer_edge.get(&node.position) {
                            //outer edge path
                            if node.level == 0 {
                                // we are at the outer edge but on level 0 -> invalid path
                                continue;
                            } else {
                                //outer edge but on an lower level, that's fine and will
                                // take us one level higher
                                neighbour_level -= 1;
                            }
                        } else {
                            // inner edge path
                            neighbour_level += 1;
                        }
                    }
                    let next_state = State {
                        position: *neighbour,
                        level: neighbour_level,
                        steps: node.steps + 1
                    };
                    if visited.get(&next_state).is_some() {
                        // if state has already been visited -> ignore
                        continue;
                    }

                    // push neighbour into the queue of paths to check
                    queue.push_back(next_state);

                    // note that next state is visited coming from this state
                    parent.insert(next_state, node);
                }
                // make note that state has been visited
                visited.insert(node);
            } else {
                // the queue is empty, there is no way to reach end
                return None;
            }
        }
    }
}

fn grid_to_idx(x: usize, y: usize, n_cols: usize) -> usize {
    x + y * n_cols
}