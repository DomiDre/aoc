use crate::computer::IntcodeComputer;

pub struct SpringDroid {
    computer: IntcodeComputer
}

impl SpringDroid {
    pub fn new(intcodes: Vec<i64>) -> SpringDroid {
        let computer = IntcodeComputer::new(intcodes);
        SpringDroid {
            computer
        }
    }

    pub fn run(&mut self) {
        loop {
            self.computer.run();
            if self.computer.terminated || self.computer.requesting_input {
                if self.computer.terminated {
                    println!("{}", self.computer.memory_output);
                }
                break;
            }
        }

    }

    pub fn input_ascii(&mut self, command: &str) {
        for character in command.chars() {
            self.computer.set_memory_input((character as u8) as i64);
            self.computer.run();
        }
        self.computer.set_memory_input(10);
        self.run();
    }
}