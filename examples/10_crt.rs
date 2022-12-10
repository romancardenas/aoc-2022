use std::fs::File;
use std::io::{prelude::*, BufReader};

/// Reads the input file and returns a vector with the changes in x for each cycle.
fn read_input(path: &str) -> Vec<i32> {
    let mut res = Vec::new();
    let file = File::open(path).expect("input file not found");
    let reader = BufReader::new(file);
    let lines = reader.lines();

    for line in lines {
        res.push(0); // this represents a noop operation or first cycle of an addx operation
        let line = line.expect("error parsing line");
        let chars: Vec<&str> = line.split_whitespace().collect();
        if chars[0] == "addx" {
            // For addx operations, we added previously a 0 and then the value of the instruction!
            res.push(chars[1].parse().expect("error parsing number"));
        }
    }
    res
}

/// From a set of operations, it returns a vector with the history of the values of the x register.
/// The value of x AT THE END of the ith cycle is the ith index of the resulting vector.
fn compute_x(ops: &[i32]) -> Vec<i32> {
    let mut x = 1; // initially, x is set to 1
    let mut x_history = vec![x]; // we add the initial value of x to its history
    for op in ops {
        x += *op;
        x_history.push(x);
    }
    x_history
}

/// It computes the signal strength as requested in the first exercise.
fn exercise_1(x_history: &[i32]) -> i32 {
    let mut res = 0;
    for i in (20..220 + 1).step_by(40) {
        res += i as i32 * x_history[i - 1];
    }
    res
}

/// It represents the display from the values of x.
fn exercise_2(x_history: &[i32]) {
    // We iterate over the 240 cycles required to print the display.
    for cycle in 1..240 + 1 {
        let sprite = x_history[cycle - 1]; // center of the sprite in the current cycle
        let column = (cycle as i32 - 1) % 40; // column index in the current cycle
        if sprite >= column - 1 && sprite <= column + 1 {
            // If the column is in the same location the pixels of the sprite, we print a '#'
            print!("#");
        } else {
            // Otherwise, we print a ' ' (the output is prettier than using '.', IMO)
            print!(" ");
        }
        if column == 40 - 1 {
            // If we reach the last column, we print a new line
            println!();
        }
    }
}

fn main() {
    // First we read the input file.
    let cycles = read_input("data/10_input.txt");
    let x_history = compute_x(&cycles);
    println!("{:?}", exercise_1(&x_history));
    exercise_2(&x_history);
}
