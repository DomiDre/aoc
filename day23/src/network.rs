use crate::computer::IntcodeComputer;
use std::collections::{VecDeque, HashSet};

#[derive(Debug, Clone)]
struct Packet {
    x: i64,
    y: i64
}

pub struct Network {
    computers: Vec<IntcodeComputer>,
    packet_queues: Vec<VecDeque<Packet>>,
    nat: Packet
}

impl Network {
    pub fn new(intcodes: Vec<i64>) -> Network {
        let mut computers = Vec::new();
        let mut packet_queues = Vec::new();

        for i in 0..50 {
            // boot up computer
            let mut next_computer = IntcodeComputer::new(intcodes.clone());
            // next_computer.show_stdinout = true;
            next_computer.run();
            // pass network address to computer
            next_computer.set_memory_input(i);
            next_computer.run();
            computers.push(next_computer);
            packet_queues.push(VecDeque::new());
        }

        Network {
            computers,
            packet_queues,
            nat: Packet { x: 0, y:0 }
        }
    }

    pub fn run(&mut self) {
        let mut nat_sent_y_vals: HashSet<i64> = HashSet::new();
        let mut idle_check = [false, false]; // check if network idle
                                             // if two times nothing happened
                                             // ( true, true ), it's idle
        'network_loop: loop {
            idle_check[0] = true; // init check 0

            for (idx, computer) in self.computers.iter_mut().enumerate() {
                loop {
                    // computer is requesting input
                    if computer.requesting_input {
                        break
                    }
                    // computer is sending a package
                    let address = computer.memory_output;
                    computer.run();
                    let x = computer.memory_output;
                    computer.run();
                    let y = computer.memory_output;
                    computer.run();
                    if address >= 0 && address < 50 { // valid address
                        self.packet_queues.get_mut(address as usize).unwrap().push_back(Packet { x, y });
                        // println!("SEND: {} -> {}: ({} {})", idx, address, x, y);
                    } else if address == 255 {
                        self.nat = Packet { x, y };
                        // println!("SEND: {} -> {}: ({} {})", idx, address, x, y);
                    } else {
                        panic!("Trying to send to unknown address: SEND: {} -> {}: ({} {})", idx, address, x, y);
                    }

                    idle_check[0] = false; // network not idle
                }
                let queue = &mut self.packet_queues[idx];
                if queue.len() == 0 {
                    computer.set_memory_input(-1);
                    computer.run();
                } else {
                    let packet = queue.pop_front().unwrap();
                    // println!("RECV: {}: ({} {})", idx, packet.x, packet.y);
                    computer.set_memory_input(packet.x);
                    computer.run();
                    computer.set_memory_input(packet.y);
                    computer.run();
                    idle_check[0] = false; // network not idle
                }
            }
            if idle_check[0] {
                 // network has once been idle
                if idle_check[1] {
                    // no sending and no receiving, two times in a row -> network is idle
                    // send nat package to 0
                    self.packet_queues.get_mut(0).unwrap().push_back(self.nat.clone());
                    if nat_sent_y_vals.len() == 0 {
                        println!("Solution Part 1: {}", self.nat.y);
                    }
                    let is_new_value = nat_sent_y_vals.insert(self.nat.y);
                    if !is_new_value {
                        println!("Solution Part 2: {}", self.nat.y);
                        break
                    }
                    // println!("SEND: NAT -> 0: ({} {})", self.nat.x, self.nat.y);
                } else {
                    // one time, set checker for second round
                    idle_check[1] = true;
                }
            } else {
                // something happened, second round checker should be also false
                idle_check[1] = false;
            }
        }
    }
}