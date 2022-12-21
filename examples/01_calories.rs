use std::fs::File;
use std::io::{prelude::*, BufReader};

/// Reads the input file and returns a vector of vectors.
/// Each element of the outer vector corresponds to one elf.
/// Inner vectors contain the calories of the food obtained by the elves.
fn read_input(path: &str) -> Vec<Vec<usize>> {
    let mut res = Vec::new();
    // If ready is false, we need to add a new vector (i.e., a new elf) to res.
    let mut ready = false;

    let file = File::open(path).expect("input file not found");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(calories) = line.expect("something went wrong").parse::<usize>() {
            // First, we check if we need to add a new vector to res.
            if !ready {
                res.push(Vec::new());
                ready = true;
            }
            // We push the new food to the last elf.
            res.last_mut().expect("something went wrong").push(calories);
        } else {
            // If blank line, we set ready to false and get ready to add a new vector to res
            ready = false;
        }
    }
    res
}

/// It returns a vector with the sum of the calories of all the food obtained by the elves.
fn sum_calories(elves: &[Vec<usize>]) -> Vec<usize> {
    let mut res = Vec::new();
    for food in elves {
        res.push(food.iter().sum());
    }
    res
}

/// It sums the calories obtained by the n first elves. YOU MUST SORT THE ELVES BEFORE.
/// If n is greater than the number of elves, it returns the sum of the whole vector.
fn sum_first_elves(elves: &[usize], n: usize) -> usize {
    let mut res = 0;
    for i in 0..n {
        if let Some(x) = elves.get(i) {
            res += x;
        } else {
            break;
        }
    }
    res
}

fn main() {
    // First we read the input file and store it as closer as possible to the input format.
    let elves_food = read_input("data/01_input.txt");
    println!("{:?}", elves_food);
    // Then, we sum the calories obtained by every elf.
    let mut elves_calories = sum_calories(&elves_food);
    println!("{:?}", elves_calories);
    // Next, we sort the vector in descending order
    elves_calories.sort_by(|a, b| b.cmp(a));
    println!("{:?}", elves_calories);
    // Finally, we sum the callories obtained by the best three elves
    println!("{:?}", sum_first_elves(&elves_calories, 3))
}
