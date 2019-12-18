use crate::computer::IntcodeComputer;

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

pub struct VacuumRobot {
    pub computer: IntcodeComputer,
    pub position: (i64, i64),
    pub direction: Direction,
    pub map: Vec<Vec<char>>,
    pub n_rows: usize,
    pub n_cols: usize,
    pub terminated: bool
}

impl VacuumRobot {
    pub fn new(intcodes: Vec<i64>) -> VacuumRobot {
        let mut computer = IntcodeComputer::new(intcodes.clone());
        // initialize camera
        computer.show_stdinout = false;
        let mut map: Vec<Vec<char>> = Vec::new();
        map.push(Vec::new());
        let mut n_rows = 0;
        loop {
            computer.run();
            match computer.memory_output {
                10 => {
                    // two times 10, means end of map reading
                    if map[n_rows].len() == 0 {
                        map.pop();
                        break;
                    }
                    map.push(Vec::new());
                    n_rows += 1;
                }
                _ => {
                    map[n_rows].push(computer.memory_output as u8 as char);
                }
            }

            if computer.terminated || computer.requesting_input {
                break;
            }
        }
        n_rows = map.len();
        let n_cols = map[0].len();
        // note that map stores coordinates as [y][x], and up is towards lower y

        // find vacuum robot position & determine its direction
        let mut direction: Direction = Direction::Right;
        let mut position: (i64, i64) = (0, 0);
        for i in 0..n_rows {
            for j in 0..n_cols {
                let symbol = map[i][j];
                if symbol == '^' {
                    direction = Direction::Up;
                    position = (i as i64, j as i64);
                } else if symbol == 'v' {
                    direction = Direction::Down;
                    position = (i as i64, j as i64);
                } else if symbol == '>' {
                    direction = Direction::Right;
                    position = (i as i64, j as i64);
                } else if symbol == '<' {
                    direction = Direction::Left;
                    position = (i as i64, j as i64);
                }
            }
        }
        VacuumRobot {
            computer,
            position,
            direction,
            map,
            n_rows,
            n_cols,
            terminated: false
        }
    }

    pub fn run(&mut self) {
        loop {
            self.computer.run();
            // if stops because it's terminated or requesting input -> stop
            if self.computer.terminated || self.computer.requesting_input {
                self.terminated = self.computer.terminated;
                break;
            }
            print!("{}", (self.computer.memory_output as u8) as char);
        }
    }

    pub fn run_chain_of_inputs(&mut self, commands: Vec<String>) {
        print!("<< ");
        for (i, command) in commands.iter().enumerate() {
            for character in command.chars() {
                self.computer.set_memory_input((character as u8) as i64);
                print!("{}", character);
                self.computer.run();
            }
            // if not last command, seperate next by comma
            if i < commands.len() - 1 {
                self.computer.set_memory_input((',' as u8) as i64);
                print!(",");
                self.computer.run();
            }
        }
        self.computer.set_memory_input(10);
        println!();
    }

    pub fn get_adjacent_position(&self, direction: &Direction) -> (i64, i64) {
        match direction {
            Direction::Up => (self.position.0 - 1, self.position.1),
            Direction::Down => (self.position.0 + 1, self.position.1),
            Direction::Left => (self.position.0, self.position.1 - 1),
            Direction::Right => (self.position.0, self.position.1 + 1),
        }
    }

    pub fn move_forward(&mut self) {
        match self.direction {
            Direction::Up => self.position.0 -= 1,
            Direction::Down => self.position.0 += 1,
            Direction::Left => self.position.1 -= 1,
            Direction::Right => self.position.1 += 1,
        };
    }

    pub fn position_in_map(&self, position: &(i64, i64)) -> bool {
        return position.0 < (self.n_rows as i64)
            && position.0 >= 0
            && position.1 < (self.n_cols as i64)
            && position.1 >= 0;
    }

    pub fn turn_left(&mut self) {
        self.direction = self.direction.turn_left();
    }

    pub fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();
    }
}
