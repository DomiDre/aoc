use crate::computer::IntcodeComputer;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Movement {
    North,
    South,
    West,
    East,
}

impl Movement {
    pub fn to_code(&self) -> i64 {
        match self {
            Movement::North => 1,
            Movement::South => 2,
            Movement::West => 3,
            Movement::East => 4,
        }
    }

    pub fn turn_anticlockwise(&self) -> Movement {
        match self {
            Movement::North => Movement::West,
            Movement::South => Movement::East,
            Movement::West => Movement::South,
            Movement::East => Movement::North,
        }
    }

    pub fn turn_clockwise(&self) -> Movement {
        match self {
            Movement::North => Movement::East,
            Movement::South => Movement::West,
            Movement::West => Movement::North,
            Movement::East => Movement::South,
        }
    }

    pub fn turn_anticlockwise_n(&self, n: u8) -> Movement {
        let mut direction = self.clone();
        for _i in 0..n {
            direction = direction.turn_anticlockwise();
        }
        direction
    }

    pub fn turn_clockwise_n(&self, n: u8) -> Movement {
        let mut direction = self.clone();
        for _i in 0..n {
            direction = direction.turn_clockwise();
        }
        direction
    }
}


pub enum Tile {
    Wall,
    Floor,
    OxygenFilledFloor
}

pub struct RepairDroid {
    pub computer: IntcodeComputer,
    pub position: (i64, i64),
    pub droid_status: i64,
    pub map: HashMap<(i64, i64), Tile>,
    pub oxygen_station: Option<(i64, i64)>,
    pub walkable_positions: HashMap<(i64, i64), bool>,
    pub deadend_path: HashMap<(i64, i64), bool>,
    pub found_oxygen_station: bool,
    pub explored_whole_map: bool,
    direction: Movement,
    tested_positions: HashMap<(i64, i64), bool>,
    in_deadend: bool,
}

impl RepairDroid {
    pub fn new(intcodes: Vec<i64>) -> RepairDroid {
        let mut computer = IntcodeComputer::new(intcodes);
        computer.show_stdinout = false;
        // init computer
        computer.run();
        RepairDroid {
            computer,
            position: (0, 0),
            droid_status: -1,
            oxygen_station: None,
            map: HashMap::new(),
            walkable_positions: HashMap::new(),
            deadend_path: HashMap::new(),
            found_oxygen_station: false,
            explored_whole_map: false,
            direction: Movement::North,
            tested_positions: HashMap::new(),
            in_deadend: false,
        }
    }

    pub fn move_droid(&mut self, movement: &Movement) {
        self.computer.set_memory_input(movement.to_code());
        self.computer.run();
        let droid_status = self.computer.memory_output;
        let prospective_position = self.get_move_position(movement);
        match droid_status {
            0 => {
                self.map.insert(prospective_position, Tile::Wall);
            }
            1 => {
                self.position = prospective_position;
                self.map.insert(prospective_position, Tile::Floor);
            }
            2 => {
                self.position = prospective_position;
                self.oxygen_station = Some(prospective_position);
            }
            _ => (),
        }
        self.droid_status = droid_status;
        self.computer.run();
    }

    pub fn get_move_position(&self, movement: &Movement) -> (i64, i64) {
        match movement {
            Movement::North => (self.position.0, self.position.1 + 1),
            Movement::South => (self.position.0, self.position.1 - 1),
            Movement::West => (self.position.0 - 1, self.position.1),
            Movement::East => (self.position.0 + 1, self.position.1),
        }
    }

    pub fn draw_map(&self) -> String {
        // first find max dimension of field
        let mut screen = String::new();
        let mut x_range = (0, 0);
        let mut y_range = (0, 0);
        for field in self.map.keys() {
            if field.0 < x_range.0 {
                x_range.0 = field.0;
            }
            if field.0 > x_range.1 {
                x_range.1 = field.0
            }

            if field.1 < y_range.0 {
                y_range.0 = field.1;
            }
            if field.1 > y_range.1 {
                y_range.1 = field.1
            }
        }

        for y in (y_range.0..=y_range.1).rev() {
            for x in x_range.0..=x_range.1 {
                let ch: &str;
                if (x, y) == self.position {
                    ch = "D"
                } else {
                    ch = match self.map.get(&(x, y)) {
                        Some(Tile::Floor) => ".",
                        Some(Tile::Wall) => "#",
                        Some(Tile::OxygenFilledFloor) => "O",
                        _ => "?",
                    };
                }
                screen += ch;
            }
            screen += "\n";
        }
        screen
    }

    pub fn explore(&mut self) {
        //move droid and store that the tried positions was tested
        let try_new_pos = self.get_move_position(&self.direction);
        self.tested_positions.insert(try_new_pos, true);
        self.move_droid(&self.direction.clone());
        match self.droid_status {
            0 => {
                // hit a wall after moving
                // turn one time and check if any way is untested
                for i in 1..=3 {
                    let test_direction = self.direction.turn_anticlockwise_n(i);
                    let prospective_pos = self.get_move_position(&test_direction);
                    if self.tested_positions.get(&prospective_pos).is_none() {
                        // direction is unknown
                        self.direction = test_direction;
                        return;
                    }
                }

                // forward is wall, and all other three directions have been tested
                // we are in a deadend and have to turn around
                self.deadend_path.insert(self.position, true);
                // if this point is reached all three directions have already been
                // tested -> go back the walkable path
                for i in 1..=3 {
                    let test_direction = self.direction.turn_anticlockwise_n(i);
                    let prospective_pos = self.get_move_position(&test_direction);
                    // check if direction is walkable (know its tested from check before)
                    // and avoid paths that have been marked as dead end before
                    if self.walkable_positions.get(&prospective_pos).is_some()
                        && self.deadend_path.get(&prospective_pos).is_none()
                    {
                        self.direction = test_direction;
                        return;
                    }
                }

                // at this point all directions are either a wall or a deadend
                // go into next walkable direction forward
                // turn clockwise instead anti-clockwise to avoid infinite loop
                for i in 1..=3 {
                    let test_direction = self.direction.turn_clockwise_n(i);
                    let prospective_pos = self.get_move_position(&test_direction);
                    if self.walkable_positions.get(&prospective_pos).is_some() {
                        self.direction = test_direction;
                        return;
                    }
                }
            }
            1 => {
                if self.position == (0, 0) {
                    // returned back to first position
                    self.explored_whole_map = true;
                    return;
                }
                // droid is on a floor position
                if self.walkable_positions.get(&self.position).is_none() {
                    // position was not visited before
                    self.walkable_positions.insert(self.position, true);

                    if self.in_deadend {
                        self.deadend_path.remove(
                            &self.get_move_position(&self.direction.turn_anticlockwise_n(2)),
                        );
                        self.in_deadend = false;
                    }
                } else {
                    self.in_deadend = true;
                    // have already been here... so I am on a way back
                    // this is therefore a way back from a deadend
                    // check if field is known as deadend path
                    if self.deadend_path.get(&self.position).is_none() {
                        self.deadend_path.insert(self.position, true);
                    }
                    //look ahead, is the next position known to be a dead end?
                    let forward_pos = self.get_move_position(&self.direction);
                    if self.deadend_path.get(&forward_pos).is_some() {
                        let left_turn = self.direction.turn_anticlockwise();
                        let left_pos = self.get_move_position(&left_turn);
                        let right_turn = self.direction.turn_clockwise();
                        let right_pos = self.get_move_position(&right_turn);
                        // is left or right a non dead path way? -> turn
                        if self.deadend_path.get(&left_pos).is_none() {
                            self.direction = left_turn;
                        } else if self.deadend_path.get(&right_pos).is_none() {
                            self.direction = right_turn;
                        }
                    }

                    if self.deadend_path.get(&self.position).is_none() {
                        self.deadend_path.insert(self.position, true);
                    } else {
                        // if already known, we are walking a second time on a
                        // deadend path, check left and
                        // right if a walkable path is available that is not
                        // part of the dead end
                        let left_turn = self.direction.turn_anticlockwise();
                        let right_turn = self.direction.turn_clockwise();
                        if self
                            .deadend_path
                            .get(&self.get_move_position(&left_turn))
                            .is_none()
                        {
                            self.direction = left_turn;
                        } else if self
                            .deadend_path
                            .get(&self.get_move_position(&right_turn))
                            .is_none()
                        {
                            self.direction = right_turn;
                        }
                        // otherwise just go on further forward
                    }
                }
            }
            2 => {
                self.found_oxygen_station = true;
                self.map.insert(self.position, Tile::OxygenFilledFloor);
                return;
            }
            _ => (),
        }
    }
}
