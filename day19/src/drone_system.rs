use crate::computer::IntcodeComputer;
use std::fmt;

pub enum DroneState {
    Stationary,
    Pulled,
}

impl DroneState {
    fn output_to_state(digit: i64) -> Option<DroneState> {
        match digit {
            0 => Some(DroneState::Stationary),
            1 => Some(DroneState::Pulled),
            _ => None,
        }
    }
}

impl fmt::Display for DroneState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DroneState::Stationary => write!(f, "Stationary"),
            DroneState::Pulled => write!(f, "Pulled"),
        }
    }
}

pub struct DroneSystem {
    pub computer: IntcodeComputer,
    intcodes: Vec<i64>,
}

impl DroneSystem {
    pub fn new(intcodes: Vec<i64>) -> DroneSystem {
        let mut computer = IntcodeComputer::new(intcodes.clone());
        computer.show_stdinout = false;
        DroneSystem { computer, intcodes }
    }

    fn restart_computer(&mut self) {
        self.computer = IntcodeComputer::new(self.intcodes.clone());
        self.computer.show_stdinout = false;
    }

    pub fn deploy_drone(&mut self, x: usize, y: usize) -> DroneState {
        self.restart_computer();
        self.computer.run();
        self.computer.set_memory_input(x as i64);
        self.computer.run();
        self.computer.set_memory_input(y as i64);
        self.computer.run();
        if let Some(droid_state) = DroneState::output_to_state(self.computer.memory_output) {
            droid_state
        } else {
            panic!("Computer output invalid drone state upon drone deployment.")
        }
    }
}
