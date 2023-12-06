use std::path::Path;

use aoc_utils::{get_entire_puzzle, Cli};

fn main() {
    let part_two = Cli::parse_args().part_two;

    if part_two {
        todo!()
    } else {
        let result = race_records("input");
        println!("Race record product: {result}");
    }
}

fn parse_line(puzzle_line: &str) -> Vec<u64> {
    puzzle_line
        .split(':')
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn number_winning_times(time: u64, distance: u64) -> u64 {
    (1..time)
        .map(|held| (time - held) * held)
        .filter(|d| d > &distance)
        .count() as u64
}

fn race_records(input: impl AsRef<Path>) -> u64 {
    let puzzle = get_entire_puzzle(input);

    let time: Vec<u64> = parse_line(puzzle.first().unwrap());
    let distance: Vec<u64> = parse_line(puzzle.get(1).unwrap());

    time.iter()
        .zip(distance)
        .map(|(t, d)| number_winning_times(*t, d))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let result = race_records("test_part1");
        assert_eq!(result, 288);
    }
}
