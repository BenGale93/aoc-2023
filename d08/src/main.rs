use std::{collections::HashMap, path::Path};

use aoc_utils::{get_entire_puzzle, Cli};
use num::integer::lcm;

fn main() {
    let part_two = Cli::parse_args().part_two;

    let result = if part_two {
        simultaneous_step_count("input")
    } else {
        step_count("input")
    };
    println!("Step count: {result}");
}

fn step_count(input: impl AsRef<Path>) -> usize {
    let (instructions, network) = parse_puzzle(input);

    let mut element = "AAA".to_string();
    let mut steps = 1;
    for i in instructions.iter().cycle() {
        let values = network.get(&element).unwrap();
        element = if *i {
            values.0.to_owned()
        } else {
            values.1.to_owned()
        };
        if element == "ZZZ" {
            return steps;
        }
        steps += 1;
    }
    usize::MAX
}

fn simultaneous_step_count(input: impl AsRef<Path>) -> usize {
    let (instructions, network) = parse_puzzle(input);

    let mut elements: Vec<(&String, Option<usize>)> = network
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| (k, None))
        .collect();

    let mut steps: usize = 1;
    for i in instructions.into_iter().cycle() {
        for element in &mut elements {
            if element.1.is_some() {
                continue;
            }
            let values = network.get(element.0).unwrap();
            let next_element = if i { &values.0 } else { &values.1 };
            let z = if next_element.ends_with('Z') {
                Some(steps)
            } else {
                None
            };
            *element = (next_element, z);
        }
        if elements.iter().filter(|e| e.1.is_some()).count() == elements.len() {
            break;
        }
        steps += 1;
    }

    elements.iter().map(|e| e.1.unwrap()).reduce(lcm).unwrap()
}

fn parse_puzzle(input: impl AsRef<Path>) -> (Vec<bool>, HashMap<String, (String, String)>) {
    let puzzle = get_entire_puzzle(input);

    let instructions = puzzle.first().unwrap();
    let instructions: Vec<bool> = instructions.chars().map(|c| c == 'L').collect();

    let mut network = HashMap::new();

    for line in puzzle.iter().skip(2) {
        let line: Vec<&str> = line.split(" = ").collect();
        let key = line.first().unwrap();
        let values: Vec<&str> = line
            .last()
            .unwrap()
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split(", ")
            .collect();
        let values = (
            values.first().unwrap().to_string(),
            values.last().unwrap().to_string(),
        );
        network.insert(key.to_string(), values);
    }

    (instructions, network)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_one_one() {
        let result = step_count("test1_part1");
        assert_eq!(result, 2);
    }

    #[test]
    fn part_one_two() {
        let result = step_count("test2_part1");
        assert_eq!(result, 6);
    }

    #[test]
    fn part_two() {
        let result = simultaneous_step_count("test_part2");
        assert_eq!(result, 6);
    }
}
