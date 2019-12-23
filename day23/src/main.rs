mod computer;
mod network;

use std::io::Error;
use std::fs;

use network::Network;

fn read_input() -> Result<Vec<i64>, Error> {
    let content = fs::read_to_string("./input")?;
    let digits: Vec<i64> = content
        .split(',')
        .map(|d| d.parse::<i64>().unwrap())
        .collect();

    Ok(digits)
}

fn main() {
    let intcodes = read_input().unwrap();
    let mut network = Network::new(intcodes);
    network.run();
}
