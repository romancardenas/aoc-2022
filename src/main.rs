use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::{Add, AddAssign, Sub};

/// Structure that represents a point of the rope
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
struct Point (i32, i32);

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


struct Rock {
    /// The location of a rock is always the upper left corner,
    location: Point,
    /// This vector indicates which internal points are rock in case of collision
    pieces: Vec<Point>,
}

impl Rock {
    fn new(location: Point, n: usize) -> Rock {
        match n {
            0 => Rock {
                location,
                pieces: vec![
                    Point(0, 0), Point(1, 0), Point(2, 0), Point (3, 0),
                ],
            },
            1 => Rock {
                location,
                pieces: vec![               Point(1, 2),
                    Point(0, 1), Point(1, 1), Point(2, 1),
                                            Point(1, 0),
                ],
            },
            2 => Rock {
                location,
                pieces: vec![
                                                                   Point(2, 2),
                                                                   Point(2, 1),
                    Point(0, 0), Point(1, 0), Point(2, 0),
                ],
            },
            3 => Rock {
                location,
                pieces: vec![
                    Point(0, 3),
                    Point(0, 2),
                    Point(0, 1),
                    Point(0, 0),
                ],
            },
            4 => Rock {
                location,
                pieces: vec![
                    Point(0, 1), Point(1, 1),
                    Point(0, 0), Point(1, 0),
                ],
            },
            _ => panic!("unknown rock code")
        }
    }

    fn advance(&mut self, new_location: Point) {
        self.location = new_location;
    }

    fn get_points(&self) -> Vec<Point> {
        self.pieces.iter().map(|&p| self.location + p).collect()
    }
}

struct Scenario {
    width: usize,
    map: Vec<Vec<bool>>,
    jets: Vec<Point>,
}

impl Scenario {
    fn new(width: usize, jets: Vec<Point>) -> Self {
        Self {width, map: Vec::new(), jets}
    }

    fn reset(&mut self) {
        self.map.clear();
    }

    fn n_blank_rows(&self) -> usize {
        for (i, line) in self.map.iter().rev().enumerate() {
            if !line.iter().all(|&b| b) {
                return i;
            }
        }
        0
    }

    fn get_top_signature(&self) -> [bool; 7] {
        let mut res = [false; 7];
        let top = self.map.len().saturating_sub(1) - self.n_blank_rows();
        for line in self.map[top] {
            for i in 0..7 {
                res[i] = line[i];
            }
        }
        res
    }

    fn get_jet(&self, t: usize) -> Point {
        let n_jets = self.jets.len();
        self.jets[t % n_jets]
    }

    fn free_space(&self, points: &[Point]) -> bool {
        points.iter().all(|p| {
            match p.0 >= 0 && p.1 >= 0 {
                true => {
                    match self.map.get(p.1 as usize) {
                        Some(row) => {
                            match row.get(p.0 as usize) {
                                Some(&point) => point,  // Check if there is something already
                                None => false,  // It surpassed the right margin of the scenario
                            }
                        },
                        None => true, // It is way up, so it is valid
                    }
                },
                false => false,  // It is out under the floor!
            }
        })
    }

    fn how_tall(&mut self, n_rocks: usize, mem: &mut HashMap<([bool; 7], usize, usize), (usize, usize)>) -> usize {
        let mut t = 0;
        for n in 0..n_rocks {

            let top = self.get_top_signature();
            let jet_i = t % self.jets.len();
            let rock_i = n % 5;
            let mem_key = (top, jet_i, rock_i);
            if mem.contains_key(&mem_key) {
                todo!() // Ha habido un match!
            }
            // First we add empty rows (if required)
            for _ in 0..(3 as usize).saturating_sub(self.n_blank_rows()) {
                self.map.push(vec![true; self.width]);
            }
            // We create the corresponding rock
            let mut prev_location = Point(2, self.map.len() as i32);
            let mut rock = Rock::new(prev_location, rock_i);

            // The rock moves until we reach an invalid location
            while self.free_space(&rock.get_points()) {
                prev_location = rock.location;
                // First, we try to move according to the jet
                rock.advance(prev_location + self.jets[jet_i]);
                if !self.free_space(&rock.get_points()) {
                    // Oops! the location is invalid, we go back
                    rock.advance(prev_location);
                }
                // Then, we try to go down
                prev_location = rock.location;
                rock.advance(prev_location + Point(0, -1));
                t += 1;  // At the end, we increase the time
            }
            // Finally, we go back to the previous valid location and fill the gap in the map
            rock.advance(prev_location);
            // We add rows if needed...
            for point in rock.get_points() {
                while point.1 as usize >= self.map.len() {
                    self.map.push(vec![true; self.width]);
                }
                self.map[point.1 as usize][point.0 as usize] = false;
            }
            mem.insert(mem_key, (t, self.map.len() - self.n_blank_rows()));
        }
        self.map.len() - self.n_blank_rows()
    }

    fn exercise_1(&mut self, n_rocks: usize) -> usize {
        let mut t = 0;
        for n in 0..n_rocks {
            // First we add empty rows
            for _ in 0..(3 as usize).saturating_sub(self.n_blank_rows()) {
                self.map.push(vec![true; self.width]);
            }
            // We create the corresponding rock
            let mut prev_location = Point(2, self.map.len() as i32);
            let mut rock = Rock::new(prev_location, n);

            while self.free_space(&rock.get_points()) {
                prev_location = rock.location;
                // First, we try to move according to the jet
                let jet = self.get_jet(t);
                rock.advance(prev_location + jet);
                if !self.free_space(&rock.get_points()) {
                    // If the location is invalid, we go back
                    rock.advance(prev_location);
                }
                // Then, we try to go down
                prev_location = rock.location;
                rock.advance(prev_location + Point(0, -1));
                t += 1;
            }
            // Finally, we go back to the previous valid location and fill the gap in the map
            rock.advance(prev_location);

            for point in rock.get_points() {
                while point.1 as usize >= self.map.len() {
                    self.map.push(vec![true; self.width]);
                }
                self.map[point.1 as usize][point.0 as usize] = false;
            }
        }
        self.map.len() - self.n_blank_rows()
    }
}

impl Display for Scenario {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.map.iter().rev() {
            write!(f, "|")?;
            for val in row.iter().map(|&x| match x { true => ".", false => "#" }) {
                write!(f, "{}", val)?;
            }
            writeln!(f, "|")?;
        }
        write!(f, "+")?;
        for _ in 0..self.width {
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

    lines.next().unwrap().unwrap().chars().map(|c| {
        match c {
            '<' => Point(-1, 0),
            '>' => Point(1, 0),
            _ => panic!("unknown steam direction")
        }
    }).collect()
}

fn main() {
    let jets = read_input("data/17_input_test.txt");
    let mut scenario = Scenario::new(7, jets);
    println!("{}", &scenario.exercise_1(2022));
    println!("{}", &scenario.exercise_2(1000000000000));
}
