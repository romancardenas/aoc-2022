use std::cmp;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Tile {
    Free,
    Elf,
}

impl Tile {
    fn new(c: char) -> Result<Self, &'static str> {
        match c {
            '.' => Ok(Self::Free),
            '#' => Ok(Self::Elf),
            _ => Err("unknown character"),
        }
    }

    fn is_free(&self) -> bool {
        matches!(self, Self::Free)
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Free => ".",
                Self::Elf => "#",
            }
        )
    }
}

/// Structure that represents a point of the rope
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point(i32, i32);

impl Add<Point> for Point {
    type Output = Self;

    fn add(self, rhs: Point) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Self::Output {
        Point(self.0 * rhs, self.1 * rhs)
    }
}

impl Sub<Point> for Point {
    type Output = Self;

    fn sub(self, rhs: Point) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

type Dir = [(i32, i32); 3];
const UP: Dir = [(-1, -1), (0, -1), (1, -1)];
const DOWN: Dir = [(-1, 1), (0, 1), (1, 1)];
const LEFT: Dir = [(-1, -1), (-1, 0), (-1, 1)];
const RIGHT: Dir = [(1, -1), (1, 0), (1, 1)];
const DIRS: [Dir; 4] = [UP, DOWN, LEFT, RIGHT];

#[derive(Debug)]
struct Scenario {
    tiles: VecDeque<VecDeque<Tile>>,
}

impl Scenario {
    fn empty_ground(&self) -> usize {
        let (mut min_x, mut min_y, mut max_x, mut max_y) =
            (self.tiles.len(), self.tiles.len(), 0, 0);
        for (j, row) in self.tiles.iter().enumerate() {
            for (i, _) in row.iter().enumerate().filter(|(_, &t)| !t.is_free()) {
                (min_x, min_y) = (cmp::min(min_x, i), cmp::min(min_y, j));
                (max_x, max_y) = (cmp::max(max_x, i), cmp::max(max_y, j))
            }
        }
        let mut res = 0;
        for i in min_x..max_x + 1 {
            for j in min_y..max_y + 1 {
                if self.tiles[j][i].is_free() {
                    res += 1;
                }
            }
        }
        res
    }

    fn run_until(&mut self, n_iterations: usize) {
        println!("== Initial State ==");
        println!("{}", self);
        for n in 0..n_iterations {
            self.round(n);
        }
        println!("== End of Round {} ==", n_iterations);
        println!("{}", self);
    }

    fn run(&mut self) -> usize {
        println!("== Initial State ==");
        println!("{}", self);
        let mut n = 0;
        while self.round(n) != 0 {
            n += 1;
        }
        println!("== End of Round {} ==", n + 1);
        println!("{}", self);
        n + 1
    }

    fn round(&mut self, n: usize) -> usize {
        // First, we compute the candidates
        let mut candidates: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
        let mut offset = (0, 0);
        for (j, row) in self.tiles.iter().enumerate() {
            for (i, _) in row.iter().enumerate().filter(|(_, &t)| !t.is_free()) {
                let origin = (i as i32, j as i32);
                if self.isolated(&origin) {
                    // If the elf is isolated, we stop
                    continue;
                }
                for x in 0..DIRS.len() {
                    let directions = DIRS[(n + x) % DIRS.len()];
                    if self.free_direction(&origin, &directions) {
                        let central = directions[directions.len() / 2];
                        let destination = (origin.0 + central.0, origin.1 + central.1);
                        // We check if we are out of bounds from the up/left
                        if destination.0 + offset.0 < 0 {
                            offset.0 += 1;
                        }
                        if destination.1 + offset.1 < 0 {
                            offset.1 += 1;
                        }
                        candidates.entry(destination).or_default().push(origin);
                        break;
                    }
                }
            }
        }
        // Next, we add left/up padding to the scenario if needed
        for _ in 0..offset.0 {
            for row in self.tiles.iter_mut() {
                row.push_front(Tile::Free);
            }
        }
        for _ in 0..offset.1 {
            let mut row = VecDeque::new();
            for _ in 0..self.tiles[0].len() {
                row.push_front(Tile::Free);
            }
            self.tiles.push_front(row);
        }
        // Next, we move the elves
        let mut res = 0;
        for (destination, from) in candidates.iter() {
            if from.len() != 1 {
                // If there are more than one candidates, we ignore them
                continue;
            }
            // We apply the offset and cast to usize
            let origin = (
                (from[0].0 + offset.0) as usize,
                (from[0].1 + offset.1) as usize,
            );
            let destination = (
                (destination.0 + offset.0) as usize,
                (destination.1 + offset.1) as usize,
            );
            // Next, we add right/down padding to the scenario if needed
            if destination.0 >= self.tiles[0].len() {
                for row in self.tiles.iter_mut() {
                    row.push_back(Tile::Free);
                }
            }
            if destination.1 >= self.tiles.len() {
                let mut row = VecDeque::new();
                for _ in 0..self.tiles[0].len() {
                    row.push_front(Tile::Free);
                }
                self.tiles.push_back(row);
            }

            assert_eq!(Tile::Elf, self.tiles[origin.1][origin.0]);
            assert_eq!(Tile::Free, self.tiles[destination.1][destination.0]);
            self.tiles[origin.1][origin.0] = Tile::Free;
            self.tiles[destination.1][destination.0] = Tile::Elf;
            res += 1;
        }
        res
    }

    fn inside(&self, point: &(i32, i32)) -> bool {
        point.1 >= 0
            && point.1 < self.tiles.len() as i32
            && point.0 >= 0
            && point.0 < self.tiles[0].len() as i32
    }

    fn isolated(&self, origin: &(i32, i32)) -> bool {
        self.free_direction(origin, &UP)
            && self.free_direction(origin, &DOWN)
            && self.free_direction(origin, &LEFT)
            && self.free_direction(origin, &RIGHT)
    }

    fn free_direction(&self, origin: &(i32, i32), directions: &Dir) -> bool {
        for (i_delta, j_delta) in directions.iter() {
            let point = (origin.0 + i_delta, origin.1 + j_delta);
            if self.inside(&point) && !self.tiles[point.1 as usize][point.0 as usize].is_free() {
                return false;
            }
        }
        true
    }
}

impl Display for Scenario {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.tiles.iter() {
            for tile in row.iter() {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

/// Reads the input file and returns the list of blueprints.
fn read_input(path: &str) -> Scenario {
    let file = File::open(path).expect("input file not found");
    let reader = BufReader::new(file);

    let tiles: VecDeque<VecDeque<_>> = reader
        .lines()
        .map(|l_res| {
            let l = l_res.expect("unable to read line");
            l.chars()
                .map(|c| Tile::new(c).expect("unknown character"))
                .collect()
        })
        .collect();
    Scenario { tiles }
}

fn main() {
    let mut scenario = read_input("data/23_input.txt");
    scenario.run_until(10);
    println!(
        "EXERCISE 1: the number of empty ground tiles after 10 iterations is {}",
        scenario.empty_ground()
    );

    let mut scenario = read_input("data/23_input.txt");
    let n = scenario.run();
    println!("EXERCISE 2: after {} iterations, the number of empty ground tiles once all the elves stop is {}", n, scenario.empty_ground());
}
