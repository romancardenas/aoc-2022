use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;
use std::thread;

/// Auxiliary structure for storing the number of resources and robots
type Resource = (usize, usize, usize, usize);

/// Structure that represents a point of the rope
#[derive(Debug, Clone)]
struct Blueprint {
    ore_robot: usize,
    clay_robot: usize,
    obsidian_robot: (usize, usize),
    geode_robot: (usize, usize),
}

impl Blueprint {
    /// It returns the maximum required number of ore that I may consume in a minute
    fn max_ore_per_minute(&self) -> usize {
        max(
            self.ore_robot,
            max(
                self.clay_robot,
                max(self.obsidian_robot.0, self.geode_robot.0),
            ),
        )
    }

    /// It returns the maximum required number of clay that I may consume in a minute
    fn max_clay_per_minute(&self) -> usize {
        self.obsidian_robot.1
    }

    /// It returns the maximum required number of obsidian that I may consume in a minute.
    fn max_obsidian_per_minute(&self) -> usize {
        self.geode_robot.1
    }

    /// Returns the maximum number of geodes that we can get given a time limit
    fn best_scenario(&self, max_t: usize) -> usize {
        let mut cache = HashMap::new();
        // Initially, we only have an ore robot
        self.best_scenario_mem((0, 0, 0, 0), (1, 0, 0, 0), max_t, &mut cache)
    }

    /// Analyzes the best scenario using memoization.
    fn best_scenario_mem(
        &self,
        resources: Resource,
        robots: Resource,
        t: usize,
        mem: &mut HashMap<(Resource, Resource, usize), usize>,
    ) -> usize {
        // Stop condition: we ran out of time. We return the number of geodes
        if t == 0 {
            return resources.3;
        }
        // Next we check if we already explored an scenario
        let (mut ore, mut clay, mut obsidian, geode) = resources;
        let (mut ore_r, mut clay_r, mut obsidian_r, geode_r) = robots;
        // To increase the number of cache hits, we limit the number of robots to the maximum needed
        ore_r = min(ore_r, self.max_ore_per_minute());
        clay_r = min(clay_r, self.max_clay_per_minute());
        obsidian_r = min(obsidian_r, self.max_obsidian_per_minute());
        // We also limit the number of resources. If we have plenty, we limit them to the maximum.
        if ore + ore_r * (t - 1) >= self.max_ore_per_minute() * t {
            ore = self.max_ore_per_minute() * t - ore_r * (t - 1);
        }
        if clay + clay_r * (t - 1) >= self.max_clay_per_minute() * t {
            clay = self.max_clay_per_minute() * t - clay_r * (t - 1);
        }
        if obsidian + obsidian_r * (t - 1) >= self.max_obsidian_per_minute() * t {
            obsidian = self.max_obsidian_per_minute() * t - obsidian_r * (t - 1);
        }
        let mem_key = (
            (ore, clay, obsidian, geode),
            (ore_r, clay_r, obsidian_r, geode_r),
            t,
        );
        // Stop condition: we have the result already cached
        if mem.contains_key(&mem_key) {
            return *mem.get(&mem_key).unwrap();
        }

        // By default, we assume that no robots are created in the next step.
        let next_n_resources = (
            ore + ore_r,
            clay + clay_r,
            obsidian + obsidian_r,
            geode + geode_r,
        );
        let mut best_score = self.best_scenario_mem(next_n_resources, robots, t - 1, mem);
        // We check what happens if we create an ore robot
        if ore >= self.ore_robot {
            let mut aux_resources = next_n_resources;
            aux_resources.0 -= self.ore_robot;
            let aux_robots = (ore_r + 1, clay_r, obsidian_r, geode_r);
            best_score = max(
                best_score,
                self.best_scenario_mem(aux_resources, aux_robots, t - 1, mem),
            );
        }
        // If we have enough ore, we check what happens if we create a clay robot
        if ore >= self.clay_robot {
            let mut aux_resources = next_n_resources;
            aux_resources.0 -= self.clay_robot;
            let aux_robots = (ore_r, clay_r + 1, obsidian_r, geode_r);
            best_score = max(
                best_score,
                self.best_scenario_mem(aux_resources, aux_robots, t - 1, mem),
            );
        }
        // If we have enough ore and clay, we check what happens if we create an obsidian robot
        if ore >= self.obsidian_robot.0 && clay >= self.obsidian_robot.1 {
            let mut aux_resources = next_n_resources;
            aux_resources.0 -= self.obsidian_robot.0;
            aux_resources.1 -= self.obsidian_robot.1;
            let aux_robots = (ore_r, clay_r, obsidian_r + 1, geode_r);
            best_score = max(
                best_score,
                self.best_scenario_mem(aux_resources, aux_robots, t - 1, mem),
            );
        }
        // If we have enough ore and obsidian, we check what happens if we create an geode robot
        if ore >= self.geode_robot.0 && obsidian >= self.geode_robot.1 {
            let mut aux_resources = next_n_resources;
            aux_resources.0 -= self.geode_robot.0;
            aux_resources.2 -= self.geode_robot.1;
            let aux_robots = (ore_r, clay_r, obsidian_r, geode_r + 1);
            best_score = max(
                best_score,
                self.best_scenario_mem(aux_resources, aux_robots, t - 1, mem),
            );
        }
        // Once we are done, we store the result in the cache and return the best score
        mem.insert(mem_key, best_score);
        best_score
    }
}

impl FromStr for Blueprint {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<_> = s
            .split_whitespace()
            .map(|c| c.parse::<usize>())
            .filter_map(|r| r.ok())
            .collect();
        match numbers.len() == 6 {
            true => Ok(Blueprint {
                ore_robot: numbers[0],
                clay_robot: numbers[1],
                obsidian_robot: (numbers[2], numbers[3]),
                geode_robot: (numbers[4], numbers[5]),
            }),
            false => Err("incorrect number of input numbers"),
        }
    }
}

/// Reads the input file and returns the list of blueprints.
fn read_input(path: &str) -> Vec<Blueprint> {
    let file = File::open(path).expect("input file not found");
    let reader = BufReader::new(file);
    let lines = reader.lines();
    lines
        .map(|l| Blueprint::from_str(&l.expect("unable to parse line")).unwrap())
        .collect()
}

/// It sums the the quality index of every blueprint
fn exercise_1(blueprints: &[Blueprint], max_t: usize) -> usize {
    // Blueprints are independent with each other, so I decided to explore them in parallel
    let mut handles = Vec::new();
    for blueprint in blueprints.iter() {
        let b = blueprint.clone();
        handles.push(thread::spawn(move || b.best_scenario(max_t)))
    }
    // We wait until all the threads finish and sum their quality level.
    let mut res = 0;
    for (i, geode) in handles.into_iter().map(|h| h.join().unwrap()).enumerate() {
        println!("Blueprint {} achieved {} geodes", i + 1, geode);
        res += (i + 1) * geode;
    }
    res
}

/// It multiplies the obtained geodes by all the blueprints
fn exercise_2(blueprints: &[Blueprint], max_t: usize) -> usize {
    // Blueprints are independent with each other, so I decided to explore them in parallel
    let mut handles = Vec::new();
    for blueprint in blueprints.iter() {
        let b = blueprint.clone();
        handles.push(thread::spawn(move || b.best_scenario(max_t)))
    }
    // We wait until all the threads finish and multiply the geodes achieved.
    let mut res = 1;
    for (i, geode) in handles.into_iter().map(|h| h.join().unwrap()).enumerate() {
        println!("Blueprint {} achieved {} geodes", i + 1, geode);
        res *= geode;
    }
    res
}

fn main() {
    let mut blueprints = read_input("data/19_input.txt");
    println!("PART 1: FINAL RESULT IS {}", exercise_1(&blueprints, 24));
    // For the second part, we just keep the three first lines
    if blueprints.len() > 3 {
        blueprints.drain(3..);
    }
    println!("PART 2: FINAL RESULT IS {}", exercise_2(&blueprints, 32));
}
