use std::fs::File;
use std::io::{prelude::*, BufReader};

/// Auxiliary type for referring to the areas that an elf must cover.
type ElfAreas = (usize, usize);

/// Reads the input file and returns a vector with the pairs of area ranges.
fn read_input(path: &str) -> Vec<(ElfAreas, ElfAreas)> {
    let mut res = Vec::new();
    let file = File::open(path).expect("input file not found");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let x: Vec<usize> = line
            .split(['-', ','])
            .map(|c| c.parse::<usize>().unwrap())
            .collect();
        res.push(((x[0], x[1]), (x[2], x[3])));
    }
    res
}

/// Returns true if the areas of the two elves fully overlap.
fn fully_overlaps(elf_1: &ElfAreas, elf_2: &ElfAreas) -> bool {
    (elf_1.0 <= elf_2.0 && elf_1.1 >= elf_2.1) || (elf_1.0 >= elf_2.0 && elf_1.1 <= elf_2.1)
}

/// Returns true if the areas of the two elves overlap at least in one area.
fn overlaps(elf_1: &ElfAreas, elf_2: &ElfAreas) -> bool {
    (elf_1.0 <= elf_2.0 && elf_1.1 >= elf_2.0) || (elf_1.0 >= elf_2.0 && elf_1.0 <= elf_2.1)
}

/// Function for the first exercise
fn exercise_1(areas: &[(ElfAreas, ElfAreas)]) -> usize {
    let mut res = 0;
    for (elf_1, elf_2) in areas {
        if fully_overlaps(elf_1, elf_2) {
            res += 1;
        }
    }
    res
}

/// Function for the second exercise
fn exercise_2(areas: &[(ElfAreas, ElfAreas)]) -> usize {
    let mut res = 0;
    for (elf_1, elf_2) in areas {
        if overlaps(elf_1, elf_2) {
            res += 1;
        }
    }
    res
}

fn main() {
    // First we read the input file.
    let areas = read_input("data/4_input.txt");
    println!("{:?}", exercise_1(&areas));
    println!("{:?}", exercise_2(&areas));
}
