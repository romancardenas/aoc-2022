use std::cmp;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Clone, Eq, PartialEq)]
/// Wrapper of a candidate node for the A* algorithm.
struct State {
    /// Candidate node
    node: (usize, usize),
    /// Associated cost.
    cost: usize,
}

impl State {
    /// Creates a freshly new candidate path.
    fn new(node: (usize, usize), cost: usize) -> Self {
        Self { node, cost }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
/// My simulation scenario.
struct Scenario {
    /// Pixel map of the place. The height is represented with usize numbers.
    map: Vec<Vec<usize>>,
    /// Coordinates of the starting point.
    start: (usize, usize),
    /// Coordinates of the exit point.
    exit: (usize, usize),
}

impl Scenario {
    /// Returns the number of rows in the map.
    fn n_rows(&self) -> usize {
        self.map.len()
    }

    /// Returns the number of columns in the map.
    fn n_cols(&self) -> usize {
        self.map[0].len()
    }

    /// Checks if a given cell is in the map.
    fn in_map(&self, node: (usize, usize)) -> bool {
        node.0 < self.n_rows() && node.1 < self.n_cols()
    }

    /// It computes the Manhattan distance between two locations.
    fn manhattan(from: (usize, usize), to: (usize, usize)) -> usize {
        cmp::max(from.0, to.0) - cmp::min(from.0, to.0) + cmp::max(from.1, to.1)
            - cmp::min(from.1, to.1)
    }

    /// It computes the distance between two cells.
    /// If you cannot move from `from` to `to`, it returns `None`.
    /// If `from` and `to` are the same, it returns `Some(0)`.
    /// Otherwise, it returns `Some(1)` (you only need to move one step).
    fn distance(&self, from: (usize, usize), to: (usize, usize)) -> Option<usize> {
        // First we check that the cells are part of the map
        if !self.in_map(from) || !self.in_map(to) {
            return None;
        }
        match Self::manhattan(from, to) {
            0 => Some(0), // It is the same cell! distance is 0
            1 => {
                // Cells are adjacent, let's see the heights
                let from = self.map[from.0][from.1];
                let to = self.map[to.0][to.1];
                // Destination cell must be 1 step higher or less!
                match from >= to || to - from < 2 {
                    true => Some(1),
                    false => None,
                }
            }
            _ => None, // Cells are not adjacent, distance does not apply
        }
    }

    /// Generates the von Neumann neighborhood of a node.
    fn neighbors(&self, node: (usize, usize)) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        if self.in_map(node) {
            if node.0 > 0 {
                res.push((node.0 - 1, node.1));
            }
            if node.0 < self.n_rows() - 1 {
                res.push((node.0 + 1, node.1))
            }
            if node.1 > 0 {
                res.push((node.0, node.1 - 1));
            }
            if node.1 < self.n_cols() - 1 {
                res.push((node.0, node.1 + 1))
            }
        }
        res
    }

    /// A* algorithm for moving from `from` to `to`.
    /// If you can move from `from` to `to`, it returns a vector with the best path.
    /// Otherwise, it return `None`.
    fn a_star(&self, from: (usize, usize), to: (usize, usize)) -> Option<Vec<(usize, usize)>> {
        // Origin and destination must be part of the scenario.
        if !self.in_map(from) || !self.in_map(to) {
            return None;
        }
        // Open set is a priority list.
        let mut open_set = BinaryHeap::new();
        open_set.push(State::new(from, 0)); // We start from the origin!
                                            // For node n, the previous best step is path[n]. It is useful to reconstruct the best path.
        let mut path = HashMap::new();
        // Record of the best score for reaching a given node in the map.
        let mut score = HashMap::new();
        score.insert(from, 0); // the cost to go to the origin is 0!
                               // Now, we evaluate iteratively the shortest buffered path.
        while let Some(current) = open_set.pop() {
            // If the shortest buffered path goes to the destination... we are done!
            if current.node == to {
                // We reconstruct the best path from destination to origin...
                let mut res = Vec::new();
                let mut current = to;
                while current != from {
                    res.push(current);
                    current = path[&current];
                }
                res.push(from);
                // and reverse the vector before returning it
                res.reverse();
                return Some(res);
            }
            // Otherwise, we explore all the potential neighbors of the current node
            for neighbor in self.neighbors(current.node) {
                // We first check that the neighbor is actually reachable according to the distance function.
                if let Some(distance) = self.distance(current.node, neighbor) {
                    let new_cost = current.cost + distance;
                    let prev_cost = *score.get(&neighbor).unwrap_or(&usize::MAX);
                    // If the new cost is less than the previous, we record this path
                    if new_cost < prev_cost {
                        score.insert(neighbor, new_cost);
                        path.insert(neighbor, current.node);
                        open_set.push(State::new(neighbor, new_cost));
                    }
                }
            }
        }
        None
    }

    /// Function to find the best hiking route (exercise 2)
    fn best_hiking(&self) -> Option<Vec<(usize, usize)>> {
        let mut best: Option<Vec<(usize, usize)>> = None;
        for i in 0..self.n_rows() {
            for j in 0..self.n_cols() {
                if self.map[i][j] == 0 {
                    // We only explore origin cells with a height of 'a'
                    if let Some(v) = self.a_star((i, j), self.exit) {
                        // If the A* algorithm finds an optimal path, we check if this is a new best
                        best = match best {
                            Some(prev) => match prev.len() <= v.len() {
                                true => Some(prev),
                                false => Some(v),
                            },
                            None => Some(v),
                        };
                    }
                }
            }
        }
        best
    }
}

/// Reads the input file and returns a scenario.
fn read_input(path: &str) -> Scenario {
    let mut map = Vec::new();
    let mut start = (0, 0);
    let mut exit = (0, 0);

    let file = File::open(path).expect("input file not found");
    let reader = BufReader::new(file);
    let lines = reader.lines();
    for (i, line) in lines.map(|l| l.expect("error parsing line")).enumerate() {
        let mut row = Vec::new(); // Each line is a row of the map
        for (j, c) in line.chars().enumerate() {
            if c.is_ascii_lowercase() {
                // lowercase imply a height
                row.push(c as usize - 'a' as usize);
            } else if c.is_ascii_uppercase() {
                if c == 'S' {
                    // Starting point (height is 'a')
                    start = (i, j);
                    row.push(0);
                } else if c == 'E' {
                    // Exit point (height is 'z')
                    exit = (i, j);
                    row.push('z' as usize - 'a' as usize);
                } else {
                    panic!("unknown character: {}", c);
                }
            }
        }
        map.push(row);
    }
    Scenario { map, start, exit }
}

fn main() {
    // First we read the input file.
    let scenario = read_input("data/12_input.txt");
    println!(
        "Scenario dimensions {} by {}",
        scenario.n_rows(),
        scenario.n_cols()
    );
    println!("I start in {:?}", &scenario.start);
    println!("I exit in {:?}", &scenario.exit);
    println!();
    // Exercise 1: best path form start to exit -> we use A* algorithm
    let best_path = scenario.a_star(scenario.start, scenario.exit);
    println!("Best path: {:?}", &best_path);
    if let Some(best) = best_path {
        println!("We need to move {} times", best.len() - 1);
    }
    println!();
    // Exercise 2: best hiking path from an 'a' location
    let best_hiking = scenario.best_hiking();
    println!("Best hiking: {:?}", &best_hiking);
    if let Some(best) = best_hiking {
        println!("We need to move {} times", best.len() - 1);
    }
}
