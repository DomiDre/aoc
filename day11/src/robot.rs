use std::collections::HashMap;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_90deg_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn turn_90deg_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
}

pub struct PaintingRobot {
    facing_direction: Direction,
    position: (i64, i64),
    painted_fields: HashMap<(i64, i64), u8>,
}

impl PaintingRobot {
    /// Initialize a robot
    pub fn new() -> PaintingRobot {
        PaintingRobot {
            facing_direction: Direction::Up,
            position: (0, 0),
            painted_fields: HashMap::new(),
        }
    }

    /// Turn robot left or right depending on input command
    /// 0 means turn left 90 degrees,
    /// 1 means turn right 90 degrees
    pub fn turn_robot(&mut self, command: i64) {
        self.facing_direction = match command {
            0 => self.facing_direction.turn_90deg_left(),
            1 => self.facing_direction.turn_90deg_right(),
            _ => panic!("Turn command received unknown command."),
        };
    }

    /// Move the robot one step into current facing direction
    pub fn move_robot(&mut self) {
        match self.facing_direction {
            Direction::Left => self.position.0 -= 1,
            Direction::Right => self.position.0 += 1,
            Direction::Down => self.position.1 -= 1,
            Direction::Up => self.position.1 += 1,
        }
    }

    /// Get color at argument position
    fn color_at_position(&self, position: &(i64, i64)) -> u8 {
        let opt_field = self.painted_fields.get(position);
        if let Some(i) = opt_field {
            *i
        } else {
            0
        }
    }

    /// Access robot camera to get color of current field
    pub fn get_color(&self) -> u8 {
        self.color_at_position(&self.position)
    }

    /// Function called by the intcode program
    /// To color the current position
    pub fn color(&mut self, color: i64) {
        // translate position which can be positive/negative to
        // grid coordinates (only positive)

        match color {
            0 | 1 => {
                self.painted_fields.insert(self.position, color as u8);
            }
            _ => panic!("Invalid color."),
        };
    }

    /// get number of individually painted fields
    pub fn get_individually_painted_fields(&self) -> usize {
        self.painted_fields.len()
    }

    /// print painted fields
    pub fn print_painted_fields(&self) {
        // first find max dimension of field
        let mut x_range = (0, 0);
        let mut y_range = (0, 0);
        for field in self.painted_fields.keys() {
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
                match self.color_at_position(&(x, y)) {
                    0 => print!("."),
                    1 => print!("#"),
                    _ => panic!("Invalid color detected."),
                }
            }
            println!();
        }
    }
}
