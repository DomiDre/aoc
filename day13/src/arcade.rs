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
    pub fn get_tile(code: usize) -> Tile {
        match code {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Horizontal,
            4 => Tile::Ball,
            _ => panic!("Unknown tile code")
        }
    }
}

enum JoystickStates {
    Neutral,
    LeftTilt,
    RightTilt
}

impl JoystickStates {
    fn state_to_instruction(&self) -> i64 {
        match self {
            JoystickStates::Neutral => 0,
            JoystickStates::LeftTilt => -1,
            JoystickStates::RightTilt => 1
        }
    }
}

pub struct Arcade {
    computer: IntcodeComputer,
    pub screen: HashMap<(i64,i64), Tile>,
    joystickState: JoystickStates
}

impl Arcade {
    pub fn new(intcodes: Vec<i64>) -> Arcade {
        let mut computer = IntcodeComputer::new(intcodes);
        computer.show_stdinout = false;
        Arcade {
            computer,
            screen: HashMap::new()
        }
    }

    pub fn run(&mut self) {
        loop {
            self.computer.run();
            let x = self.computer.memory_output;
            self.computer.run();
            let y = self.computer.memory_output;
            self.computer.run();
            let tile = Tile::get_tile(self.computer.memory_output as usize);
            self.screen.insert((x,y), tile);
            if self.computer.terminated {
                break;
            }
        }
    }
}