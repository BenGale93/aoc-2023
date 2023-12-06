use std::path::Path;

use aoc_utils::{get_entire_puzzle, Cli};

fn main() {
    let part_two = Cli::parse_args().part_two;

    let result = if part_two {
        race_records_part2("input")
    } else {
        race_records("input")
    };
    println!("Race record product: {result}");
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

fn parse_line_part2(puzzle_line: &str) -> u64 {
    let number_fragments: Vec<&str> = puzzle_line
        .split(':')
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .collect();

    number_fragments.join("").parse().unwrap()
}

fn number_winning_times(time: u64, distance: u64) -> u64 {
    let time = time as f64;
    let distance = distance as f64;

    let discriminant = (time.powf(2.0) - 4.0 * distance).sqrt();

    let lower = (-time + discriminant) / (-2.0);
    let lower_ceiling = lower.ceil();

    // Need to account for draws
    let lower = if lower == lower_ceiling {
        lower_ceiling + 1.0
    } else {
        lower_ceiling
    };

    let upper = (-time - discriminant) / -2.0;
    let upper_floor = upper.floor();

    // Need to account for draws
    let upper = if upper == upper_floor {
        upper_floor - 1.0
    } else {
        upper_floor
    };

    (upper - lower + 1.0) as u64
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

fn race_records_part2(input: impl AsRef<Path>) -> u64 {
    let puzzle = get_entire_puzzle(input);

    let time = parse_line_part2(puzzle.first().unwrap());
    let distance = parse_line_part2(puzzle.get(1).unwrap());

    number_winning_times(time, distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let result = race_records("test_part1");
        assert_eq!(result, 288);
    }

    #[test]
    fn part_two() {
        let result = race_records_part2("test_part1");
        assert_eq!(result, 71503);
    }
}
