
/// Find password which is a 6 digit number, within the range of the input
/// further more it's known that two adjacents digits in it are the same
/// and from left to right the digits never decrease
fn part1() {
    let puzzle_range = 134792..675810;
    let mut possible_passwords = Vec::<u32>::new();
    
    'candidate_for: for candidate in puzzle_range {

        // transform number to Vec of digits
        let digits: Vec<u32> = candidate
        .to_string().chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();
        
        // check if there is adjacent pair of same digits
        // & check if digits are descending
        let mut adjacent_digits_present = false;
        let mut previous_digit: &u32 = &digits[0];
        for digit in digits.iter().skip(1) {
            if digit < &previous_digit {
                continue 'candidate_for // decreasing digit -> invalid candidate
            }
            if digit == previous_digit {
                adjacent_digits_present = true;
            }
            previous_digit = digit;
        }

        if !adjacent_digits_present {
            continue
        }
        possible_passwords.push(candidate);       
    }

    println!("{:?}", possible_passwords.len());
}

/// Additionally check for adjacent digits if next digit is different
fn part2() {
    let puzzle_range = 134792..675810;
    let mut possible_passwords = Vec::<u32>::new();
    
    'candidate_for: for candidate in puzzle_range {

        // transform number to Vec of digits
        let digits: Vec<u32> = candidate
        .to_string().chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();
        
        let mut valid_candidate = false;
        
        // check if digits are descending -> skip
        // check if there is adjacent pair of same digits that is not pair of
        // a larger group -> candidate
        for (i, digit) in digits.iter().enumerate().skip(1) {
            
            let previous_digit: &u32 = &digits[i-1];
            if digit < previous_digit {
                // decreasing digit -> invalid candidate
                continue 'candidate_for;
            }

            // check if current candidate is not already deemed valid
            // then if current digit same as the previous
            // and if yes, if next digit is either the end or a different digit
            // if also yes, if the digit before the previous digit is either
            // the start or a different digit
            // if all is valid -> adjacent pair present -> valid candidate
            // still keep on checking digit though in case descending digit appears
            if (!valid_candidate) &&
               (digit == previous_digit) &&
               ((i == digits.len() -1) || (digit != &digits[i+1])) &&
               ((i == 1) || (digit != &digits[i-2]))
               {
                valid_candidate = true;
            }
        }
        if valid_candidate {
            possible_passwords.push(candidate);
        }
    }
    println!("{:?}", possible_passwords.len());
}

fn main() {
    //part1();
    part2();
}
