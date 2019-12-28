use std::fs;
use std::io::{Error, stdin};

mod computer;
mod ascii_computer;
use ascii_computer::AsciiComputer;

fn read_input() -> Result<Vec<i64>, Error> {
    let content = fs::read_to_string("./input")?;
    let digits: Vec<i64> = content
        .split(',')
        .map(|d| d.parse::<i64>().unwrap())
        .collect();

    Ok(digits)
}

fn get_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input)
    .expect("Error occured trying to read user input.");
    input
         
}

fn part1(intcodes: Vec<i64>) {
    println!("Part 1");
    let mut computer = AsciiComputer::new(intcodes);
    computer.run();
    let start_commands = 
    [
        "east",
        "take food ration",
        "east",
        "take manifold",
        "east",
        "east",
        "take jam",
        "west",
        "north",
        "north",
        "take fuel cell",
        "south",
        "east",
        "take spool of cat6",
        "west",
        "south",
        "west",
        "west",
        "south",
        "take prime number",
        "north",
        "west", // back at start now
        "north",
        "north",
        "north",
        "east",
        "east",
        "take loom",
        "west",
        "west",
        "south",
        "west",
        "take mug",
        "east",
        "south",
        "west",
        "north",
        "west",
        "drop spool of cat6"// in front of pressure sensitive room now
    ];
    for &command in start_commands.iter() {
        computer.enter_command(command);
    }
    let mut items = [
        "jam",
        "loom",
        "mug",
        "prime number",
        "food ration",
        "fuel cell",
        "manifold"
    ];
    // cat6 spool too heavy if tested alone -> excluded
    // 7 binary options -> 2^7 = 128 possibilites
    for i in 0..128 {
        // drop all items 
        for &item in items.iter() {
            computer.enter_command(
                ("drop ".to_string() +
                 &item.to_string()).as_ref());
        }
        for n in 0..7 {
            let take_item = i & (1 << n) != 0;
            if take_item {
                computer.enter_command(
                    ("take ".to_string() +
                     &items[n].to_string()).as_ref());
            }
        }
        computer.enter_command("north");
    }

// solution:
// You take the mug.
// You take the prime number.
// You take the food ration.
// You take the fuel cell.


}

fn part2(intcodes: Vec<i64>) {
    println!("Part 2");
    // let mut computer = AsciiComputer::new(intcodes);
    // computer.run();

}

fn main() {
    let intcodes = read_input().unwrap();
    part1(intcodes.clone());
    // part2(intcodes);
}
