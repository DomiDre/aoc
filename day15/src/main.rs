use std::fs;
use std::io::Error;
use std::{thread, time};
use std::collections::HashMap;

mod computer;
mod repair_droid;

use repair_droid::{RepairDroid, Tile};

fn read_input() -> Result<Vec<i64>, Error> {
    let content = fs::read_to_string("./input")?;
    let digits: Vec<i64> = content
        .split(',')
        .map(|d| d.parse::<i64>().unwrap())
        .collect();

    Ok(digits)
}

fn part1(intcodes: Vec<i64>) {
    println!("PART 1");
    let mut droid = RepairDroid::new(intcodes);

    ncurses::initscr();
    ncurses::raw();
    ncurses::keypad(ncurses::stdscr(), true);
    ncurses::noecho();

    loop {
        thread::sleep(time::Duration::from_millis(10));
        ncurses::clear();
        ncurses::addstr(droid.draw_map().as_ref());
        ncurses::refresh();

        droid.explore();
        if droid.found_oxygen_station {
            break;
        }
    }
    ncurses::getch();
    ncurses::endwin();
    println!(
        "Minimal path length: {}",
        droid.walkable_positions.len() - droid.deadend_path.len() + 1
    );
}

fn part2(intcodes: Vec<i64>) {
    println!("PART 2");
    let mut droid = RepairDroid::new(intcodes);
    let mut oxygen_fields: HashMap<(i64, i64), bool> = HashMap::new();

    // generate map
    loop {
        droid.explore();
        if droid.explored_whole_map {
            break;
        }
    }

    ncurses::initscr();
    ncurses::raw();
    ncurses::keypad(ncurses::stdscr(), true);
    ncurses::noecho();

    ncurses::addstr(droid.draw_map().as_ref());
    ncurses::refresh();
    
    oxygen_fields.insert(droid.oxygen_station.unwrap(), true);
    let mut time_counter = 0;

    loop {
        thread::sleep(time::Duration::from_millis(10));
        ncurses::clear();
        ncurses::addstr(droid.draw_map().as_ref());
        ncurses::refresh();

        let mut new_oxygen_positions = Vec::new();
        let mut removable_positions = Vec::new();
        for oxygen_pos in oxygen_fields.keys() {
            let check_positions = [
                (oxygen_pos.0 - 1, oxygen_pos.1),
                (oxygen_pos.0 + 1, oxygen_pos.1),
                (oxygen_pos.0, oxygen_pos.1 + 1),
                (oxygen_pos.0, oxygen_pos.1 - 1)
            ];
            
            // check if the current oxygen field can be neglected moving forward
            let mut no_more_floor_neighbours = true;
            for position in check_positions.iter() {
                if let Some(Tile::Floor) = droid.map.get(&position) {
                    // found an adjacent floor that is not oxygen filled
                    droid.map.insert(*position, Tile::OxygenFilledFloor);
                    new_oxygen_positions.push(position.to_owned());
                    // set flag false if any floor was found
                    no_more_floor_neighbours = false;
                }
            }
            if no_more_floor_neighbours {
                removable_positions.push(oxygen_pos.clone());
            }
        }
        // remove oxygens that no longer need checks
        for removable_pos in removable_positions.iter() {
            oxygen_fields.remove(removable_pos);
        }

        if new_oxygen_positions.len() == 0 {
            // no new oxygen position added -> finished
            break;
        }

        // add new fields for the next loop
        for new_oxygen_pos in new_oxygen_positions.iter() {
            oxygen_fields.insert(new_oxygen_pos.to_owned(), true);
        }
        time_counter += 1;
    }
    ncurses::getch();
    ncurses::endwin();

    println!("Filled after {} minutes", time_counter);
    
}

fn main() {
    let intcodes = read_input().unwrap();
    part1(intcodes.clone());
    part2(intcodes);
}
