use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Monkey {
    Num(i64),
    Op((String, String, String)),
}

impl Monkey {
    fn apply_op(lhs: i64, op: &str, rhs: i64) -> i64 {
        match op {
            "+" => lhs + rhs,
            "-" => lhs - rhs,
            "*" => lhs * rhs,
            "/" => lhs / rhs,
            _ => panic!("unknown operation"),
        }
    }

    fn inverse(op: &str) -> &'static str {
        match op {
            "+" => "-",
            "-" => "+",
            "*" => "/",
            "/" => "*",
            _ => panic!("unknown operation"),
        }
    }
}

impl FromStr for Monkey {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chunks: Vec<_> = s.trim().split(' ').collect();
        match chunks.len() {
            1 => match chunks[0].parse::<i64>() {
                Ok(n) => Ok(Self::Num(n)),
                Err(_) => Err("unable to parse number as i64"),
            },
            3 => match chunks[1] {
                "+" | "-" | "*" | "/" => Ok(Self::Op((
                    chunks[0].to_string(),
                    chunks[1].to_string(),
                    chunks[2].to_string(),
                ))),
                _ => Err("unknown operation"),
            },
            _ => Err("incorrect number of parameters"),
        }
    }
}

#[derive(Debug, Clone)]
enum Res {
    Unknown,
    Num(i64),
    Op(Box<Node>, String, Box<Node>),
}

#[derive(Debug, Clone)]
struct Node {
    id: String,
    val: Res,
}

impl Node {
    fn new(monkeys: &HashMap<String, Monkey>, id: &str, human: Option<&str>) -> Self {
        if human.is_some() && id == human.unwrap() {
            return Self {
                id: id.to_string(),
                val: Res::Unknown,
            };
        }
        let monkey = monkeys.get(id).expect("monkey not found");
        match monkey {
            Monkey::Num(n) => Self {
                id: id.to_string(),
                val: Res::Num(*n),
            },
            Monkey::Op((lhs, op, rhs)) => {
                let lnode = Box::new(Self::new(monkeys, lhs, human));
                let rnode = Box::new(Self::new(monkeys, rhs, human));
                Self {
                    id: id.to_string(),
                    val: Res::Op(lnode, op.clone(), rnode),
                }
            }
        }
    }

    fn fill_val_cache(&self, cache: &mut HashMap<String, i64>) -> Option<i64> {
        if let Some(&n) = cache.get(&self.id) {
            return Some(n);
        }
        let res = match &self.val {
            Res::Unknown => None,
            Res::Num(n) => Some(*n),
            Res::Op(lhs, op, rhs) => {
                let lval = lhs.fill_val_cache(cache);
                let rval = rhs.fill_val_cache(cache);
                match lval.is_some() && rval.is_some() {
                    true => Some(Monkey::apply_op(lval.unwrap(), op, rval.unwrap())),
                    false => None,
                }
            }
        };
        if let Some(n) = res {
            cache.insert(self.id.clone(), n);
        }
        res
    }

    fn solve(&self, res: i64, cache: &mut HashMap<String, i64>) -> Result<i64, ()> {
        if let Some(&val) = cache.get(&self.id) {
            return match val == res {
                true => Ok(res),
                false => Err(()),
            };
        }
        match &self.val {
            Res::Unknown => Ok(res),
            Res::Num(n) => match *n == res {
                true => Ok(res),
                false => Err(()),
            },
            Res::Op(lhs, op, rhs) => {
                let (lval, rval) = (cache.get(&lhs.id), cache.get(&rhs.id));
                match lval {
                    None => match rval {
                        None => Err(()),
                        Some(&r) => {
                            let new_res = Monkey::apply_op(res, Monkey::inverse(op), r);
                            lhs.solve(new_res, cache)
                        }
                    },
                    Some(&l) => match rval {
                        None => {
                            let new_res = match op.as_str() {
                                "/" | "-" => Monkey::apply_op(l, op, res),
                                _ => Monkey::apply_op(res, Monkey::inverse(op), l),
                            };
                            rhs.solve(new_res, cache)
                        }
                        Some(&r) => match Monkey::apply_op(l, op, r) == res {
                            true => Ok(res),
                            false => Err(()),
                        },
                    },
                }
            }
        }
    }
}

/// Reads the input file and returns the list of blueprints.
fn read_input(path: &str) -> HashMap<String, Monkey> {
    let file = File::open(path).expect("input file not found");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|l| {
            let l_ok = l.expect("unable to read line");
            let aux: Vec<_> = l_ok.split(':').collect();
            (
                aux[0].to_string(),
                Monkey::from_str(aux[1]).expect("unable to parse monkey"),
            )
        })
        .collect()
}

fn exercise_1(monkeys: &HashMap<String, Monkey>, target: &str) -> Option<i64> {
    let nodes = Node::new(monkeys, target, None);
    let mut cache = HashMap::new();
    nodes.fill_val_cache(&mut cache);
    cache.get(target).copied()
}

fn exercise_2(monkeys: &HashMap<String, Monkey>, target: &str, human: &str) -> Result<i64, ()> {
    let nodes = Node::new(monkeys, target, Some(human));
    let mut cache = HashMap::new();
    nodes.fill_val_cache(&mut cache);
    match nodes.val {
        Res::Unknown => Err(()),
        Res::Num(n) => Ok(n),
        Res::Op(lhs, _, rhs) => {
            let lval = cache.get(&lhs.id);
            let rval = cache.get(&rhs.id);
            match lval {
                None => match rval {
                    None => Err(()),
                    Some(&r) => lhs.solve(r, &mut cache),
                },
                Some(&l) => match rval {
                    None => rhs.solve(l, &mut cache),
                    Some(_) => Err(()),
                },
            }
        }
    }
}

fn main() {
    let monkeys = read_input("data/21_input.txt");
    println!("{:?}", exercise_1(&monkeys, "root"));
    println!("{:?}", exercise_2(&monkeys, "root", "humn"));
}
