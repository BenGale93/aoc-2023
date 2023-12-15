use std::{fs::read_to_string, path::Path};

use aoc_utils::Cli;

fn main() {
    let part_two = Cli::parse_args().part_two;

    let result = if part_two {
        focusing_power("input")
    } else {
        sequence_hash("input")
    };
    println!("Puzzle result: {result}");
}

fn sequence_hash(input: impl AsRef<Path>) -> usize {
    parse_puzzle(input).iter().map(|s| simple_hash(s)).sum()
}

fn simple_hash(value: &str) -> usize {
    let mut hash_value = 0;

    for v in value.bytes() {
        hash_value += v as usize;
        hash_value *= 17;
        hash_value %= 256;
    }
    hash_value
}

fn focusing_power(input: impl AsRef<Path>) -> usize {
    let instructions = parse_puzzle(input);

    let mut holiday_map = HolidayMap::new();

    for instruction in instructions {
        holiday_map.evaluate(&instruction);
    }

    holiday_map.power()
}

fn parse_puzzle(input: impl AsRef<Path>) -> Vec<String> {
    let input = read_to_string(input).unwrap();
    input.trim().split(',').map(|s| s.to_string()).collect()
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal: usize,
}

#[derive(Debug)]
struct HolidayMap {
    // Don't want to clash with the std HashMap...
    boxes: Vec<Vec<Lens>>,
}

impl HolidayMap {
    fn new() -> Self {
        Self {
            boxes: vec![Vec::new(); 256],
        }
    }

    fn evaluate(&mut self, instruction: &str) {
        if instruction.contains('-') {
            self.remove(instruction.strip_suffix('-').unwrap());
        } else {
            let instruction: Vec<_> = instruction.split('=').collect();
            let label = instruction.first().unwrap();
            let focal = instruction.last().unwrap().parse().unwrap();
            self.add(label.to_string(), focal);
        }
    }

    fn add(&mut self, label: String, focal: usize) {
        let hash = simple_hash(&label);
        let box_ = &mut self.boxes[hash];
        for lens in box_.iter_mut() {
            if lens.label == label {
                lens.focal = focal;
                return;
            }
        }

        box_.push(Lens { label, focal });
    }

    fn remove(&mut self, label: &str) {
        let hash = simple_hash(label);
        let box_ = &mut self.boxes[hash];
        for (i, lens) in box_.iter().enumerate() {
            if lens.label == label {
                box_.remove(i);
                return;
            }
        }
    }

    fn power(&self) -> usize {
        let mut power = 0;
        for (i, box_) in self.boxes.iter().enumerate() {
            for (j, len) in box_.iter().enumerate() {
                power += (i + 1) * (j + 1) * len.focal;
            }
        }
        power
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let result = sequence_hash("test_part1");
        assert_eq!(result, 1320);
    }

    #[test]
    fn hash_single_character() {
        let result = simple_hash("H");
        assert_eq!(result, 200);
    }

    #[test]
    fn hash_multiple_character() {
        let result = simple_hash("rn=1");
        assert_eq!(result, 30);
    }
}
