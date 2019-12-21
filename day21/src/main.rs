use std::fs;
use std::io::Error;

mod computer;
mod springdroid;
use springdroid::SpringDroid;

fn read_input() -> Result<Vec<i64>, Error> {
    let content = fs::read_to_string("./input")?;
    let digits: Vec<i64> = content
        .split(',')
        .map(|d| d.parse::<i64>().unwrap())
        .collect();

    Ok(digits)
}

fn part1(intcodes: Vec<i64>) {
    println!("Part 1");
    let mut springdroid = SpringDroid::new(intcodes);
    // a jump takes you 4 fields further
    springdroid.run();
    //case 1: 1 field in front is a hole: J = !1
    springdroid.input_ascii("NOT A J"); // jump if the next field is a hole 

    //case 2: 3 fields in front is a hole and the fourth field is free to jump to
    // T = !3 && 4
    springdroid.input_ascii("NOT C T"); // jump if 3 fields in front is hole
    springdroid.input_ascii("AND D T"); // and the fourth field is not a hole

    // J = J || T
    springdroid.input_ascii("OR T J"); // jump in case 1 and 2
    springdroid.input_ascii("WALK");
}

fn part2(intcodes: Vec<i64>) {
    println!("Part 2");
    let mut springdroid = SpringDroid::new(intcodes);
    springdroid.run();
    // J, T are initialised as false
    // check if 3 is hole and 4 is free field: !3 && 4
    springdroid.input_ascii("NOT C J");
    springdroid.input_ascii("AND D J");

    // check if both 5 and 8 are holes: !5 && !8, in this case set T to false, otherwise its true
    // !(!5 && !8) is the same as (5||f) || 8
    springdroid.input_ascii("OR E T"); //  
    springdroid.input_ascii("OR H T"); 
 
    // if 3 is hole & 4 is free, J is true, however if 5 & 8 are holes J should become false
    // J = (!3 && 4) && !(!5 && !8)
    springdroid.input_ascii("AND T J");

    // set T to same value as J
    springdroid.input_ascii("NOT J T");
    springdroid.input_ascii("NOT T T");

    // if J is true, it should remain true (only apply OR to J) -> field in range of 3 
    // causes jump
    // if jump is false, check if fields in range 2 can cause a jump
    // in this case T is false (as T = J)
    // jump if both 2 and 5 are holes, as otherwise next step 1 & 4 are holes -> dead
    // T = !2 && !5 = !( (2||f) || 5)
    springdroid.input_ascii("OR B T");
    springdroid.input_ascii("OR E T");
    springdroid.input_ascii("NOT T T");

    // if J was true, it's still true. if it was false, it's true now if 2 and 5 are holes
    springdroid.input_ascii("OR T J"); 
    
    // last check: if 1 is a hole: jump
    springdroid.input_ascii("NOT A T");
    springdroid.input_ascii("OR T J");

    springdroid.input_ascii("RUN");

}

fn main() {
    let intcodes = read_input().unwrap();
    part1(intcodes.clone());
    part2(intcodes);
}
