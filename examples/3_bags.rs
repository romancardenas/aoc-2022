use std::fs::File;
use std::io::{prelude::*, BufReader};

/// Reads the input file and returns a vector of vectors with characters.
fn read_input(path: &str) -> Vec<String> {
    let mut res = Vec::new();
    let file = File::open(path).expect("input file not found");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        res.push(line.unwrap());
    }
    res
}

/// Returns a numeric priority from an ASCII character
fn get_priority(item: char) -> usize {
    if item.is_ascii_lowercase() {
        item as usize - 'a' as usize + 1
    } else if item.is_ascii_uppercase() {
        item as usize - 'A' as usize + 27
    } else {
        panic!("unknown char")
    }
}

/// It finds the duplicate in the two pockets and returns its priority code.
/// If there is no duplicate in the pockets, it returns 0.
fn find_duplicate(pocket_1: &str, pocket_2: &str) -> usize {
    for char_1 in pocket_1.chars() {
        for char_2 in pocket_2.chars() {
            if char_1 == char_2 {
                return get_priority(char_1);
            }
        }
    }
    0
}

/// Exercise 1: sums the priority code of all the duplicates in the pockets of the bags.
fn exercise_1(bags: &[String]) -> usize {
    let mut res = 0;
    for bag in bags.iter() {
        // We divide the bag in two pockets
        let (bag_1, bag_2) = (&bag[..bag.len() / 2], &bag[bag.len() / 2..]);
        res += find_duplicate(bag_1, bag_2);
    }
    res
}

/// From the bags of the three elves of the group, it finds the group badge and returns its priority code.
/// If it does not find the group badge, it panics.
fn find_group_badge(elf_1: &str, elf_2: &str, elf_3: &str) -> usize {
    for char_1 in elf_1.chars() {
        for char_2 in elf_2.chars() {
            if char_1 == char_2 {
                for char_3 in elf_3.chars() {
                    if char_1 == char_3 {
                        return get_priority(char_1);
                    }
                }
            }
        }
    }
    panic!("this group has no badge")
}

/// Exercise 2: sums the priority code of all group badges.
fn exercise_2(bags: &[String]) -> usize {
    let mut res = 0;
    for i in (0..bags.len()).step_by(3) {
        res += find_group_badge(&bags[i], &bags[i + 1], &bags[i + 2])
    }
    res
}

fn main() {
    // First we read the input file.
    let bags = read_input("data/3_input.txt");
    println!("{:?}", exercise_1(&bags));
    println!("{:?}", exercise_2(&bags));
}
