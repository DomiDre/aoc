use crate::computer::IntcodeComputer;

pub struct AsciiComputer {
    computer: IntcodeComputer,
    pub terminated: bool
}

impl AsciiComputer {
    pub fn new(intcodes: Vec<i64>) -> AsciiComputer {
        let mut computer = IntcodeComputer::new(intcodes);
        AsciiComputer {
            computer,
            terminated: false
        }
    }

    pub fn run(&mut self) {
        loop {
            self.computer.run();
            if self.computer.terminated || self.computer.requesting_input {
                self.terminated = self.computer.terminated;
                break;
            }
            print!("{}", self.computer.memory_output as u8 as char);
        }
        println!();
    }

    pub fn input_ascii(&mut self, command: &str) {
        for character in command.chars() {
            self.computer.set_memory_input((character as u8) as i64);
            self.computer.run();
        }
        self.computer.set_memory_input(10);
        // self.run();
    }

    pub fn enter_command(&mut self, command: &str) {
        self.input_ascii(command);
        self.run();
    }
}