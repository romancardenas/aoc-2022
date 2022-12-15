use regex::Regex;
use std::cmp;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::{AddAssign, Sub};
use std::str::FromStr;

/// Structure that represents a point
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    /// x coordinate
    x: i32,
    /// y coordinate
    y: i32,
}

impl Point {
    /// To compute the Manhattan distance
    fn manhattan(from: &Self, to: &Self) -> i32 {
        let distance = *from - *to;
        distance.x.abs() + distance.y.abs()
    }

    /// To compute the tuning frequency of a beacon in a given point
    fn tuning_freq(&self) -> i64 {
        self.x as i64 * 4000000 + self.y as i64
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Point> for Point {
    type Output = Self;

    fn sub(self, rhs: Point) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

/// A structure for representing a measurement.
#[derive(Debug)]
struct Measure {
    /// Location of the sensor.
    sensor: Point,
    /// Location of the beacon nearest to the sensor.
    beacon: Point,
}

impl FromStr for Measure {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points: Vec<Point> = Vec::new();
        let re = Regex::new(r"x=(-?\d+), *y=(-?\d+)").unwrap();
        for cap in re.captures_iter(s) {
            points.push(Point {
                x: cap[1].parse::<i32>().unwrap(),
                y: cap[2].parse::<i32>().unwrap(),
            });
        }
        match points.len() {
            2 => Ok(Measure {
                sensor: points[0],
                beacon: points[1],
            }),
            _ => Err("different number of points"),
        }
    }
}

/// Reads the input file and returns the list of measurements.
fn read_input(path: &str) -> Vec<Measure> {
    let mut measures = Vec::new();
    let file = File::open(path).expect("input file not found");
    let reader = BufReader::new(file);
    let lines = reader.lines();
    for line in lines.map(|l| l.expect("error parsing line")) {
        measures.push(Measure::from_str(&line).unwrap());
    }
    measures
}

/// Returns the number of locations where a beacon cannot be located for a given row `y`
fn exercise_1(scenario: &[Measure], y: i32) -> usize {
    let mut impossible = HashSet::new(); // we use a hash set to avoid duplicates
    for measure in scenario {
        let manhattan = Point::manhattan(&measure.sensor, &measure.beacon);
        // First, we check if the range of the measure contains the row under study
        if measure.sensor.y - manhattan <= y && measure.sensor.y + manhattan >= y {
            // If so, we iterate over all the potential points in the range of the sensor
            let max_i = manhattan - (measure.sensor.y - y).abs();
            for x in (measure.sensor.x - max_i)..(measure.sensor.x + max_i + 1) {
                let point = Point { x, y };
                // If the point is not the beacon, we add it to the set of impossibles
                if point != measure.beacon {
                    impossible.insert(point);
                }
            }
        }
    }
    impossible.len() // We return the length of the resulting hash set.
}

/// Returns the tuning frequency of the beacon (if any)
fn exercise_2(scenario: &[Measure], min_val: i32, max_val: i32) -> Option<i64> {
    // We iterate over  rows looking for intersections with the sensors' ranges
    for y in min_val..max_val {
        let mut intersections = Vec::new();
        // For every measure, we look for the intersection with the current row
        for measure in scenario.iter() {
            let manhattan = Point::manhattan(&measure.sensor, &measure.beacon);
            // First, we check that the sensor is close enough to the row under study
            if (measure.sensor.y - y).abs() > manhattan {
                continue;
            }
            // If so, we compute the intersection and limit it to our search space.
            let max_i = manhattan - (measure.sensor.y - y).abs();
            let lower_x = cmp::max(min_val, measure.sensor.x - max_i);
            let upper_x = cmp::min(max_val, measure.sensor.x + max_i);
            // If the intersection happens within our search space, we add it to the intersections
            if upper_x >= lower_x {
                intersections.push((lower_x, upper_x));
            }
        }
        intersections.sort(); // Next we sort the intersection intervals in increasing order
        for i in 0..intersections.len() - 1 {
            // Now that intersections are sorted, we look for a gap between them
            let (_, first_right) = intersections[i];
            let (second_left, second_right) = intersections[i + 1];
            if second_left > first_right + 1 {
                // There is a gap between intersections! this is the solution
                let point = Point {
                    x: second_left - 1,
                    y,
                };
                return Some(point.tuning_freq());
            }
            // If there is no gap between intersections, we modify the tight-most value
            // This is important to avoid intersections fully contained in the previous one.
            intersections[i + 1].1 = cmp::max(first_right, second_right);
        }
    }
    None
}

fn main() {
    let scenario = read_input("data/15_input.txt");
    let row = 2000000;
    println!(
        "Number of points where the beacon cannot be in row {}: {}",
        row,
        exercise_1(&scenario, row)
    );
    let (min_val, max_val) = (0, 4000000);
    let beacon = exercise_2(&scenario, min_val, max_val);
    if let Some(b) = beacon {
        println!("The tuning frequency of the beacon is {}", b);
    } else {
        println!("There is no beacon from {} to {}", min_val, max_val);
    }
}
