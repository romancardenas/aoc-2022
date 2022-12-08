use std::fs::File;
use std::io::{prelude::*, BufReader};

/// Reads the input file and returns the scenario.
fn read_input(path: &str) -> Vec<Vec<usize>> {
    let mut scenario = Vec::new();

    let file = File::open(path).expect("input file not found");
    let reader = BufReader::new(file);
    let lines = reader.lines();

    for line in lines {
        let line = line.expect("error parsing line");
        let trees: Vec<usize> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        scenario.push(trees);
    }
    scenario
}

/// It returns the number of trees that are visible from the outside
fn exercise_1(scenario: &Vec<Vec<usize>>) -> usize {
    let i_limit = scenario.len();
    let mut res = i_limit * 2; // The edges are completely visible
    for i in 1..i_limit - 1 {
        let j_limit = scenario[i].len();
        res += 2; // The edges are completely visible
        for j in 1..j_limit - 1 {
            let tree = scenario[i][j];
            let mut visible = true;
            // Let's see if the tree is visible from the top
            for row in scenario.iter().take(i) {
                let other = row[j];
                if other >= tree {
                    visible = false;
                    break;
                }
            }
            if !visible {
                visible = true;
                // Let's see if the tree is visible from the bottom
                for row in scenario.iter().take(i_limit).skip(i + 1) {
                    let other = row[j];
                    if other >= tree {
                        visible = false;
                        break;
                    }
                }
            }
            if !visible {
                visible = true;
                // Let's see if the tree is visible from the left
                for m in 0..j {
                    let other = scenario[i][m];
                    if other >= tree {
                        visible = false;
                        break;
                    }
                }
            }
            if !visible {
                visible = true;
                // Let's see if the tree is visible from the right
                for m in j + 1..j_limit {
                    let other = scenario[i][m];
                    if other >= tree {
                        visible = false;
                        break;
                    }
                }
            }
            // If visible, we add it to the final result
            if visible {
                res += 1;
            }
        }
    }
    res
}

/// It returns the highest scenic score possible for any tree in the scenario.
fn exercise_2(scenario: &Vec<Vec<usize>>) -> usize {
    let mut best_score = 0;
    let max_i = scenario.len();
    for i in 0..max_i {
        let max_j = scenario[i].len();
        for j in 0..max_j {
            // For every tree in the scenario, we check the number of trees that are visible
            let tree = scenario[i][j];
            let (mut top, mut down, mut left, mut right) = (0, 0, 0, 0);
            // First, we check the north
            for n in (0..i).rev() {
                let other = scenario[n][j];
                top += 1;
                if other >= tree {
                    break;
                }
            }
            // Then, we check the south
            for n in i + 1..max_i {
                let other = scenario[n][j];
                down += 1;
                if other >= tree {
                    break;
                }
            }
            // Next, the east
            for m in (0..j).rev() {
                let other = scenario[i][m];
                left += 1;
                if other >= tree {
                    break;
                }
            }
            // FInally, the west
            for m in j + 1..max_j {
                let other = scenario[i][m];
                right += 1;
                if other >= tree {
                    break;
                }
            }
            // The tree's score is the multiplication of all the directions
            let score = top * down * left * right;
            if score > best_score {
                best_score = score;
            }
        }
    }
    best_score
}

fn main() {
    // First we read the input file.
    let x = read_input("data/8_input.txt");
    println!("{}", exercise_1(&x));
    println!("{}", exercise_2(&x));
}
