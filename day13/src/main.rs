use std::fs;
use std::io::{Error};

mod computer;
mod arcade;
use arcade::{Arcade, Tile};

fn read_input() -> Result<Vec<i64>, Error> {
    let content = fs::read_to_string("./input")?;
    let digits: Vec<i64> = content.split(',')
    .map(|d| d.parse::<i64>().unwrap())
    .collect();

    Ok(digits)
}

fn part1(intcodes: Vec<i64>) {
    let mut arcade = Arcade::new(intcodes);
    arcade.run();
    let mut num_blocks = 0;
    for (key, val) in arcade.screen.iter() {
        if let Tile::Block  = val {
            num_blocks += 1;
        }
    }

    println!("The screen has {} blocks", num_blocks);
}

fn part2(mut intcodes: Vec<i64>) {
    intcodes[0] = 2;
    let mut arcade = Arcade::new(intcodes);
}

fn main() {
    let intcodes = read_input().unwrap();
    part1(intcodes.clone());
}
