use std::cmp;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::{Add, AddAssign, Sub};

/// Structure that represents a point of the rope
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
struct Point(i32, i32);

impl Add<Point> for Point {
    type Output = Self;

    fn add(self, rhs: Point) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

/// Trait for adding points in place
impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

/// Trait for subtracting points
impl Sub<Point> for Point {
    type Output = Self;

    fn sub(self, rhs: Point) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

/// Structure to represent a rock
struct Rock {
    /// The location of a rock is always the upper left corner,
    location: Point,
    /// This vector indicates which internal points are rock in case of collision
    pieces: Vec<Point>,
}

impl Rock {
    /// It creates a new rock in a given location.
    fn new(location: Point, n: usize) -> Rock {
        match n {
            0 => Rock {
                location,
                pieces: vec![Point(0, 0), Point(1, 0), Point(2, 0), Point(3, 0)],
            },
            1 => Rock {
                location,
                pieces: vec![
                                 Point(1, 0),
                    Point(0, 1), Point(1, 1), Point(2, 1),
                                 Point(1, 2),
                ],
            },
            2 => Rock {
                location,
                pieces: vec![
                    Point(0, 0), Point(1, 0),  Point(2, 0),
                                               Point(2, 1),
                                               Point(2, 2),
                ],
            },
            3 => Rock {
                location,
                pieces: vec![
                    Point(0, 0),
                    Point(0, 1),
                    Point(0, 2),
                    Point(0, 3)
                ],
            },
            4 => Rock {
                location,
                pieces: vec![Point(0, 0), Point(1, 0),
                             Point(0, 1), Point(1, 1)],
            },
            _ => panic!("unknown rock code"),
        }
    }

    fn update_location(&mut self, new_location: Point) {
        self.location = new_location;
    }

    /// Returns a vector with the location of every "rock" points depending on its current location
    fn get_points(&self) -> Vec<Point> {
        self.pieces.iter().map(|&p| self.location + p).collect()
    }
}

struct Scenario {
    /// Simulated scenario with rocks etc.
    map: Vec<Vec<bool>>,
    /// Vector with the movement vector of the jets.
    jets: Vec<Point>,
}

impl Scenario {
    fn new(jets: Vec<Point>) -> Self {
        Self {
            map: Vec::new(),
            jets,
        }
    }

    fn reset(&mut self) {
        self.map = Vec::new()
    }

    /// It returns how many lines are blank at the bottom
    fn n_blank_rows(&self) -> usize {
        for (i, line) in self.map.iter().rev().enumerate() {
            if !line.iter().all(|&b| b) {
                return i;
            }
        }
        0
    }

    /// The scenario signature is a slice indicating how far is the first obstacle from the top
    fn get_signature(&self) -> [usize; 7] {
        // By default, the signature is the number of rows
        let mut res = [self.map.len(); 7];
        // Then, we iterate from top to bottom to find the first obstacle one each column.
        for (j, row) in self.map.iter().rev().enumerate() {
            for (i, _) in row.iter().enumerate().filter(|(_, &b)| !b) {
                res[i] = cmp::min(j, res[i]);
                // If we are done found the first obstacle for all the columns, we are done
                if res.iter().all(|&b| b < self.map.len()) {
                    return res;
                }
            }
        }
        res // If we reach this, it means that at least on column is free from top to bottom
    }

    /// Returns true if all the points are located in a free space
    fn valid_location(&self, points: &[Point]) -> bool {
        points.iter().all(|p| {
            match p.0 >= 0 && p.1 >= 0 {
                true => {
                    match self.map.get(p.1 as usize) {
                        Some(row) => {
                            match row.get(p.0 as usize) {
                                Some(&point) => point, // Check if there is something already
                                None => false, // It surpassed the right margin of the scenario
                            }
                        }
                        None => true, // It is way up, so it is valid
                    }
                }
                false => false, // It is out under the floor!
            }
        })
    }

    fn how_tall(&mut self, n_rocks: usize) -> usize {
        let (mut t, mut n, mut tall) = (0, 0, 0);
        let mut mem = HashMap::new();
        let mut res = HashMap::new();
        while n < n_rocks {
            // First we add empty rows (if required)
            for _ in 0..(3_usize).saturating_sub(self.n_blank_rows()) {
                self.map.push(vec![true; 7]);
            }
            // We create the corresponding rock
            let mut prev_location = Point(2, self.map.len() as i32);
            let mut rock = Rock::new(prev_location, n % 5);
            // The rock moves until we reach an invalid location
            while self.valid_location(&rock.get_points()) {
                prev_location = rock.location;
                // First, we try to move according to the jet
                rock.update_location(prev_location + self.jets[t % self.jets.len()]);
                if !self.valid_location(&rock.get_points()) {
                    // Oops! the location is invalid, we go back
                    rock.update_location(prev_location);
                }
                // Then, we try to go down
                prev_location = rock.location;
                rock.update_location(prev_location + Point(0, -1));
                t += 1; // At the end, we increase the time
            }
            // Finally, we go back to the previous valid location and fill the gap in the map
            rock.update_location(prev_location);
            // We add rows if needed...
            for point in rock.get_points() {
                while point.1 as usize >= self.map.len() {
                    self.map.push(vec![true; 7]);
                }
                self.map[point.1 as usize][point.0 as usize] = false;
            }
            // And compute the new state
            n += 1;
            tall = self.map.len() - self.n_blank_rows();
            // Now we check if we have a cache hit
            let top = self.get_signature();
            let jet_i = t % self.jets.len();
            let rock_i = n % 5;
            let mem_key = (top, jet_i, rock_i);
            if mem.contains_key(&mem_key) {
                // We found a repetition!!! Let's complete this easily Lets get the cached values
                let n_last = *mem.get(&mem_key).unwrap();
                let tall_last = *res.get(&n_last).unwrap();
                // First, we complete the entire repetitions
                let n_repetitions = (n_rocks - n) / (n - n_last);
                n += (n - n_last) * n_repetitions;
                tall += (tall - tall_last) * n_repetitions;
                // If there are still rocks pending, we consider just a part of the chunk once
                if n < n_rocks {
                    let n_extra = n_last + (n_rocks - n);
                    let tall_extra = *res.get(&n_extra).unwrap();
                    tall += tall_extra - tall_last;
                    return tall;
                }
            }
            // If we were not lucky, we cache the current state for the future
            mem.insert(mem_key, n);
            res.insert(n, tall);
        }
        tall // If we reach here, we simulated all the rocks without cache hits
    }
}

impl Display for Scenario {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.map.iter().rev() {
            write!(f, "|")?;
            for val in row.iter().map(|&x| match x {
                true => ".",
                false => "#",
            }) {
                write!(f, "{}", val)?;
            }
            writeln!(f, "|")?;
        }
        write!(f, "+")?;
        for _ in 0..7 {
            write!(f, "-")?;
        }
        writeln!(f, "+")
    }
}

/// Reads the input file and returns the list of measurements.
fn read_input(path: &str) -> Vec<Point> {
    let file = File::open(path).expect("input file not found");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| match c {
            '<' => Point(-1, 0),
            '>' => Point(1, 0),
            _ => panic!("unknown steam direction"),
        })
        .collect()
}

fn main() {
    let jets = read_input("data/17_input.txt");
    let mut scenario = Scenario::new(jets);
    println!(
        "After 2022 rocks, the scenario will be {} tall",
        scenario.how_tall(2022)
    );
    scenario.reset();
    println!(
        "After 1000000000000 rocks, the scenario will be {} tall",
        scenario.how_tall(1000000000000)
    );
}
