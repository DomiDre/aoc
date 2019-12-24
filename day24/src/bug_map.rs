use crate::{Map, GRID_SIZE};

pub struct BugMap {
    map : Map
}

impl BugMap {
    pub fn new(map: Map) -> BugMap {
        BugMap {
            map
        }
    }

    /// evolve following rules of part 1
    pub fn evolve(&mut self) {
        // count number of neighbour bugs for every field
        let mut neighbours = [[0; GRID_SIZE]; GRID_SIZE];
        for (j, row) in self.map.iter().enumerate() {
            for (i, c) in row.iter().enumerate() {
                // if field is a bug, add +1 to every neighbour
                if let '#' = c {
                    // check 4 adjacent fields around i,j
                    // to avoid signed integers, i_n, j_n are checked with value+1
                    for &(i_n, j_n) in [(0,1), (2,1), (1,2), (1,0)].iter() {
                        // skip out of bound cases
                        if (i == 0 && i_n == 0) 
                        || (j == 0 && j_n == 0)
                        || (i == GRID_SIZE - 1 && i_n == 2)
                        || (j == GRID_SIZE - 1 && j_n == 2){
                            continue
                        }
                        neighbours[j + j_n - 1][i + i_n - 1] += 1;
                    }
                }
            }
        }
        // update map
        for (j, row) in self.map.iter_mut().enumerate() {
            for (i, c) in row.iter_mut().enumerate() {
                match c {
                    '#' => {
                        // a bug dies unless it has exactly one neighbour
                        if neighbours[j][i] != 1 {
                            *c = '.'
                        }
                    },
                    '.' => {
                        // an empty space becomes infested if it has 1 or 2 neighbours
                        if neighbours[j][i] == 1 ||  neighbours[j][i] == 2 {
                            *c = '#'
                        }
                    },
                    _ => {

                    }
                }
            }
        }
    }

    pub fn biodiversity_rating(&self) -> usize {
        let mut rating = 0;
        let mut power = 1;
        for row in self.map.iter() {
            for c in row.iter() {
                if let '#' = c {
                    rating += power;
                }
                power *= 2;
            }
        }
        rating
    }
}

impl std::fmt::Display for BugMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map_string = "".to_string();
        for row in self.map.iter() {
            for c in row.iter() {
                map_string += &c.to_string();
            }
            map_string += &"\n".to_string();
        }
        write!(f, "{}", map_string)
    }
}