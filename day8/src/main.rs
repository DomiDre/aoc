use std::fs;
use std::io::{Error};

fn read_input() -> Result<Vec<u32>, Error> {
    let content = fs::read_to_string("./input")?;
    let digits: Vec<u32> = content.to_string().chars()
    .map(|d| d.to_digit(10).unwrap())
    .collect();
    Ok(digits)
}

fn part1() {
    let digits: Vec<u32> = read_input().unwrap();

    let image_width = 25;
    let image_height = 6;
    let pts_in_layers = image_height * image_width;
    let num_layers = digits.len() / pts_in_layers;
    
    let mut idx_fewest_0 = 0;
    let mut num_fewest_0 = pts_in_layers;
    
    for i in 0..num_layers {
        let layer = &digits[pts_in_layers*i..pts_in_layers*(i+1)];
        let mut num_zeros = 0;
        for num in layer {
            if *num == 0 {
                num_zeros += 1;
            }
        }
        if num_zeros < num_fewest_0 {
            idx_fewest_0 = i;
            num_fewest_0 = num_zeros;
        }
    }

    let mut num_ones = 0;
    let mut num_twos = 0;

    for num in &digits[pts_in_layers*idx_fewest_0..pts_in_layers*(idx_fewest_0+1)] {
        match num {
            1 => {
                num_ones += 1;
            },
            2 => {
                num_twos += 1;
            },
            _ => continue
        }
    }
    println!("{}", num_ones*num_twos);
}


fn part2() {
    let digits: Vec<u32> = read_input().unwrap();
    let image_width = 25;
    let image_height = 6;
    let pts_in_layers = image_height * image_width;
    let num_layers = digits.len() / pts_in_layers;
    
    let mut summed_layer = vec![2; pts_in_layers];
    

    for ix in 0..image_width {
        for iy in 0..image_height {
            for il in 0..num_layers {
                let value = digits[il*pts_in_layers + iy*image_width + ix];
                match value {
                    0 => {
                        summed_layer[iy*image_width + ix] = 0;
                        break;
                    },
                    1 => {
                        summed_layer[iy*image_width + ix] = 1;
                        break;
                    },
                    2 => {
                        continue;
                    },
                    _ => {
                        panic!("INVALID PIXEL");
                    }
                }
            }
        }
    }

    for iy in 0..image_height {
        for ix in 0..image_width {
            print!("{}", summed_layer[iy*image_width + ix]);
        }
        println!("");
    }
}

fn main() {
    // part1();
    part2();
}