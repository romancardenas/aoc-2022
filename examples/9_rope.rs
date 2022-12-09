use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::{AddAssign, Sub};

/// Structure that represents a point of the rope
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    /// x coordinate
    x: i32,
    /// y coordinate
    y: i32,
}

/// Trait for adding points in place
impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

/// Trait for subtracting points
impl Sub<Point> for Point {
    type Output = Self;

    fn sub(self, rhs: Point) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

/// Trait to obtain movements from the input file
impl TryFrom<&str> for Point {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "L" => Ok(Self { x: -1, y: 0 }),
            "R" => Ok(Self { x: 1, y: 0 }),
            "D" => Ok(Self { x: 0, y: -1 }),
            "U" => Ok(Self { x: 0, y: 1 }),
            _ => Err(()),
        }
    }
}

/// Scenario.
#[derive(Debug)]
struct Scenario {
    /// position of the rope knots.
    knots: Vec<Point>,
    /// unique points where the tail of the rope has been.
    tail: HashSet<Point>,
}

impl Scenario {
    /// Creates a new scenario with `n_knots` knots.
    fn new(n_knots: usize) -> Self {
        let mut tail = HashSet::new();
        tail.insert(Point::default()); // By default, the tail starts in the point (0, 0)
        Self {
            knots: vec![Point::default(); n_knots], // Initially, all the knots are in (0, 0)
            tail,
        }
    }

    /// It returns the number of knots in the rope.
    fn n_knots(&self) -> usize {
        self.knots.len()
    }

    /// It moves the head of the rope to a given `direction`.
    /// It also computes the new location of the rest of the knots
    fn step(&mut self, direction: Point) {
        // First, the head of the rope moves to the desired direction.
        self.knots[0] += direction;
        // Next, we update the rest of the knots.
        for i in 1..self.n_knots() {
            let prev_knot = self.knots[i - 1];
            let knot = self.knots[i];
            let mut distance = prev_knot - knot; // distance between subsequent knots.
            let mut need_move = false; // flag to tell if the knot needs to move.
            if distance.x.abs() > 1 {
                // If the distance in the x axis is long enough, we prepare to move.
                need_move = true;
                distance.x = distance.x - distance.x.signum();
            }
            if distance.y.abs() > 1 {
                // If the distance in the y axis is long enough, we prepare to move.
                need_move = true;
                distance.y = distance.y - distance.y.signum();
            }
            if need_move {
                // If we need to move, we add the distance vector to the current knot.
                self.knots[i] += distance;
            }
        }
        // Finally, we add the new position of the tail of the rope to the tail history.
        let tail = *self.knots.last().unwrap();
        self.tail.insert(tail);
    }
}

/// Reads the input file and returns a vector with all the movements of the head of the rope.
fn read_input(path: &str) -> Vec<Point> {
    let mut res = Vec::new();
    let file = File::open(path).expect("input file not found");
    let reader = BufReader::new(file);
    let lines = reader.lines();

    for line in lines {
        let line = line.expect("error parsing line");
        let chars: Vec<&str> = line.split_whitespace().collect();
        let direction = chars[0].try_into().expect("error");
        for _ in 0..chars[1].parse::<usize>().expect("error") {
            res.push(direction);
        }
    }
    res
}

/// Simulates the scenario
fn simulate(scenario: &mut Scenario, directions: &[Point]) -> usize {
    for direction in directions {
        // We move the rope as much as required by the provided directions
        scenario.step(*direction);
    }
    // Finally, we return the length of the tail history.
    scenario.tail.len()
}

fn main() {
    // First we read the input file.
    let moves = read_input("data/9_input.txt");
    // Exercise 1
    let mut scenario_1 = Scenario::new(2);
    println!("{:?}", simulate(&mut scenario_1, &moves));
    // Exercise 2
    let mut scenario_2 = Scenario::new(10);
    println!("{:?}", simulate(&mut scenario_2, &moves));
}
