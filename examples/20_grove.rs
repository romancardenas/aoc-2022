use std::collections::VecDeque;
use std::fs::File;
use std::io::{prelude::*, BufReader};

/// Reads the input file and returns the list of blueprints.
fn read_input(path: &str) -> Vec<i64> {
    let file = File::open(path).expect("input file not found");
    let reader = BufReader::new(file);
    let lines = reader.lines();
    lines
        .map(|l| l.expect("unable to parse line").parse::<i64>().unwrap())
        .collect()
}

/// It decodes the input message
fn decode(code: &[i64], hash: i64, n_mixes: usize) -> i64 {
    // The deque stores tuples (original index, hashed value)
    let mut x: VecDeque<_> = code
        .iter()
        .enumerate()
        .map(|(i, &n)| (i, n * hash))
        .collect();
    // We apply the decoding function n_mixes times
    for _ in 0..n_mixes {
        // i keeps track of which original position we are processing right now
        for i in 0..x.len() {
            // First, we make sure that the front of the deque contains the position under study
            while x.front().unwrap().0 != i {
                let front = x.pop_front().unwrap();
                x.push_back(front)
            }
            // val is a tuple (i, value in the ith position)
            let val = x.pop_front().unwrap();
            let to_pop = val.1 % x.len() as i64;
            if to_pop >= 0 {
                // If the number is positive, we move elements from front to back
                for _ in 0..to_pop {
                    let front = x.pop_front().unwrap();
                    x.push_back(front);
                }
            } else {
                // If the number is negative, we move elements from back to front
                for _ in to_pop..0 {
                    let back = x.pop_back().unwrap();
                    x.push_front(back);
                }
            }
            // Once we are done, we push the current value to the back of the deque
            x.push_back(val);
        }
    }
    // Next, we find the index of the 0 element
    let i0 = x.iter().position(|&(_, x)| x == 0).unwrap();
    // Then, we resolve the indices of the coordinates
    let (i1, i2, i3) = (
        (i0 + 1000) % x.len(),
        (i0 + 2000) % x.len(),
        (i0 + 3000) % x.len(),
    );
    // and return the sum of the corresponding values
    let (n1, n2, n3) = (x[i1].1, x[i2].1, x[i3].1);
    n1 + n2 + n3
}

fn main() {
    let code = read_input("data/20_input.txt");
    // println!("{:?}", &code);
    println!("{}", decode(&code, 1, 1));
    println!("{}", decode(&code, 811589153, 10));
}
