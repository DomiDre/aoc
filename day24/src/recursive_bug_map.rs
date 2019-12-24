use crate::{Map, RecursiveMap, GRID_SIZE};
use std::collections::VecDeque;

pub struct RecursiveBugMap {
    map : RecursiveMap,
    idx_level0: usize
}

impl RecursiveBugMap {
    pub fn new(mut _map: Map) -> RecursiveBugMap {
        let mut map = VecDeque::new();
        _map[2][2] = '?';
        map.push_back(_map);
        RecursiveBugMap {
            map,
            idx_level0: 0
        }
    }

    /// evolve following rules of part 2
    pub fn evolve(&mut self) {

        let n_levels = self.map.len();
        // first: count number of neighbour bugs for every field in every 
        // already existing level
       
        // initialize neighbours map
        let mut neighbours = VecDeque::new();
        for _k in 0..n_levels {
            neighbours.push_back([[0; GRID_SIZE]; GRID_SIZE]);
        }
        // add an additional lower and upper level
        neighbours.push_back([[0; GRID_SIZE]; GRID_SIZE]);
        neighbours.push_front([[0; GRID_SIZE]; GRID_SIZE]);

        for (k, level) in self.map.iter().enumerate() {
            for (j, row) in level.iter().enumerate() {
                for (i, c) in row.iter().enumerate() {
                    // if field is a bug, add +1 to every neighbour
                    if let '#' = c {
                        // check 4 adjacent fields around i,j
                        // to avoid signed integers, i_n, j_n are checked with value+1
                        for &(i_n, j_n) in [(0,1), (2,1), (1,2), (1,0)].iter() {
                            // check cases where level is changed: 4 edges of field
                            // & the four fields in the center
                            if i == 0 && i_n == 0 {
                                //higher level left field
                                neighbours[k][2][1] += 1;

                            } 
                            else if i == GRID_SIZE - 1 && i_n == 2 {
                                //higher level right field
                                neighbours[k][2][3] += 1;
                            }
                            else if j == 0 && j_n == 0 {
                                //higher level upper field
                                neighbours[k][1][2] += 1;
                            }
                            else if j == GRID_SIZE - 1 && j_n == 2 {
                                //higher level lower field
                                neighbours[k][3][2] += 1;
                            } 
                            else if i == 1 && j == 2 && i_n == 2 {
                                //lower level left edge
                                for n in 0..GRID_SIZE {
                                    neighbours[k+2][n][0] += 1;
                                }
                            } 
                            else if i == 3 && j == 2 && i_n == 0 {
                                //lower level right edge
                                for n in 0..GRID_SIZE {
                                    neighbours[k+2][n][4] += 1;
                                }
                            } 
                            else if i == 2 && j == 1 && j_n == 2 {
                                //lower level upper edge
                                for n in 0..GRID_SIZE {
                                    neighbours[k+2][0][n] += 1;
                                }
                            } 
                            else if i == 2 && j == 3 && j_n == 0 {
                                //lower level lower edge
                                for n in 0..GRID_SIZE {
                                    neighbours[k+2][4][n] += 1;
                                }
                            } else {
                                neighbours[k+1][j + j_n - 1][i + i_n - 1] += 1;
                            }
                        }
                    }
                }
            }
        }

        // update maps
        // add empty maps for higher and lower level to align with neighbours
        self.map.push_back([['.'; GRID_SIZE]; GRID_SIZE]);
        self.map.push_front([['.'; GRID_SIZE]; GRID_SIZE]);
        self.map.front_mut().unwrap()[2][2] = '?';
        self.map.back_mut().unwrap()[2][2] = '?';
        
        self.idx_level0 += 1;
        for (k, level) in self.map.iter_mut().enumerate() {
            for (j, row) in level.iter_mut().enumerate() {
                for (i, c) in row.iter_mut().enumerate() {
                    match c {
                        '#' => {
                            // a bug dies unless it has exactly one neighbour
                            if neighbours[k][j][i] != 1 {
                                *c = '.'
                            }
                        },
                        '.' => {
                            // an empty space becomes infested if it has 1 or 2 neighbours
                            if neighbours[k][j][i] == 1 ||  neighbours[k][j][i] == 2 {
                                *c = '#'
                            }
                        },
                        _ => {
    
                        }
                    }
                }
            }
        }
        
        // check if upper or lower map contains any bugs, else remove it
        if self.bugs_in_level(self.map.front().unwrap()) == 0 {
            self.map.pop_front();
            self.idx_level0 -= 1;
        }
        if self.bugs_in_level(self.map.back().unwrap()) == 0 {
            self.map.pop_back();
        }
        
    }

    fn bugs_in_level(&self, level: &Map) -> usize {
        let mut number_of_bugs = 0;
        for row in level.iter() {
            for c in row.iter() {
                if let '#' = c {
                    number_of_bugs += 1;
                }
            }
        }
        number_of_bugs
    }

    pub fn number_of_bugs(&self) -> usize {
        let mut number_of_bugs = 0;
        for level in self.map.iter() {
            number_of_bugs += self.bugs_in_level(level);
        }
        number_of_bugs
    }
}

impl std::fmt::Display for RecursiveBugMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map_string = "".to_string();
        for (i, level) in self.map.iter().enumerate() {
            map_string += &format!("Depth {}:\n", i as i32 - self.idx_level0 as i32);
            for row in level.iter() {
                for c in row.iter() {
                    map_string += &c.to_string();
                }
                map_string += &"\n".to_string();
            }
        }
        write!(f, "{}", map_string)
    }
}