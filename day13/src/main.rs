use std::fs;
use std::io::{Error};
use std::{thread, time};
use ncurses;

mod computer;
mod arcade;
use arcade::{Arcade, Tile, JoystickStates};

fn read_input() -> Result<Vec<i64>, Error> {
    let content = fs::read_to_string("./input")?;
    let digits: Vec<i64> = content.split(',')
    .map(|d| d.parse::<i64>().unwrap())
    .collect();

    Ok(digits)
}

fn part1(intcodes: Vec<i64>) {
    println!("PART 1");
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

    // init arcade and run to initialize screen
    intcodes[0] = 2;
    let mut arcade = Arcade::new(intcodes);
    arcade.run();

    ncurses::initscr();
    ncurses::raw();
    ncurses::keypad(ncurses::stdscr(), true);
    ncurses::noecho();

    // draw screen
    ncurses::addstr(format!("Score: {}\n\n", arcade.score).as_ref());
    ncurses::addstr(arcade.get_screen().as_ref());
    ncurses::refresh();
    ncurses::halfdelay(2);
    loop {
        if arcade.ball_pos.0 < arcade.horizontal_pos.0 {
            arcade.joystick_input(JoystickStates::LeftTilt);
        } else if arcade.ball_pos.0 > arcade.horizontal_pos.0 {
            arcade.joystick_input(JoystickStates::RightTilt);
        } else {
            arcade.joystick_input(JoystickStates::Neutral);
        }

        // continue and update screen
        arcade.run();
        ncurses::clear();
        ncurses::addstr(format!("Score: {}\n\n", arcade.score).as_ref());
        ncurses::addstr(arcade.get_screen().as_ref());
        ncurses::refresh();
        
        if arcade.computer.terminated {
            break;
        }
        thread::sleep(time::Duration::from_millis(10));
    }
    
    /* Wait for one more character before exiting. */
    ncurses::getch();
    ncurses::endwin();
}

fn main() {
    let intcodes = read_input().unwrap();
    part1(intcodes.clone());
    part2(intcodes);
}
