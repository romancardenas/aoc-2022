use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

/// Struct for representing the scenario
#[derive(Debug)]
struct Scenario {
    /// Flows for each valve.
    flows: Vec<i32>,
    /// Neighboring valves (i.e., the edges of the graph)
    neighbors: Vec<Vec<usize>>,
}

impl Scenario {
    fn new(names: Vec<String>, flows: Vec<i32>, tunnels: Vec<Vec<String>>) -> Self {
        // First we parse the neighboring valves as if they were indices
        let mut neighbors: Vec<Vec<usize>> = Vec::new();
        for conn in tunnels.into_iter() {
            neighbors.push(
                conn.iter()
                    .map(|s| names.iter().position(|x| x == s).unwrap())
                    .collect(),
            );
        }
        Self { flows, neighbors }
    }

    fn best_path(
        &self,
        max_t: i32,      // maximum time before the volcano erupts
        n_people: usize, // number of people working on leaving pressure
        valve: usize,    // Current valve
        opened: usize,   // bit mask indicating which valves are opened
        t: i32,          // current time limit
        memo: &mut HashMap<(usize, usize, usize, i32), i32>, // handy cache
    ) -> i32 {
        // Break condition -> t is 0...
        if t == 0 {
            return match n_people {
                1 => 0, // ... and there's no people left
                // Otherwise, we check how the next person would perform
                n => self.best_path(max_t, n - 1, 0, opened, max_t, memo),
            };
        }
        let mem_key = (n_people, valve, opened, t);
        // Break condition: result is already in the cache
        if memo.contains_key(&mem_key) {
            return memo[&mem_key];
        }
        let mut best = 0;
        // If this valve is still closed, we explore how good would be that move
        let valve_closed = opened & (1 << valve) == 0;
        if valve_closed && self.flows[valve] > 0 {
            let new_opened = opened | (1 << valve);
            best = self.flows[valve] * (t - 1)
                + self.best_path(max_t, n_people, valve, new_opened, t - 1, memo);
        }
        // In any case, we also check how wise would be to move to neighboring valves
        for &other_valve in self.neighbors[valve].iter() {
            best = cmp::max(
                best,
                self.best_path(max_t, n_people, other_valve, opened, t - 1, memo),
            );
        }
        // Finally, we add the result to the cache and return the best result
        memo.insert(mem_key, best);
        best
    }
}

/// Reads the input file and returns the list of measurements.
fn read_input(path: &str) -> Scenario {
    let file = File::open(path).expect("input file not found");
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let re = Regex::new(
        r"^Valve ([A-Z]+) has flow rate=([0-9]+); tunnels? leads? to valves? ([A-Z,\s]+)",
    )
    .unwrap();

    let mut names = Vec::new();
    let mut flows = Vec::new();
    let mut conns: Vec<Vec<String>> = Vec::new();
    for line in lines.map(|l| l.expect("error parsing line")) {
        let cap = re.captures_iter(&line).next().unwrap();
        names.push(cap[1].to_string());
        flows.push(cap[2].parse::<i32>().unwrap());
        conns.push(cap[3].split(',').map(|s| s.trim().to_string()).collect());
    }
    Scenario::new(names, flows, conns)
}

fn exercise_1(scenario: &Scenario, t: i32) -> i32 {
    let mut memo = HashMap::new();
    scenario.best_path(t, 1, 0, 0, t, &mut memo)
}

fn exercise_2(scenario: &Scenario, t: i32) -> i32 {
    let mut memo = HashMap::new();
    scenario.best_path(t - 4, 2, 0, 0, t - 4, &mut memo)
}

fn main() {
    let scenario = read_input("data/16_input_test.txt");
    println!(
        "I can release {} pressure by myself",
        exercise_1(&scenario, 30)
    );
    println!(
        "I can release {} pressure with the elephant",
        exercise_2(&scenario, 30)
    );
}
