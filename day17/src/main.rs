use std::char;
use std::fs;
use std::io::Error;
use std::cmp;

mod computer;
mod vacuum_robot;
use vacuum_robot::VacuumRobot;

fn read_input() -> Result<Vec<i64>, Error> {
    let content = fs::read_to_string("./input")?;
    let digits: Vec<i64> = content
        .split(',')
        .map(|d| d.parse::<i64>().unwrap())
        .collect();

    Ok(digits)
}

fn print_map(map: &Vec<Vec<char>>) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            print!("{}", map[i][j]);
        }
        println!();
    }
}

fn part1(intcodes: Vec<i64>) {
    println!("PART 1");
    let mut robot = VacuumRobot::new(intcodes);

    // calc alignment parameters: scaffold ('#') intersections
    let mut alignment_parameter_sum = 0;
    for i in 0..robot.n_rows {
        for j in 0..robot.n_cols {
            let symbol = robot.map[i][j];

            if symbol == '#' {
                // check if scaffold intersection
                // only check those that are not at the edge
                if i > 0
                    && i < robot.n_rows - 1
                    && j > 0
                    && j < robot.n_cols - 1
                    && robot.map[i - 1][j] == '#'
                    && robot.map[i + 1][j] == '#'
                    && robot.map[i][j - 1] == '#'
                    && robot.map[i][j + 1] == '#'
                {
                    alignment_parameter_sum += i * j;
                    print!("{}", 'O');
                } else {
                    print!("{}", symbol);
                }
            } else {
                print!("{}", symbol);
            }
        }
        println!();
    }

    println!("Alignment parameter sum: {}", alignment_parameter_sum);
}

fn find_next_occurence_of_unit(vec: &Vec<String>, unit: &Vec<String>) -> Option<usize> {
    let mut idx_repeats = 0;
    'outer_loop: loop {
        // check every element of unit
        for (i, el) in unit.iter().enumerate() {
            if idx_repeats + i >= vec.len() {
                // reached out of bounds check -> unit is not in vec
                return None;
            } else if el != &vec[idx_repeats + i] {
                // check if the following elements from idx_repeats
                // correspond to elements in unit... if not, start again
                // from next start_idx in vec
                idx_repeats += 1;
                continue 'outer_loop;
            }
        }
        // if reached here, all elements of unit were found in vec starting from
        // idx_repeats
        return Some(idx_repeats);
    }
}

fn part2(mut intcodes: Vec<i64>) {
    println!("PART 2");
    intcodes[0] = 2;
    let mut robot = VacuumRobot::new(intcodes);

    // given the map, determine chain of movements to get to the end
    let mut cleaning_chain: Vec<String> = Vec::new();
    robot.turn_right(); // initially robot should turn right
    cleaning_chain.push(String::from("R"));
    let mut num_steps = 0;
    loop {
        // println!("{:?}", num_steps);
        // check forward direction
        let forward_pos = robot.get_adjacent_position(&robot.direction);
        if robot.position_in_map(&forward_pos)
            && robot.map[forward_pos.0 as usize][forward_pos.1 as usize] == '#'
        {
            num_steps += 1;
            robot.move_forward();
            continue;
        }
        // no longer goes forward
        // push counted steps for forward, and reset number
        cleaning_chain.push(num_steps.to_string());
        num_steps = 0;

        // check whether to turn left or right now
        let left_pos = robot.get_adjacent_position(&robot.direction.turn_left());
        if robot.position_in_map(&left_pos)
            && robot.map[left_pos.0 as usize][left_pos.1 as usize] == '#'
        {
            cleaning_chain.push(String::from("L"));
            robot.turn_left();
            continue;
        }
        let right_pos = robot.get_adjacent_position(&robot.direction.turn_right());
        if robot.position_in_map(&right_pos)
            && robot.map[right_pos.0 as usize][right_pos.1 as usize] == '#'
        {
            cleaning_chain.push(String::from("R"));
            robot.turn_right();
            continue;
        }
        // neither forward, left nor right helps -> end reached
        break;
    }

    // following algo only searches for solutions of type ABAC...
    // suffices here
    let mut unit_a: Vec<String> = Vec::new();
    let mut unit_b: Vec<String>;
    let mut unit_c: Vec<String>;
    let mut unit_pattern: Vec<String>;
    'a_loop: loop {
        unit_pattern = vec![String::from("A")];
        // keep extending a, while removing from single_command_chain
        unit_a.push(cleaning_chain.remove(0));
        // determine B by finding the next repetition of A

        let mut consume_remaining_chain = cleaning_chain.clone();
        let mut idx_a_repeats = find_next_occurence_of_unit(&consume_remaining_chain, &unit_a).unwrap();
        unit_b = consume_remaining_chain[..idx_a_repeats].to_owned();
        
        // determined structure ABA so far
        consume_remaining_chain = consume_remaining_chain[unit_b.len()+unit_a.len()..].to_owned();
        unit_pattern.push(String::from("B"));
        unit_pattern.push(String::from("A"));


        idx_a_repeats = find_next_occurence_of_unit(&consume_remaining_chain, &unit_a).unwrap();
        let idx_b_repeats = find_next_occurence_of_unit(&consume_remaining_chain, &unit_b).unwrap();
        // by definition of unit_b the structure is ABA..,
        // now need to differentiate ABAA, ABAB and ABAC
        if idx_a_repeats > 0 && idx_b_repeats > 0 {
            // structure is ABAC
            // next element is C, going up to the minimum of a & b
            let idx_minimum = cmp::min(idx_a_repeats, idx_b_repeats);
            unit_c = consume_remaining_chain[..idx_minimum].to_owned();
            consume_remaining_chain = consume_remaining_chain[idx_minimum..].to_owned();
            unit_pattern.push(String::from("C"));
        } else {
            // searches for structure ABAA, ABAB not implemented
            continue 'a_loop;
        }

        // check if remaining chain can be consumed with defined A,B,C units
        'loop_consume: loop {
            let test_a = find_next_occurence_of_unit(&consume_remaining_chain, &unit_a);
            if test_a.is_some() && test_a.unwrap() == 0 {
                consume_remaining_chain = consume_remaining_chain[unit_a.len()..].to_owned();
                unit_pattern.push(String::from("A"));
                continue 'loop_consume;
            }
            let test_b = find_next_occurence_of_unit(&consume_remaining_chain, &unit_b);
            if test_b.is_some() && test_b.unwrap() == 0 {
                consume_remaining_chain = consume_remaining_chain[unit_b.len()..].to_owned();
                unit_pattern.push(String::from("B"));
                continue 'loop_consume;
            }
            let test_c = find_next_occurence_of_unit(&consume_remaining_chain, &unit_c);
            if test_c.is_some() && test_c.unwrap() == 0 {
                consume_remaining_chain = consume_remaining_chain[unit_c.len()..].to_owned();
                unit_pattern.push(String::from("C"));
                continue 'loop_consume;
            }
            break;
        }
        if consume_remaining_chain.len() == 0 {
            break;
        }
    }

    // chain of commands determined, now feed into robot
    // robot.computer.show_stdinout = true;
    robot.run();
    robot.run_chain_of_inputs(unit_pattern);
    robot.run();
    robot.run_chain_of_inputs(unit_a);
    robot.run();
    robot.run_chain_of_inputs(unit_b);
    robot.run();
    robot.run_chain_of_inputs(unit_c);
    robot.run();
    robot.run_chain_of_inputs(vec!["n".to_string()]);
    robot.run();
    println!("Collected dust: {}", robot.computer.memory_output);
}

fn main() {
    let intcodes = read_input().unwrap();
    part1(intcodes.clone());
    part2(intcodes);
}
