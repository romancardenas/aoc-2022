use std::cmp;
use std::collections::{HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::{Add, AddAssign, Sub};
use std::str::FromStr;

/// Structure that represents a point of the rope
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash, Ord, PartialOrd)]
struct Point(i32, i32, i32);

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.0, self.1, self.2)
    }
}

impl Add<Point> for Point {
    type Output = Self;

    fn add(self, rhs: Point) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Sub<Point> for Point {
    type Output = Self;

    fn sub(self, rhs: Point) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl FromStr for Point {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<i32> = s
            .trim()
            .split(',')
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        Ok(Point(coords[0], coords[1], coords[2]))
    }
}

/// Structure to represent a rock
#[derive(Debug)]
struct Rock {
    /// Storing rock points in a set will be very useful.
    cubes: HashSet<Point>,
}

impl Rock {
    /// Creates a new rock from a slice of points.
    fn new(points: &[Point]) -> Self {
        let mut cubes = HashSet::new();
        for &point in points {
            cubes.insert(point);
        }
        assert_eq!(points.len(), cubes.len()); // the slice and the set must have the same length!
        Self { cubes }
    }

    /// Computes the ENTIRE surface area (including internal wholes)
    fn surface_area(&self) -> usize {
        let mut res = 0;
        let distances = [
            Point(1, 0, 0),
            Point(0, 1, 0),
            Point(0, 0, 1),
            Point(-1, 0, 0),
            Point(0, -1, 0),
            Point(0, 0, -1),
        ];
        for &point in self.cubes.iter() {
            for neighbor in &distances.map(|d| point + d) {
                if !self.cubes.contains(neighbor) {
                    res += 1
                }
            }
        }
        res
    }

    /// Computes the surface area without considering inner wholes
    fn outer_surface_area(&self) -> usize {
        let mut res = 0;
        // We will explore the outer space looking for outer rock pixels
        let (min_point, max_point) = self.bounding_cube();
        let distances = [
            Point(1, 0, 0),
            Point(0, 1, 0),
            Point(0, 0, 1),
            Point(-1, 0, 0),
            Point(0, -1, 0),
            Point(0, 0, -1),
        ];
        // air pixels pending to be explored
        let mut pending = VecDeque::new();
        // air pixels that were already explored or are already pending
        let mut explored_or_pending = HashSet::new();
        // Initially, only the upper left corner is pending
        pending.push_back(min_point);
        explored_or_pending.insert(min_point);

        while !pending.is_empty() {
            // We get the first pending point and explore its neighbors
            let point = pending.pop_front().unwrap();
            for &neighbor in &distances.map(|d| d + point) {
                // If the neighbor is out of bounds or it is already explored/pending, we continue
                if Self::out_of_bounds(&neighbor, &min_point, &max_point)
                    || explored_or_pending.contains(&neighbor)
                {
                    continue;
                } else if self.cubes.contains(&neighbor) {
                    // Otherwise, if the neighbor is a pixel of the rock, we found an outer surface!
                    res += 1;
                } else {
                    // If not, it is just a outer air pixel that was not already pending
                    explored_or_pending.insert(neighbor);
                    pending.push_back(neighbor);
                }
            }
        }
        res
    }

    /// Checks if a point is out of bounds according to minimum and maximum points
    fn out_of_bounds(point: &Point, min: &Point, max: &Point) -> bool {
        point.0 < min.0
            || point.0 > max.0
            || point.1 < min.1
            || point.1 > max.1
            || point.2 < min.2
            || point.2 > max.2
    }

    /// It computes the bounding cube of the rock
    fn bounding_cube(&self) -> (Point, Point) {
        let mut min = Point(i32::MAX, i32::MAX, i32::MAX);
        let mut max = Point(i32::MIN, i32::MIN, i32::MIN);
        for &p in self.cubes.iter() {
            min = Point(
                cmp::min(min.0, p.0),
                cmp::min(min.1, p.1),
                cmp::min(min.2, p.2),
            );
            max = Point(
                cmp::max(max.0, p.0),
                cmp::max(max.1, p.1),
                cmp::max(max.2, p.2),
            );
        }
        // We must add a padding to ensure that the rock does not touch the bounding box
        (min - Point(1, 1, 1), max + Point(1, 1, 1))
    }
}
/// Reads the input file and returns the list of measurements.
fn read_input(path: &str) -> Rock {
    let file = File::open(path).expect("input file not found");
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let cubes: Vec<_> = lines
        .map(|l| Point::from_str(&l.expect("unable to parse line")).unwrap())
        .collect();
    Rock::new(&cubes)
}

fn main() {
    let rock = read_input("data/18_input.txt");
    println!("The surface area of the rock is {}", rock.surface_area());
    println!(
        "The outer surface area of the rock is {}",
        rock.outer_surface_area()
    );
}
