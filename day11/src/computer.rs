pub struct IntcodeComputer {
    intcodes: [i64; 125_000], // 125000*8 Bytes = 1 MBytes
    ip: usize,
    memory_input: Option<i64>,
    pub memory_output: i64,
    pmodes: [u8; 3],
    relative_base: i64,
    pub debug_mode: bool,
    pub show_stdinout: bool,
    pub terminated: bool,
}

impl IntcodeComputer {
    /// Initialize the intcode computer
    pub fn new(intcodes: Vec<i64>) -> IntcodeComputer {
        let mut initial_memory = [0; 125_000];
        for (i, value) in intcodes.iter().enumerate() {
            initial_memory[i] = *value;
        }
        IntcodeComputer {
            intcodes: initial_memory,
            ip: 0,
            memory_input: None,
            memory_output: 0,
            pmodes: [0, 0, 0],
            relative_base: 0,
            show_stdinout: true,
            debug_mode: false,
            terminated: false,
        }
    }

    fn debug_print(&self, message: String) {
        if self.debug_mode {
            print!("{}", message);
        }
    }

    /// calculate output for given intcode table, with inputs and starting from ip
    /// makes a break at every output to be restarted from ip
    /// return tuple of current (ip, output). If opcode 99 is reached, ip i set to -1
    /// to flag termination
    pub fn run(&mut self) {
        loop {
            if self.terminated {
                break;
            }

            let instruction = self.intcodes[self.ip];
            // read instruction, first seperate by digits and transform to vec of digits
            let instruct_digits: Vec<u8> = instruction
                .to_string()
                .chars()
                .map(|d| d.to_digit(10).unwrap() as u8)
                .collect();

            // seperate instruction into opcode and parameter mode
            let n_digits = instruct_digits.len();
            let opcode: u8;
            self.pmodes = [0, 0, 0]; // default all modes to 0 first
            if n_digits == 1 {
                // speci_al case of single digit opcode
                opcode = instruct_digits[0];
            } else {
                // last two digits are opcode
                opcode = instruct_digits[n_digits - 2] * 10 + instruct_digits[n_digits - 1];
                // remaining digits set parameter mode array, if unpresent remain 0
                for i in 0..n_digits - 2 {
                    self.pmodes[i] = instruct_digits[n_digits - 3 - i];
                }
            }

            self.debug_print(format!("{:04} {} {:?} | ", self.ip, opcode, self.pmodes));

            // read opcode and execute command accordingly
            match opcode {
                1 => {
                    self.debug_print(String::from("ADD"));
                    self.add();
                }
                2 => {
                    self.debug_print(String::from("MUL"));
                    self.multiply();
                }
                3 => {
                    self.debug_print(String::from("INP"));
                    // input opcode
                    if self.memory_input.is_none() {
                        if self.show_stdinout {
                            println!("\nProgram halted, requesting for input.");
                        }
                        break;
                    } else {
                        self.input();
                    }
                }
                4 => {
                    self.debug_print(String::from("OUT"));
                    self.output();
                    // make a break to be possibly dealing with the new output
                    break;
                }
                5 => {
                    self.debug_print(String::from("JIT"));
                    self.jump_if_true();
                }
                6 => {
                    self.debug_print(String::from("JIF"));
                    self.jump_if_false();
                }
                7 => {
                    self.debug_print(String::from("LET"));
                    self.less_than();
                }
                8 => {
                    self.debug_print(String::from("EQU"));
                    self.equals();
                }
                9 => {
                    self.debug_print(String::from("RBO"));
                    self.relative_base_offset();
                }
                99 => {
                    self.debug_print(String::from("TER"));
                    if self.show_stdinout {
                        println!("\nProgram terminated");
                    }
                    self.terminated = true;
                }
                _ => {
                    panic!(
                        "Error while reading intcodes. Unknown opcode {} at \
                         position: {}",
                        self.intcodes[self.ip], self.ip
                    );
                }
            }
            self.debug_print(String::from("\n"));
        }
    }

    /// Set the memory at input, that is read in upon calling of input opcode
    pub fn set_memory_input(&mut self, input: i64) {
        self.memory_input = Some(input);
    }

    /// Get value depending on parameter mode
    /// where modes are:
    /// 0 -- position mode (parameter is address)
    /// 1 -- immediate mode (parameter is value itself)
    /// 2 -- relative mode (parameter is address but from relative base)
    fn get_value(&self, idx_parameter: usize) -> i64 {
        let parameter = self.intcodes[self.ip + idx_parameter + 1];
        let mode = self.pmodes[idx_parameter];
        match mode {
            0 => {
                if parameter < 0 {
                    panic!("Tried to access negative memory.");
                }
                self.intcodes[parameter as usize]
            }
            1 => parameter,
            2 => {
                let shifted_address = self.relative_base + parameter;
                if shifted_address < 0 {
                    panic!("Tried to access negative memory.");
                }
                self.intcodes[shifted_address as usize]
            }
            _ => {
                panic!("Passed unknown parameter mode: {}", mode);
            }
        }
    }

    /// Set value depending on parameter mode
    /// where allowed modes are
    /// 0 -- position mode (parameter is address)
    /// 2 -- relative mode (parameter is address from relative base)
    fn set_value(&mut self, idx_parameter: usize, value: i64) {
        let param: i64;
        match self.pmodes[idx_parameter] {
            0 => {
                param = self.intcodes[self.ip + idx_parameter + 1];
            }
            2 => {
                param = self.intcodes[self.ip + idx_parameter + 1] + self.relative_base;
            }
            _ => panic!("Invalid mode for input."),
        }

        self.debug_print(format!(" {}", param));
        if param < 0 {
            panic!("Tried to input to negative memory.");
        }
        self.intcodes[param as usize] = value;
    }

    /// Add first and second parameter and store to third
    fn add(&mut self) {
        // get first two parameter values
        let p1 = self.get_value(0);
        let p2 = self.get_value(1);
        self.debug_print(format!(" {} {}", p1, p2));
        // store result at third position
        self.set_value(2, p1 + p2);
        self.ip += 4;
    }

    /// Multiply first and second parameter and store to third
    fn multiply(&mut self) {
        // get first two parameter values
        let p1 = self.get_value(0);
        let p2 = self.get_value(1);
        self.debug_print(format!(" {} {}", p1, p2));

        // store result at third position
        self.set_value(2, p1 * p2);
        self.ip += 4;
    }

    /// Read input in the input memory to position at parameter
    fn input(&mut self) {
        let input_value = self.memory_input.unwrap();
        self.set_value(0, input_value);
        if self.show_stdinout {
            println!("\n<< {}", input_value);
        }
        self.memory_input = None;
        self.ip += 2;
    }

    /// Set value at parameter into the output memory
    fn output(&mut self) {
        let p1 = self.get_value(0);
        self.debug_print(format!(" {}", p1));
        self.memory_output = p1;
        if self.show_stdinout {
            println!("\n>> {}", p1);
        }
        self.ip += 2;
    }

    /// If first parameter is != 0, jump instruction pointer to second parameter
    /// Otherwise just move instruction pointer forward
    fn jump_if_true(&mut self) {
        // jump-if-true
        let p1 = self.get_value(0);
        let p2 = self.get_value(1);
        self.debug_print(format!(" {} {}", p1, p2));
        if p1 != 0 {
            if p2 < 0 {
                panic!("Trying to jump to negative memory");
            }
            self.ip = p2 as usize;
        } else {
            self.ip += 3;
        }
    }

    /// Same as jump_if_true() but jumps if parameter is zero
    fn jump_if_false(&mut self) {
        let p1 = self.get_value(0);
        let p2 = self.get_value(1);
        self.debug_print(format!(" {} {}", p1, p2));
        if p1 == 0 {
            if p2 < 0 {
                panic!("Trying to jump to negative memory");
            }
            self.ip = p2 as usize;
        } else {
            self.ip += 3;
        }
    }

    /// Compare p1 < p2. If true, store 1 in parameter, else store 0
    fn less_than(&mut self) {
        // get first two parameter values
        let p1 = self.get_value(0);
        let p2 = self.get_value(1);
        self.debug_print(format!(" {} {}", p1, p2));

        if p1 < p2 {
            self.set_value(2, 1);
        } else {
            self.set_value(2, 0);
        }
        self.ip += 4;
    }

    /// Compare p1 == p2. If true, store 1 in parameter, else store 0
    fn equals(&mut self) {
        // get first two parameter values
        let p1 = self.get_value(0);
        let p2 = self.get_value(1);
        self.debug_print(format!(" {} {}", p1, p2));

        if p1 == p2 {
            self.set_value(2, 1);
        } else {
            self.set_value(2, 0);
        }
        self.ip += 4;
    }

    /// Shift relative base by the first parameter
    fn relative_base_offset(&mut self) {
        let p1 = self.get_value(0);
        self.debug_print(format!(" {}", p1));
        self.relative_base += p1;
        self.ip += 2;
    }
}
