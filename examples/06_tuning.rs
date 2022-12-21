use std::fs;

/// Reads the input file and returns the initial scenario and the moves to be done.
fn read_input(path: &str) -> String {
    fs::read_to_string(path).expect("unable to read file")
}

/// It detects a sequence of n different characters in code.
fn detect_sequence(code: &str, n: usize) -> usize {
    // First, we crete a vector of characters
    let chars: Vec<char> = code.chars().collect();
    // Educated guess: where the sequence of different characters start
    for start in 0..chars.len() - n {
        let mut hit = false; // If set to true, there is a repetition in the sequence
                             // We go from start to the end of the sequence (minus the last character)...
        for i in start..start + n - 1 {
            let char_i = chars[i];
            // ... and check that the rest of the characters of the sequence are different
            for char_j in chars.iter().take(start + n).skip(i + 1) {
                // If there is a hit, we note it down and exit to try with the next sequence
                if char_i == *char_j {
                    hit = true;
                    break;
                }
            }
            if hit {
                break;
            }
        }
        // If hit is false at this point, then we found the sequence!
        if !hit {
            return start + n;
        }
    }
    // If we reach this part, then there is no seuquence that long in the text.
    panic!("sequence not found")
}

fn main() {
    // First we read the input file.
    let code = read_input("data/06_input.txt");
    println!("{}", detect_sequence(&code, 4));
    println!("{}", detect_sequence(&code, 14));
}
