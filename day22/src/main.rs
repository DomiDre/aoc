use std::fs;
use std::io::{BufRead, BufReader, Error};

#[derive(Debug)]
enum Instruction {
    DealNewStack,
    Cut(i64),
    DealWithIncrement(usize),
}

const N_CARDS: usize = 10007;
type CardDeck = [usize; N_CARDS];

fn read_input(file_path: &str) -> Result<Vec<Instruction>, Error> {
    let file = fs::File::open(file_path)?;
    let buffer = BufReader::new(file);
    let mut instructions = Vec::new();
    for line in buffer.lines() {
        if let Ok(instr) = line {
            if instr.starts_with("cut") {
                let number = instr["cut".len()..].trim().parse::<i64>().unwrap();
                instructions.push(Instruction::Cut(number))
            } else if instr.starts_with("deal with increment") {
                let number = instr["deal with increment".len()..]
                    .trim()
                    .parse::<usize>()
                    .unwrap();
                instructions.push(Instruction::DealWithIncrement(number))
            } else if instr.starts_with("deal into new stack") {
                instructions.push(Instruction::DealNewStack);
            } else {
                panic!("Unknown command: {}", instr);
            }
        }
    }
    Ok(instructions)
}

fn part1() {
    println!("Part 1");
    let instructions = read_input("./input").unwrap();

    let mut card_deck: CardDeck = [0; N_CARDS];
    for i in 0..N_CARDS {
        card_deck[i] = i;
    }
    for instr in instructions.iter() {
        match instr {
            Instruction::DealNewStack => {
                let mut new_stack: CardDeck = [0; N_CARDS];
                for i in 0..N_CARDS {
                    new_stack[N_CARDS - 1 - i] = card_deck[i];
                }
                card_deck = new_stack;
            }
            Instruction::DealWithIncrement(n) => {
                let mut new_stack: CardDeck = [0; N_CARDS];
                for i in 0..N_CARDS {
                    new_stack[n * i % N_CARDS] = card_deck[i];
                }
                card_deck = new_stack;
            }
            Instruction::Cut(n) => {
                if *n >= 0 {
                    card_deck.rotate_left(*n as usize);
                } else {
                    card_deck.rotate_right((*n * (-1)) as usize);
                }
            }
        }
    }
    println!(
        "Position of card 2019: {:?}",
        card_deck.iter().position(|&el| el == 2019).unwrap()
    );
}

/// all instructions can be represented as equation f(x) = a*x + b % m
/// combine instructions into single instruction
fn combine_commands_as_linear_eq(instructions: Vec<Instruction>, n_cards: i64) -> (i64, i64) {
    let mut a: i64 = 1;
    let mut b: i64 = 0;
    for instr in instructions.iter() {
        match instr {
            Instruction::DealNewStack => {
                a *= -1;
                b = n_cards - 1 - b;
            }
            Instruction::DealWithIncrement(n) => {
                a *= *n as i64;
                b *= *n as i64;
            }
            Instruction::Cut(n) => {
                if *n >= 0 {
                    b += n_cards - n;
                } else {
                    b += *n * (-1)
                }
            }
        }
        a %= n_cards;
        b %= n_cards;
    }
    (a, b)
}

/// calculate a*b % m
fn multiply_large_with_modulo(a: i64, b: i64, m: i64) -> i64 {
    let mut result = 0;
    let mut a = a % m;
    let mut b = b;
    loop {
        if b == 0 {
            break;
        }
        if b & 1 == 1 {
            // b is odd
            result = (result + a) % m;
        }
        a = (2 * a) % m;
        b /= 2;
    }
    result
}

/// calculates gcd(a,b), s and t such that gcd(a,b) = s*a + t*b
fn extended_euclid(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (dp, sp, tp) = extended_euclid(b, a % b);
    let (d, s, t) = (dp, tp, sp - (a as i64 / b as i64) * tp);
    (d, s, t)
}

fn inverse_modulo(x: i64, m: i64) -> i64 {
    // calculates inverse modulo of x % m ( 1 = x*inv % m )
    let (gcd, s, _) = extended_euclid(x, m as i64);
    if gcd != 1 {
        panic!("Couldn't find inverse modulo of {} and {}", x, m);
    }
    if s < 0 {
        (m as i64 + s)
    } else {
        s
    }
}

// perform exponentation a^n % m
fn modular_exponentiation(a: i64, n: u64, m: i64) -> i64 {
    // calc a * a = a^2, a^2 * a^2 = a^4....
    let mut intermediate_steps = vec![a % m];
    let mut pow = 2;
    while pow < n {
        let last_power = *intermediate_steps.last().unwrap();
        intermediate_steps.push(multiply_large_with_modulo(last_power, last_power, m));
        pow *= 2;
    }

    let mut result = 1;
    let mut calculated_n = n;
    let mut idx = 0;
    loop {
        if calculated_n == 0 {
            // done
            break;
        }
        if (calculated_n & 1) == 1 {
            // odd power
            result = multiply_large_with_modulo(result, intermediate_steps[idx], m);
        }
        calculated_n /= 2;
        idx += 1;
    }
    result
}

fn part2() {
    println!("Part 2");
    let instructions = read_input("./input").unwrap();
    let n_cards: i64 = 119_315_717_514_047;
    let poi = 2020; // position of interest
    let n_repetitions: u64 = 101_741_582_076_661;

    // combine all instructions as single linear equation mod n_cards
    let (mut a, mut b) = combine_commands_as_linear_eq(instructions, n_cards);

    // perform transformation n_repetition times
    // performing a*x + b N times is a^N + (a^(N-1) + a^(N-2) + .. 1) * b
    let a_to_power_n_rep = modular_exponentiation(a, n_repetitions, n_cards);
    // (a^(N-1) + a^(N-2) + .. 1) is geometric series =  (a^N - 1) / (a - 1)
    let denominator_inv = inverse_modulo(a - 1, n_cards);
    let geometric_series =
        multiply_large_with_modulo(a_to_power_n_rep - 1, denominator_inv, n_cards as i64);
    b = multiply_large_with_modulo(b, geometric_series, n_cards);
    a = a_to_power_n_rep; // forgot old a, now a, b combine all instructions n_rep times

    // now solve 2020 = a*x + b % m
    // x = a_inv * (2020 - b) % m
    let a_inv = inverse_modulo(a, n_cards);
    let factor = if poi < b {
        (n_cards + (poi - b)) % n_cards
    } else {
        (poi - b) % n_cards
    };

    // We need to get the modular multiplicative inverse of new_factor
    // so we can multiply it to both sides of the equation
    let x = multiply_large_with_modulo(a_inv, factor, n_cards);
    println!("Position of card 2020: {}", x);
}

fn main() {
    part1();
    part2();
}
