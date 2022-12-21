use std::fs::File;
use std::io::{prelude::*, BufReader};

type Scenario = Vec<Vec<char>>;
type Move = (usize, usize, usize); // (n, from, to)

/// Reads the input file and returns the initial scenario and the moves to be done.
fn read_input(path: &str) -> (Scenario, Vec<Move>) {
    let mut scenario = Vec::new();
    let mut moves = Vec::new();

    let file = File::open(path).expect("input file not found");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // PARSE THE SCENARIO
    // First, we buffer all the text lines until we find the indices of the columns
    let mut raw_lines: Vec<String> = Vec::new();
    while let Some(Ok(row)) = lines.next() {
        for col in row.split_whitespace() {
            if col.parse::<usize>().is_ok() {
                // We found the scenario indices! Let's create the inner vectors of the scenario.
                scenario.push(Vec::new())
            } else {
                // We still are processing the crates. Let's push a copy of the row to raw_lines.
                raw_lines.push(row.clone());
                break;
            }
        }
        // If the scenario is already in progress, we break and move to the next step.
        if !scenario.is_empty() {
            break;
        }
    }
    // Now, we parse the raw lines from bottom to top and push the crates to the column when applies
    for line in raw_lines.iter().rev() {
        let chars: Vec<char> = line.chars().collect();
        for col in 0..scenario.len() {
            let val = chars[col * 4 + 1];
            if val.is_ascii_uppercase() {
                // The character is an ASCII uppercase symbol -> we must add it to the scenario
                scenario[col].push(val);
            }
        }
    }

    // PARSE THE MOVES
    lines.next(); // We first ignore the separation line
    while let Some(Ok(line)) = lines.next() {
        let x: Vec<&str> = line.split_whitespace().collect();
        let n = x[1].parse::<usize>().unwrap();
        let from = x[3].parse::<usize>().unwrap();
        let to = x[5].parse::<usize>().unwrap();
        moves.push((n, from, to));
    }

    (scenario, moves)
}

/// Sorts the crates one by one.
fn sort_cargo_1(scenario: &mut Scenario, moves: &[Move]) {
    for (n, from, to) in moves {
        for _ in 0..*n {
            let val = scenario[*from - 1].pop().unwrap();
            scenario[*to - 1].push(val);
        }
    }
}

/// Sorts the crates in stacks
fn sort_cargo_2(scenario: &mut Scenario, moves: &[Move]) {
    for (n, from, to) in moves {
        let len_from = scenario[*from - 1].len();
        let vals: Vec<char> = scenario[*from - 1].drain(len_from - n..).collect();
        scenario[*to - 1].extend(vals);
    }
}

/// Auxiliary function to print the top of the columns.
fn print_top(scenario: &Scenario) {
    for row in scenario.iter() {
        print!("{}", row.get(row.len() - 1).unwrap_or(&' '));
    }
    println!()
}

fn main() {
    // First we read the input file.
    let (mut scenario, moves) = read_input("data/05_input.txt");
    print_top(&scenario);
    println!();
    // Then, we make a copy of the scenario and sort it according to the first exercise.
    let mut scenario_1 = scenario.clone();
    sort_cargo_1(&mut scenario_1, &moves);
    print_top(&scenario_1);
    println!();
    // Finally, we sort the original scenario according to the second exercise (i.e., in stacks).
    sort_cargo_2(&mut scenario, &moves);
    print_top(&scenario);
}
