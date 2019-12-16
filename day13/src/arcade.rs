use crate::computer::IntcodeComputer;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Tile {
    Empty,
    Wall,
    Block,
    Horizontal,
    Ball
}

impl Tile {
    pub fn get_tile(code: usize) -> Option<Tile> {
        match code {
            0 => Some(Tile::Empty),
            1 => Some(Tile::Wall),
            2 => Some(Tile::Block),
            3 => Some(Tile::Horizontal),
            4 => Some(Tile::Ball),
            _ => None
        }
    }
}

pub enum JoystickStates {
    Neutral,
    LeftTilt,
    RightTilt
}

impl JoystickStates {
    pub fn state_to_instruction(&self) -> i64 {
        match self {
            JoystickStates::Neutral => 0,
            JoystickStates::LeftTilt => -1,
            JoystickStates::RightTilt => 1
        }
    }
}

pub struct Arcade {
    pub computer: IntcodeComputer,
    pub screen: HashMap<(i64,i64), Tile>,
    joystickState: JoystickStates,
    pub ball_pos: (i64, i64),
    pub horizontal_pos: (i64, i64),
    pub score: i64
}

impl Arcade {
    pub fn new(intcodes: Vec<i64>) -> Arcade {
        let mut computer = IntcodeComputer::new(intcodes);
        computer.show_stdinout = false;
        Arcade {
            computer,
            screen: HashMap::new(),
            joystickState: JoystickStates::Neutral,
            ball_pos: (0, 0),
            horizontal_pos: (0, 0),
            score: 0
        }
    }

    pub fn run(&mut self) {
        loop {
            self.computer.run();
            if self.computer.requesting_input {
                break;
            }
            let x = self.computer.memory_output;
            self.computer.run();
            let y = self.computer.memory_output;
            self.computer.run();
            let value = self.computer.memory_output;
            if (x == -1) && (y == 0) {
                self.score = value;
            } else {
                let tile = Tile::get_tile(value as usize);
                if tile.is_some() {
                    if let Some(Tile::Ball) = tile {
                        self.ball_pos = (x,y);
                    } else if let Some(Tile::Horizontal) = tile {
                        self.horizontal_pos = (x,y);
                    }
                    self.screen.insert((x,y), tile.unwrap());
                }
            }
            if self.computer.terminated {
                break;
            }
        }
    }

    pub fn joystick_input(&mut self, direction: JoystickStates) {
        self.computer.set_memory_input(direction.state_to_instruction());
    }

    pub fn get_screen(&self) -> String {
        // first find max dimension of field
        let mut screen = String::new();
        let mut x_range = (0, 0);
        let mut y_range = (0, 0);
        for field in self.screen.keys() {
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
        for y in (y_range.0..=y_range.1) {
            for x in x_range.0..=x_range.1 {
                let ch = match self.screen.get(&(x, y)) {
                    Some(Tile::Empty) => ".",
                    Some(Tile::Wall) => "|",
                    Some(Tile::Block) => "#",
                    Some(Tile::Horizontal) => "-",
                    Some(Tile::Ball) => "o",
                    _ => "",
                };
                screen += ch;
            }
            screen += "\n";
        }
        screen
    }
}