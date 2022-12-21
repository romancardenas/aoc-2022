use std::fs::File;
use std::io::{prelude::*, BufReader};

/// Enumeration with all the symbols and their associated points
#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
enum Symbol {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

/// It parses a symbol from a string slice.
/// This is useful to obtain the elves' symbol as well as my symbol in the first exercise.
fn parse_symbol(symbol: &str) -> Symbol {
    if symbol == "A" || symbol == "X" {
        return Symbol::Rock;
    } else if symbol == "B" || symbol == "Y" {
        return Symbol::Paper;
    } else if symbol == "C" || symbol == "Z" {
        return Symbol::Scissors;
    } else {
        panic!("unknwon symbol");
    }
}

/// It parses the elf's symbol as regular and mine depending on whether I must win/draw/lose.
/// This function is useful only in the second exercise.
fn parse_symbols(elf: &str, me: &str) -> (Symbol, Symbol) {
    let elf_symbol = parse_symbol(elf);
    let my_symbol: Symbol;
    if me == "X" {
        // I need to lose
        my_symbol = match elf_symbol {
            Symbol::Rock => Symbol::Scissors,
            Symbol::Paper => Symbol::Rock,
            Symbol::Scissors => Symbol::Paper,
        };
    } else if me == "Y" {
        // I need a draw
        my_symbol = elf_symbol
    } else if me == "Z" {
        // I need to win
        my_symbol = match elf_symbol {
            Symbol::Rock => Symbol::Paper,
            Symbol::Paper => Symbol::Scissors,
            Symbol::Scissors => Symbol::Rock,
        };
    } else {
        panic!("unknown symbol")
    }
    (elf_symbol, my_symbol)
}

/// Reads the input file and returns a vector of plays. This is only useful for the first exercise.
fn read_input_1(path: &str) -> Vec<(Symbol, Symbol)> {
    let mut res = Vec::new();
    let file = File::open(path).expect("input file not found");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let mut x = line.split_whitespace();
        let elf: Symbol = parse_symbol(x.next().unwrap());
        let me: Symbol = parse_symbol(x.next().unwrap());
        res.push((elf, me));
    }
    res
}

fn read_input_2(path: &str) -> Vec<(Symbol, Symbol)> {
    let mut res = Vec::new();
    let file = File::open(path).expect("input file not found");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let mut x = line.split_whitespace();
        res.push(parse_symbols(x.next().unwrap(), x.next().unwrap()));
    }
    res
}

/// It computes the points I obtained in a given match.
fn match_points(elf: Symbol, me: Symbol) -> usize {
    let mut res: usize = me as usize;
    res += match elf {
        Symbol::Rock => match me {
            Symbol::Rock => 3,
            Symbol::Paper => 6,
            Symbol::Scissors => 0,
        },
        Symbol::Paper => match me {
            Symbol::Rock => 0,
            Symbol::Paper => 3,
            Symbol::Scissors => 6,
        },
        Symbol::Scissors => match me {
            Symbol::Rock => 6,
            Symbol::Paper => 0,
            Symbol::Scissors => 3,
        },
    };
    res
}

/// It computes the total amount of points obtained in all the matches.
fn total_points(plays: &[(Symbol, Symbol)]) -> usize {
    let mut res = 0;
    for play in plays.iter() {
        res += match_points(play.0, play.1);
    }
    res
}

fn main() {
    // First we read the input file.
    // let plays = read_input_1("data/2_input.txt"); // For the first exercise
    let plays = read_input_2("data/02_input.txt"); // For the second exercise
    println!("{:?}", plays);
    // Finally, we sum the points obtained in the competition.
    println!("{:?}", total_points(&plays))
}
