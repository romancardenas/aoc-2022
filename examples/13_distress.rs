use std::cmp::{Ord, Ordering};
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone)]
/// An element can either be a number or a list of elements.
enum Element {
    Num(usize),
    List(Vec<Element>),
}

impl Element {
    /// Adds an element to a list of elements.
    fn add_element(&mut self, element: Element) {
        match self {
            Element::Num(_) => panic!("cannot add an element to a number"),
            Element::List(v) => v.push(element),
        };
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Num(n) => {
                write!(f, "{}", n)
            }
            Element::List(l) => {
                write!(f, "{:?}", l)
            }
        }
    }
}

impl Ord for Element {
    /// Comparison as requested in the exercise.
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Element::Num(x) => match other {
                Element::Num(y) => x.cmp(y),
                Element::List(_) => Element::List(vec![Element::Num(*x)]).cmp(other),
            },
            Element::List(v) => match other {
                Element::Num(y) => self.cmp(&Element::List(vec![Element::Num(*y)])),
                Element::List(w) => v.cmp(w),
            },
        }
    }
}

impl PartialOrd<Self> for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Element {
    type Err = &'static str;

    /// This was the toughest part: parsing the file!
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        /// Auxiliary function to parse lists without inner lists
        fn parse_number_list(s: &str, elem: &mut Element) -> Result<(), &'static str> {
            let s_trim = s
                .trim()
                .trim_start_matches('[')
                .trim_end_matches(']')
                .trim();
            if s_trim.is_empty() {
                elem.add_element(Element::List(Vec::new()));
            } else {
                for n in s_trim.split(',') {
                    elem.add_element(Element::from_str(n.trim())?);
                }
            }
            Ok(())
        }

        // We trim the string just in case
        let s_trim = s.trim();
        // If string is empty, we return an error
        if s_trim.is_empty() {
            return Err("empty string");
        }
        // First, we try to cast it to a simple number.
        if let Ok(n) = s_trim.parse::<usize>() {
            return Ok(Element::Num(n));
        }
        // If we reach this, the result must be a list!
        let mut res = Element::List(Vec::new());
        // Let's first look for nested lists. First, we locate all the brackets
        let mut opening: Vec<usize> = s_trim
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '[')
            .map(|(i, _)| i)
            .collect();
        let mut closing: VecDeque<usize> = s_trim
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == ']')
            .map(|(i, _)| i)
            .collect();
        if opening.len() != closing.len() {
            return Err("number of opening and closing brackets do not match");
        }
        // Next, we identify all the immediate inner lists (i.e., those that belong to the result)
        let mut sublists: Vec<(usize, usize)> = Vec::new();
        // Closing brackets are resolved from front to back
        while let Some(close) = closing.pop_front() {
            // Its opening bracket is the one closest to the left to the closing bracket
            let (i, open) = opening
                .iter()
                .enumerate()
                .filter(|(_, c)| **c < close)
                .map(|(i, o)| (i, *o))
                .last()
                .unwrap();
            opening.remove(i); // We remove the ith opening bracket, as we found its pair
            if opening.is_empty() {
                // If we are done with opening brackets, we must be done with closing brackets
                if !closing.is_empty() {
                    return Err("brackets are not aligned");
                }
                break;
            }
            // We remove potential nested lists from sublists (we only consider immediate children)
            while let Some(prev) = sublists.pop() {
                if prev.0 < open {
                    sublists.push(prev); // Oops we went to far, let's insert it again.
                    break;
                }
            }
            sublists.push((open, close)); // We insert the new pair of brackets
        }
        // If we are done with closing brackets, we must be done with opening brackets
        if !opening.is_empty() {
            return Err("brackets are not aligned");
        }
        // If list does not contain sublists, we parse it as if it only had numbers
        if sublists.is_empty() {
            parse_number_list(s_trim, &mut res)?;
        } else {
            // Otherwise, we handle the rest as follows:
            let mut prev = (0, 0);
            for (start, stop) in sublists {
                // We first check if the gap between the previous list and this one contains numbers
                let gap = s_trim[prev.1 + 1..start]
                    .trim_matches(' ')
                    .trim_matches(',');
                if !gap.is_empty() {
                    parse_number_list(gap, &mut res)?;
                }
                // Then, we recursively create a new element and add it to the list
                match Element::from_str(&s_trim[start..stop + 1]) {
                    Ok(element) => {
                        res.add_element(element);
                    }
                    Err(e) => {
                        return Err(e);
                    }
                };
                prev = (start, stop);
            }
            // We also must check the tail of the string for numbers
            let tail = s_trim[prev.1 + 1..]
                .trim_end_matches(']')
                .trim_matches(' ')
                .trim_matches(',');
            if !tail.is_empty() {
                parse_number_list(tail, &mut res)?;
            }
        }
        Ok(res)
    }
}

/// Reads the input file and returns a scenario.
fn read_input(path: &str) -> Vec<(Element, Element)> {
    let mut res = Vec::new();
    let file = File::open(path).expect("input file not found");
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut elements: Vec<Element> = Vec::new();
    for line in lines.map(|l| l.expect("error parsing line")) {
        if line.is_empty() {
            assert_eq!(2, elements.len());
            let second = elements.pop().unwrap();
            let first = elements.pop().unwrap();
            res.push((first, second));
        } else {
            let elem = Element::from_str(&line).unwrap();
            // println!("{}", &elem);
            elements.push(elem);
        }
    }
    if !elements.is_empty() {
        assert_eq!(2, elements.len());
        let second = elements.pop().unwrap();
        let first = elements.pop().unwrap();
        res.push((first, second));
    }
    res
}

fn exercise_1(scenario: &[(Element, Element)]) -> usize {
    let mut res = 0;
    for (i, (first, second)) in scenario.iter().enumerate() {
        if first <= second {
            res += i + 1;
        }
    }
    res
}

fn exercise_2(scenario: &[(Element, Element)]) -> usize {
    let mut packets = Vec::new();
    let packet_2 = Element::List(vec![Element::List(vec![Element::Num(2)])]);
    let packet_6 = Element::List(vec![Element::List(vec![Element::Num(6)])]);
    packets.push(packet_2.clone());
    packets.push(packet_6.clone());
    for (first, second) in scenario.iter() {
        packets.push(first.clone());
        packets.push(second.clone());
    }
    packets.sort();
    let mut res = 1;
    for (i, packet) in packets.iter().enumerate() {
        if *packet == packet_2 || *packet == packet_6 {
            res *= i + 1;
        }
    }
    res
}

fn main() {
    let scenario = read_input("data/13_input.txt");
    println!("{}", exercise_1(&scenario));
    println!("{}", exercise_2(&scenario));
}
