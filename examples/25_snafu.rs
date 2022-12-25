use std::fs::File;
use std::io::{prelude::*, BufReader};

/// Reads the input file and returns the list of numbers.
fn read_input(path: &str) -> Vec<i64> {
    let mut res = Vec::new();
    let file = File::open(path).expect("input file not found");
    let reader = BufReader::new(file);
    for line in reader.lines().filter_map(|l| l.ok()) {
        let mut num = 0;
        for (i, char) in line.chars().rev().enumerate() {
            let base = i64::pow(5, i as u32);
            let digit = match char.to_digit(5) {
                Some(n) => n as i64,
                None => match char {
                    '=' => -2,
                    '-' => -1,
                    _ => panic!("unknown character"),
                },
            };
            num += digit * base;
        }
        res.push(num);
    }
    res
}

fn dec_to_snafu(mut num: i64) -> String {
    // First, we translate the number to a 5 base
    let mut numeric = Vec::new();
    while num != 0 {
        numeric.push(num % 5);
        num /= 5;
    }
    numeric.push(0); // We add an extra digit for convenience in case of an overflow
                     // Now it is time to modify the base to snafu
    for i in 0..numeric.len() - 1 {
        // First, we correct any potential overflow due to carries
        while numeric[i] >= 5 {
            numeric[i] -= 5;
            numeric[i + 1] += 1;
        }
        // Next, we change the digit to negative values if needed
        if numeric[i] >= 3 {
            numeric[i + 1] += 1; // We need to add 1 to the next digit!
            numeric[i] = match numeric[i] {
                3 => -2,
                4 => -1,
                _ => panic!("this should never happen"),
            };
        }
        assert!(numeric[numeric.len() - 1] < 3); // Just checking the the last digit is correct
    }
    // Finally, we iterate in reverse order to add the digit in the string
    let mut res = String::new();
    for &digit in numeric.iter().rev() {
        res.push(match digit {
            -2 => '=',
            -1 => '-',
            n => char::from_digit(n as u32, 5).unwrap(),
        })
    }
    String::from(res.trim_start_matches('0')) // We trim the starting zero just in case
}

fn main() {
    let scenario = read_input("data/25_input.txt");
    println!("{:?}", &scenario);
    let sum: i64 = scenario.iter().sum();
    println!("{}", dec_to_snafu(sum));
}
