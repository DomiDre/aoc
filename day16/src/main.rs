use std::fs;
use std::io::Error;

fn read_input() -> Result<Vec<i64>, Error> {
    let content = fs::read_to_string("./input")?;
    let digit_vec = content
        .chars()
        .map(|c| c.to_string().parse::<i64>().unwrap())
        .collect();
    Ok(digit_vec)
}

fn apply_pattern(digits: &Vec<i64>) -> Vec<i64> {
    let base_pattern = [0, 1, 0, -1];
    let mut result = Vec::new();
    for iteration in 1..=digits.len() {
        // generate pattern
        let mut pattern = Vec::new();
        for base_number in base_pattern.iter() {
            for _k in 0..iteration {
                pattern.push(base_number);
            }
        }
        let first_element = pattern.remove(0); // remove first element, append at the end
        pattern.push(first_element);

        let n_p = pattern.len();
        let mut digit_result = 0;
        for (i, digit) in digits.iter().enumerate() {
            digit_result += digit * pattern[i % n_p];
        }
        if digit_result >= 0 {
            result.push(digit_result % 10);
        } else {
            result.push((-1 * digit_result) % 10);
        }
    }
    result
}

fn part1(mut digits: Vec<i64>) {
    // just apply the pattern 100 times
    for _i in 0..100 {
        digits = apply_pattern(&digits);
    }
    println!("{:?}", &digits[..8]);
}

fn part2(digits: Vec<i64>) {
    let n_digits = digits.len();
    // get first seven numbers as offset
    let mut message_offset = 0;
    for i in 0..7 {
        message_offset += digits[i] * (10 as i64).pow(6 - (i as u32));
    }
    // message offset in the order of 6_000_000
    // input message has length of 650
    // 10times repated, signal message has length 6_500_000
    // pattern at this large numbers is only 0 0 ...  1 1 1, where first
    // 1 is at position of number that is being calculated
    // => looking at last numbers and only taking the sum here suffices

    let mut signal: Vec<i64> = Vec::new();
    for k in message_offset as usize..n_digits * 10000 {
        signal.push(digits[k % n_digits]);
    }
    let n_reduced = signal.len();

    for _i in 0..100 {
        let mut sum = 0;
        let mut updated_signal = Vec::new();
        // go backwards and just calculate the sum on the way
        for k in (0..n_reduced).rev() {
            let digit = signal[k];
            sum += digit;
            // updated digits are all positive, so just take last digit
            updated_signal.push(sum % 10);
        }
        // revert to get original order
        updated_signal.reverse();
        signal = updated_signal;
    }
    println!("Result: {:?}", &signal[..8]);
}

fn main() {
    let numbers = read_input().unwrap();
    part1(numbers.clone());
    part2(numbers);
}
