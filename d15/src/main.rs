use std::{fs::read_to_string, path::Path};

use aoc_utils::Cli;

fn main() {
    let part_two = Cli::parse_args().part_two;

    let result = if part_two {
        todo!()
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

fn parse_puzzle(input: impl AsRef<Path>) -> Vec<String> {
    let input = read_to_string(input).unwrap();
    input.trim().split(',').map(|s| s.to_string()).collect()
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
