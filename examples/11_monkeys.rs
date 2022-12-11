use std::fs::File;
use std::io::{prelude::*, BufReader};

/// Auxiliary structure for worry operations used by the monkeys
#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(usize),
    Multiply(usize),
    Exponential,
}

impl Operation {
    /// It computes the new worry level depending on the operation.
    fn new_worry(&self, old: usize) -> usize {
        match self {
            Operation::Add(x) => old + x,
            Operation::Multiply(x) => old * x,
            Operation::Exponential => old * old,
        }
    }
}

/// Structure representing a monkey.
#[derive(Debug, Clone)]
struct Monkey {
    /// Vector of objects with its worry level that the monkey currently holds.
    objects: Vec<usize>,
    /// Worry operation used by the monkey.
    operation: Operation,
    /// Monkey checks if the worry level is divisible by this number to send it to other monkey.
    divisible: usize,
    /// ID of the monkey that receives an object if its worry level is divisible.
    if_true: usize,
    /// ID of the monkey that receives an object if its worry level is not divisible.
    if_false: usize,
    /// Number of times a monkey checked an object.
    n_times: usize,
}

impl Monkey {
    /// Triggers the turn of the monkey.
    /// If `relaxed` is set to `true`, the worry level is divided by 3.
    /// It returns a vector of pairs (worry level of object, ID of the monkey receiving the object).
    fn turn(&mut self, relaxed: bool) -> Vec<(usize, usize)> {
        // n times increments by the number of objects that the monkey is holding.
        self.n_times += self.objects.len() as usize;
        let mut res = Vec::new();
        for old_worry in self.objects.iter() {
            // We update the worry level associated to each object.
            let mut new_worry = self.operation.new_worry(*old_worry);
            if relaxed {
                // If I'm relaxed, I divide the worry level by 3.
                new_worry /= 3;
            }
            // We determine which monkey will receive the object and push it to the result vector
            let monkey = match new_worry % self.divisible == 0 {
                true => self.if_true,
                false => self.if_false,
            };
            res.push((new_worry, monkey));
        }
        self.objects.clear(); // We clear all the objects before exiting!
        res
    }
}

/// A scenario for this exercise.
#[derive(Debug, Clone)]
struct Scenario {
    /// Vector of all the monkeys in the scenario.
    monkeys: Vec<Monkey>,
}

impl Scenario {
    /// Creates a new, empty scenario.
    fn new() -> Self {
        Self {
            monkeys: Vec::new(),
        }
    }

    /// Adds a monkey to the scenario.
    fn add_monkey(&mut self, monkey: Monkey) {
        self.monkeys.push(monkey);
    }

    /// Executes a single round of monkey stuff.
    /// LCM is the least common multiplicator of all the monkeys.
    /// It is required to keep the numbers manageable when I'm not relaxed.
    fn round(&mut self, lcm: Option<usize>) {
        for i in 0..self.monkeys.len() {
            let monkey = &mut self.monkeys[i];
            // Recall that if lcm is None, then I'm relaxed!
            for (mut object, other) in monkey.turn(lcm.is_none()) {
                object %= lcm.unwrap_or(object + 1);
                self.monkeys[other].objects.push(object);
            }
        }
    }

    /// Executes n consecutive rounds. You can also indicate whether you are relaxed or not.
    fn rounds(&mut self, n: usize, relaxed: bool) {
        // We first compute lcm
        let lcm = match relaxed {
            true => None, // If I'm relaxed, lcm is not required
            false => {
                // Otherwise, lcm is the product of all the divisible values of all the monkeys.
                Some(self.monkeys.iter().map(|m| m.divisible).product())
            }
        };
        // Then, we execute the rounds.
        for i in 1..n + 1 {
            self.round(lcm);
            // We log some results from time to time
            if i == 1 || i <= 20 && i % 5 == 0 || i % 1000 == 0 {
                println!("== After round {i} ==");
                self.monkey_business();
                println!();
            }
        }
    }

    /// It computes the monkey business score.
    fn monkey_business(&self) -> usize {
        // We just have to find the two highest n_times within all the monkeys.
        let (mut first, mut second) = (0, 0);
        for (i, monkey) in self.monkeys.iter().enumerate() {
            println!("Monkey {} inspected items {} times", i, monkey.n_times);
            if monkey.n_times > first {
                second = first;
                first = monkey.n_times;
            } else if monkey.n_times > second {
                second = monkey.n_times;
            }
        }
        first * second
    }
}

/// Reads the input file and returns a scenario with the initial configuration of the monkeys.
fn read_input(path: &str) -> Scenario {
    let mut scenario = Scenario::new();
    let file = File::open(path).expect("input file not found");
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let x: Vec<String> = lines.map(|l| l.expect("error parsing line")).collect();
    for i in (0..x.len()).step_by(7) {
        // First, we parse the monkey objects
        let aux: Vec<&str> = x[i + 1].split(&[':', ',']).collect();
        let objects: Vec<usize> = aux
            .iter()
            .map(|s| (*s).trim().parse::<usize>())
            .filter(|c| c.is_ok())
            .map(|c| c.expect(""))
            .collect();
        // Next, we parse the operation
        let aux: Vec<&str> = x[i + 2].split_whitespace().collect();
        let operation = match aux[4] {
            "+" => Operation::Add(aux[5].parse::<usize>().expect("error parsing addition")),
            "*" => match aux[5] {
                "old" => Operation::Exponential,
                _ => Operation::Multiply(
                    aux[5]
                        .parse::<usize>()
                        .expect("error parsing multiplication"),
                ),
            },
            _ => panic!("unknown operation"),
        };
        // Next, we parse the division factor
        let aux: Vec<&str> = x[i + 3].split_whitespace().collect();
        let divisible = aux[3].parse::<usize>().expect("error parsing divisible");
        // Next, the if_true monkey
        let aux: Vec<&str> = x[i + 4].split_whitespace().collect();
        let if_true = aux[5]
            .parse::<usize>()
            .expect("error parsing if_true monkey");
        // Finally, the if_false monkey
        let aux: Vec<&str> = x[i + 5].split_whitespace().collect();
        let if_false = aux[5]
            .parse::<usize>()
            .expect("error parsing if_true monkey");
        // We add a new monkey to the scenario
        scenario.add_monkey(Monkey {
            objects,
            operation,
            divisible,
            if_true,
            if_false,
            n_times: 0,
        })
    }
    scenario
}

fn main() {
    // First we read the input file.
    let mut scenario_1 = read_input("data/11_input.txt");
    let mut scenario_2 = scenario_1.clone();
    // Exercise 1: I'm relaxed
    scenario_1.rounds(20, true);
    println!("Monkey business: {}", scenario_1.monkey_business());
    println!();
    // Exercise 2: I'm NOT relaxed
    scenario_2.rounds(10000, false);
    println!("Monkey business: {}", scenario_2.monkey_business());
}
