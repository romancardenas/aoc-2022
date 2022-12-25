use num::integer::lcm;
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{prelude::*, BufReader};

///                            ^        v       <        >       .
const DIRS: [(i32, i32); 5] = [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)];

#[derive(Debug)]
struct Scenario {
    n_rows: usize,
    n_cols: usize,
    blizzards: [HashSet<(usize, usize)>; 4],
}

impl Scenario {
    fn min_t(&self) -> Option<usize> {
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();
        let target = (self.n_rows, self.n_cols - 1);
        let lcm = lcm(self.n_rows, self.n_cols);

        queue.push_back((0, (-1, 0)));
        while !queue.is_empty() {
            let (t_prev, (i_prev, j_prev)) = queue.pop_front().unwrap();
            let t = t_prev + 1;
            for (i_delta, j_delta) in DIRS {
                let (i, j) = (i_prev + i_delta, j_prev + j_delta);
                if (i as usize, j as usize) == target {
                    // We reached the end!
                    return Some(t as usize);
                } else if self.is_valid(t as i32, (i, j)) {
                    // If we are in a valid location, we cache
                    let key = (t % lcm, (i, j));
                    if seen.insert(key) {
                        // We add the new location to the queue only if it was not previously cached
                        queue.push_back((t, (i, j)));
                    }
                }
            }
        }
        None
    }

    fn is_valid(&self, t: i32, location: (i32, i32)) -> bool {
        if location == (-1, 0) || location == (self.n_rows as i32, self.n_cols as i32 - 1) {
            // the origin and target locations are always valid.
            return true;
        }
        let (i, j) = location;
        if i < 0 || i as usize >= self.n_rows || j < 0 || j as usize >= self.n_cols {
            // the location is out of bounds
            return false;
        }
        for k in 0..DIRS.len() - 1 {
            // We check that there is no blizzard in the location at that time
            let (i_offset, j_offset) = DIRS[k];
            // To do so, we guess where should a blizzard be at time 0
            let blizzard_origin = (
                (i - i_offset * t) as usize % self.n_rows,
                (j - j_offset * t) as usize % self.n_cols,
            );
            if self.blizzards[k].contains(&blizzard_origin) {
                // If we find a blizzard that matches, then the location is not valid
                return false;
            }
        }
        true
    }
}

/// Reads the input file and returns the list of blueprints.
fn read_input(path: &str) -> Scenario {
    let file = File::open(path).expect("input file not found");
    let reader = BufReader::new(file);
    let lines: Vec<_> = reader.lines().filter_map(|l| l.ok()).collect();

    debug_assert!(lines.len() > 2);
    debug_assert!(lines[0].len() > 2);
    let n_rows = lines.len() - 2;
    let n_cols = lines[0].len() - 2;

    let mut blizzards = [
        HashSet::new(),
        HashSet::new(),
        HashSet::new(),
        HashSet::new(),
    ];
    for (i, line) in lines.iter().skip(1).enumerate() {
        for (j, item) in line.chars().filter(|&c| c != '#').enumerate() {
            if let Some(k) = "^v<>".chars().position(|c| c == item) {
                blizzards[k].insert((i, j));
            }
        }
    }
    Scenario {
        n_rows,
        n_cols,
        blizzards,
    }
}

fn main() {
    let scenario = read_input("data/24_input_test.txt");
    println!("{:?}", scenario.min_t());
}
