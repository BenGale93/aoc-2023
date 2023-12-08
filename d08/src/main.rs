use std::{collections::HashMap, path::Path};

use aoc_utils::{get_entire_puzzle, Cli};

fn main() {
    let part_two = Cli::parse_args().part_two;

    if part_two {
        todo!();
    } else {
        let result = step_count("input");
        println!("Step count: {result}");
    };
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
        assert!(true);
    }
}
