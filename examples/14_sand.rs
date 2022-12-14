extern crate core;

use std::cmp;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::{AddAssign, Sub};
use std::str::FromStr;

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

/// Trait to obtain points from string
impl FromStr for Point {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coordinates = s.trim().split(',').map(|x| x.trim());
        let (x, y) = (coordinates.next(), coordinates.next());
        if x.is_none() || y.is_none() {
            return Err("not enough coordinates");
        }
        if coordinates.next().is_some() {
            return Err("too many coordinates");
        }
        let (x, y) = (x.unwrap().parse::<i32>(), y.unwrap().parse::<i32>());
        if x.is_err() || y.is_err() {
            return Err("unable to parse coordinates");
        }
        let (x, y) = (x.unwrap(), y.unwrap());
        Ok(Self { x, y })
    }
}

/// Helper struct to parse the input file
#[derive(Debug)]
struct Rock {
    points: Vec<Point>,
}

impl FromStr for Rock {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = Vec::new();
        for point in s.trim().split("->") {
            points.push(Point::from_str(point)?);
        }
        Ok(Self { points })
    }
}

/// All the possible elements in the scenario
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Element {
    Air,
    Rock,
    Sand,
    Source,
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let x = match self {
            Element::Air => ' ', // Representing the air as '.' is a little bit confusing
            Element::Rock => '#',
            Element::Sand => 'o',
            Element::Source => '+',
        };
        write!(f, "{}", x)
    }
}

/// The actual simulation scenario
struct Scenario {
    map: Vec<Vec<Element>>, // Pixels of the map
    corner: Point,          // lower-right coordinate
    source: Point,          // Point where the sand source is located
}

static PADDING: usize = 2; // Padding used for the second exercise when adding the floor

impl Scenario {
    fn new(rocks: &[Rock], source: Point, floor: bool) -> Self {
        // First let's find the corner
        let mut corner = source;
        for rock in rocks {
            for point in rock.points.iter() {
                corner.x = cmp::max(corner.x, point.x);
                corner.y = cmp::max(corner.y, point.y);
            }
        }
        // The dimensions of the map depends on whether we want ot include floor or not
        let mut map = match floor {
            true => {
                let mut aux =
                    vec![vec![Element::Air; corner.x as usize * 2]; corner.y as usize + PADDING];
                aux.push(vec![Element::Rock; corner.x as usize * 2]); // we add a row of rocks
                aux
            }
            false => vec![vec![Element::Air; corner.x as usize + 1]; corner.y as usize + 1],
        };
        // Then, we just have to add the proper rocks
        for rock in rocks {
            for i in 0..rock.points.len() - 1 {
                let (mut from, to) = (rock.points[i], rock.points[i + 1]);
                let distance = to - from;
                match distance {
                    Point { x: _, y: 0 } | Point { x: 0, y: _ } => {}
                    _ => panic!("invalid distance"), // at least one of the coordinates must be 0
                };
                let gradient = Point {
                    x: distance.x.signum(),
                    y: distance.y.signum(),
                };
                // We fill the pixels from from to to with rocks
                map[from.y as usize][from.x as usize] = Element::Rock;
                while from != to {
                    from += gradient;
                    map[from.y as usize][from.x as usize] = Element::Rock;
                }
            }
        }
        // Finally, we add the sand source to the map
        map[source.y as usize][source.x as usize] = Element::Source;
        Self {
            map,
            corner,
            source,
        }
    }

    /// It creates a new sand pixel and looks for its resting place
    fn step(&mut self) -> Point {
        let mut sand = self.source; // initially, the location is the source
                                    // Next we iterate looking for the resting place
        while let Some(next_sand) = match self.map[sand.y as usize + 1][sand.x as usize] {
            Element::Air => Some(Point {
                // we move down
                x: sand.x,
                y: sand.y + 1,
            }),
            _ => match self.map[sand.y as usize + 1][sand.x as usize - 1] {
                // we move down-left
                Element::Air => Some(Point {
                    x: sand.x - 1,
                    y: sand.y + 1,
                }),
                _ => match self.map[sand.y as usize + 1][sand.x as usize + 1] {
                    // we move down-right
                    Element::Air => Some(Point {
                        x: sand.x + 1,
                        y: sand.y + 1,
                    }),
                    _ => None, // we cannot move anymore!
                },
            },
        } {
            sand = next_sand;
            if sand.y as usize >= self.map.len() - 1 {
                // We reached the limit of the map
                break;
            }
        }
        // we modify the proper pixel and return it
        self.map[sand.y as usize][sand.x as usize] = Element::Sand;
        sand
    }
}

impl Display for Scenario {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // First we find the first column with something that is not air
        let mut first_elem: usize = usize::MAX;
        for row in self.map.iter() {
            for (i, element) in row.iter().enumerate() {
                if *element != Element::Air {
                    first_elem = cmp::max(1, cmp::min(first_elem, i));
                }
            }
        }
        // Then, we print each row after skipping the empty space
        for row in self.map.iter() {
            for element in row.iter().skip(first_elem - 1) {
                write!(f, "{}", element)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

/// Reads the input file and returns a scenario.
fn read_input(path: &str) -> Vec<Rock> {
    let mut res = Vec::new();
    let file = File::open(path).expect("input file not found");
    let reader = BufReader::new(file);
    let lines = reader.lines();
    for line in lines.map(|l| l.expect("error parsing line")) {
        let rock = Rock::from_str(&line).unwrap();
        res.push(rock);
    }
    res
}

/// First exercise -> we wait until the sand reaches the last row
fn exercise_1(scenario: &mut Scenario) -> usize {
    let mut res = 1;
    while scenario.step().y < scenario.corner.y {
        res += 1;
    }
    res - 1 // we are already overflowing, we need to subtract one!
}

/// Second exercise -> the scenario has floor, we wait until the sand reaches the source
fn exercise_2(scenario: &mut Scenario) -> usize {
    let mut res = 1;
    while scenario.step() != scenario.source {
        res += 1;
    }
    res
}

fn main() {
    let rocks = read_input("data/14_input.txt");
    let mut scenario = Scenario::new(&rocks, Point { x: 500, y: 0 }, false);
    let x = exercise_1(&mut scenario);
    println!("{scenario}");
    println!();
    println!();
    println!("The sand starts to overflow after {} grains", x);

    println!();
    println!("===================================================================================");
    println!("===================================================================================");
    println!("===================================================================================");
    println!();
    let mut scenario = Scenario::new(&rocks, Point { x: 500, y: 0 }, true);
    let x = exercise_2(&mut scenario);
    println!("{scenario}");
    println!();
    println!("===================================================================================");
    println!("===================================================================================");
    println!("===================================================================================");
    println!();
    println!("The sand reaches the source after {} grains", x);
}
