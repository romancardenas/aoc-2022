use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Tile {
    Air,
    Open,
    Wall,
}

impl Tile {
    fn new(c: char) -> Result<Self, &'static str> {
        match c {
            ' ' => Ok(Self::Air),
            '.' => Ok(Self::Open),
            '#' => Ok(Self::Wall),
            _ => Err("unknown character"),
        }
    }

    fn is_air(&self) -> bool {
        matches!(self, Self::Air)
    }

    fn is_open(&self) -> bool {
        matches!(self, Self::Open)
    }

    fn is_wall(&self) -> bool {
        matches!(self, Self::Wall)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Move {
    Steps(u32),
    TurnRight,
    TurnLeft,
}

impl Move {
    fn is_steps(&self) -> bool {
        matches!(self, Self::Steps(_))
    }
    fn get_steps(&self) -> Option<u32> {
        match self {
            Self::Steps(n) => Some(*n),
            _ => None,
        }
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

const UP: Point = Point(0, -1);
const DOWN: Point = Point(0, 1);
const LEFT: Point = Point(-1, 0);
const RIGHT: Point = Point(1, 0);

#[derive(Debug)]
struct State {
    location: Point,
    direction: Point,
}

impl State {
    fn new(tiles: &[Vec<Tile>]) -> Self {
        let mut location = None;
        for (j, row) in tiles.iter().enumerate() {
            for (i, tile) in row.iter().enumerate() {
                if let Tile::Open = tile {
                    location = Some(Point(i as i32, j as i32));
                    break;
                }
            }
            if location.is_some() {
                break;
            }
        }
        let location = location.expect("unable to find an open tile");
        Self {
            location,
            direction: Point(1, 0),
        }
    }

    fn step_plane(&mut self, n: u32, tiles: &[Vec<Tile>]) {
        let n_rows = tiles.len() as i32;
        let n_cols = tiles[0].len() as i32;
        for _ in 0..n {
            assert!(tiles[self.location.1 as usize][self.location.0 as usize].is_open());
            let mut next_location = self.location + self.direction;
            // First we make sure that we are not out of the scenario
            next_location.1 = (next_location.1 + n_rows) % n_rows;
            next_location.0 = (next_location.0 + n_cols) % n_cols;
            // Next we wrap in case of finding void tiles
            while tiles[next_location.1 as usize][next_location.0 as usize].is_air() {
                next_location += self.direction;
                next_location.1 = (next_location.1 + n_rows) % n_rows;
                next_location.0 = (next_location.0 + n_cols) % n_cols;
            }
            // If the tile is a wall, we don't move anymore
            if tiles[next_location.1 as usize][next_location.0 as usize].is_wall() {
                break;
            }
            self.location = next_location;
        }
    }

    fn step_cube(
        &mut self,
        n: u32,
        tiles: &[Vec<Tile>],
        cube_map: &HashMap<((i32, i32), Point), ((i32, i32), Point)>,
    ) {
        let n_rows = tiles.len() as i32;
        let n_cols = tiles[0].len() as i32;
        let cube_dim = cmp::min(n_rows, n_cols) / 3;
        for _ in 0..n {
            assert!(tiles[self.location.1 as usize][self.location.0 as usize].is_open());
            let mut next_location = self.location + self.direction;
            let mut next_direction = self.direction;
            // First we make sure that we are not out of the scenario
            next_location.1 = (next_location.1 + n_rows) % n_rows;
            next_location.0 = (next_location.0 + n_cols) % n_cols;
            // If we are in the middle of nowhere, we need to find the corresponding cube face
            if tiles[next_location.1 as usize][next_location.0 as usize].is_air() {
                let current_face = (self.location.0 / cube_dim, self.location.1 / cube_dim);
                let (next_face, next_dir) = cube_map.get(&(current_face, self.direction)).unwrap();
                next_direction = *next_dir;
                next_location = Point(next_face.0 * cube_dim, next_face.1 * cube_dim);
                // I Probably not cover all the possibilities, just those I encountered
                match next_direction {
                    UP => {
                        next_location.1 += cube_dim - 1;
                        next_location.0 += match self.direction {
                            UP => self.location.0 % cube_dim,
                            RIGHT => self.location.1 % cube_dim,
                            _ => panic!("not happen"),
                        }
                    }
                    DOWN => {
                        next_location.0 += match self.direction {
                            LEFT => self.location.1 % cube_dim,
                            DOWN => self.location.0 % cube_dim,
                            _ => panic!("not happen"),
                        }
                    }
                    LEFT => {
                        next_location.0 += cube_dim - 1;
                        next_location.1 += match self.direction {
                            LEFT => self.location.1 % cube_dim,
                            RIGHT => (cube_dim - 1) - self.location.1 % cube_dim,
                            DOWN => self.location.0 % cube_dim,
                            _ => panic!("not happen"),
                        }
                    }
                    RIGHT => {
                        next_location.1 += match self.direction {
                            LEFT => (cube_dim - 1) - self.location.1 % cube_dim,
                            RIGHT => self.location.1 % cube_dim,
                            UP | DOWN => self.location.0 % cube_dim,
                            _ => panic!("not happen"),
                        }
                    }
                    _ => panic!("not happen"),
                };
            }
            if tiles[next_location.1 as usize][next_location.0 as usize].is_wall() {
                break;
            }
            self.location = next_location;
            self.direction = next_direction;
        }
    }

    fn turn_right(&mut self) {
        self.direction = Point(-self.direction.1, self.direction.0);
    }

    fn turn_left(&mut self) {
        self.direction = Point(self.direction.1, -self.direction.0);
    }
}

#[derive(Debug)]
struct Scenario {
    tiles: Vec<Vec<Tile>>,
    moves: Vec<Move>,
    state: State,
}

impl Scenario {
    fn new(tiles: Vec<Vec<Tile>>, moves: Vec<Move>) -> Self {
        let state = State::new(&tiles);
        Self {
            tiles,
            moves,
            state,
        }
    }

    fn reset(&mut self) {
        self.state = State::new(&self.tiles);
    }

    fn exercise_1(&mut self) {
        println!(
            "INITIAL STATE: {:?} ({:?})",
            self.state.location, self.state.direction
        );
        for &m in self.moves.iter() {
            match m {
                Move::Steps(n) => self.state.step_plane(n, &self.tiles),
                Move::TurnRight => self.state.turn_right(),
                Move::TurnLeft => self.state.turn_left(),
            }
        }
        println!(
            "FINAL STATE: {:?} ({:?})",
            self.state.location, self.state.direction
        );
    }

    fn exercise_2(&mut self, cube_map: &HashMap<((i32, i32), Point), ((i32, i32), Point)>) {
        println!(
            "INITIAL STATE: {:?} ({:?})",
            self.state.location, self.state.direction
        );
        for &m in self.moves.iter() {
            match m {
                Move::Steps(n) => self.state.step_cube(n, &self.tiles, cube_map),
                Move::TurnRight => self.state.turn_right(),
                Move::TurnLeft => self.state.turn_left(),
            }
        }
        println!(
            "FINAL STATE: {:?} ({:?})",
            self.state.location, self.state.direction
        );
    }
}

/// Reads the input file and returns the list of blueprints.
fn read_input(path: &str) -> Scenario {
    let file = File::open(path).expect("input file not found");
    let reader = BufReader::new(file);

    let mut rows: Vec<_> = reader
        .lines()
        .map(|l| l.expect("unable to read line"))
        .collect();

    let instructions = rows.pop().expect("instructions line is missing");
    assert!(rows.pop().expect("blank line is missing").is_empty());

    let mut tiles: Vec<Vec<_>> = rows
        .into_iter()
        .map(|l| {
            l.chars()
                .map(|c| Tile::new(c).expect("unknown character"))
                .collect()
        })
        .collect();
    let n_cols = tiles.iter().map(|r| r.len()).max().unwrap();
    for row in tiles.iter_mut() {
        for _ in 0..n_cols - row.len() {
            row.push(Tile::Air);
        }
    }

    let mut moves: Vec<Move> = Vec::new();
    for c in instructions.chars() {
        if c.is_numeric() {
            let mut n = c.to_digit(10).unwrap();
            if !moves.is_empty() && moves.last().unwrap().is_steps() {
                n += moves.pop().unwrap().get_steps().unwrap() * 10;
            }
            moves.push(Move::Steps(n));
        } else if c == 'R' {
            moves.push(Move::TurnRight);
        } else if c == 'L' {
            moves.push(Move::TurnLeft);
        } else {
            panic!("unknown movement")
        }
    }
    Scenario::new(tiles, moves)
}

fn password(scenario: &Scenario) -> i32 {
    let (col, row) = (scenario.state.location.0 + 1, scenario.state.location.1 + 1);
    let facing = match scenario.state.direction {
        Point(1, 0) => 0,
        Point(0, 1) => 1,
        Point(-1, 0) => 2,
        Point(0, -1) => 3,
        _ => panic!("unknown direction"),
    };
    1000 * row + 4 * col + facing
}

fn main() {
    let mut scenario = read_input("data/22_input.txt");
    scenario.exercise_1();
    println!("THE PASSWORD IS {}", password(&scenario));

    // For the second part, I need to create a hash map which key is
    // (previous face, previous direction) and the value is
    // (new face, new direction).
    // This dictionary varies depending on how you sort the faces
    let mut cube_map = HashMap::new();
    cube_map.insert(((1, 0), LEFT), ((0, 2), RIGHT));
    cube_map.insert(((1, 0), UP), ((0, 3), RIGHT));
    cube_map.insert(((2, 0), RIGHT), ((1, 2), LEFT));
    cube_map.insert(((2, 0), UP), ((0, 3), UP));
    cube_map.insert(((2, 0), DOWN), ((1, 1), LEFT));
    cube_map.insert(((1, 1), LEFT), ((0, 2), DOWN));
    cube_map.insert(((1, 1), RIGHT), ((2, 0), UP));
    cube_map.insert(((0, 2), LEFT), ((1, 0), RIGHT));
    cube_map.insert(((0, 2), UP), ((1, 1), RIGHT));
    cube_map.insert(((1, 2), RIGHT), ((2, 0), LEFT));
    cube_map.insert(((1, 2), DOWN), ((0, 3), LEFT));
    cube_map.insert(((0, 3), LEFT), ((1, 0), DOWN));
    cube_map.insert(((0, 3), RIGHT), ((1, 2), UP));
    cube_map.insert(((0, 3), DOWN), ((2, 0), DOWN));
    scenario.reset();
    scenario.exercise_2(&cube_map);
    println!("THE PASSWORD IS {}", password(&scenario));
}
