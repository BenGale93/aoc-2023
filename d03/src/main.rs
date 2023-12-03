use std::{ops::Range, path::Path};

use aoc_utils::{get_entire_puzzle, Cli};
use regex::Regex;

fn main() {
    let part_two = Cli::parse_args().part_two;

    if part_two {
        let result = gear_ratio_sum("input");
        println!("Gear ratio number sum is: {result}");
    } else {
        let result = part_number_sum("input");
        println!("Engine part number sum is: {result}");
    }
}

fn part_number_sum(input: impl AsRef<Path>) -> u64 {
    let digits = Regex::new(r"\d+").unwrap();
    let symbols = Regex::new(r"[^.^\d]").unwrap();

    let engine_schema = get_entire_puzzle(input);
    let puzzle_width = engine_schema[0].len();
    let puzzle_length = engine_schema.len();

    let mut total = 0;
    let padding = ".".repeat(puzzle_width);

    for (i, row) in engine_schema.iter().enumerate() {
        let number_matches: Vec<_> = digits.find_iter(row).collect();

        for number_match in number_matches {
            let number_range = {
                let start = number_match.start().saturating_sub(1);
                let end = puzzle_width.min(number_match.end() + 1);
                start..end
            };
            let row_range = {
                let row_above = i.saturating_sub(1);
                let row_below = puzzle_length.min(i + 1);
                row_above..=row_below
            };

            for j in row_range {
                let test_row = engine_schema.get(j).unwrap_or(&padding);
                let symbol_matches: Vec<_> =
                    symbols.find_iter(&test_row[number_range.clone()]).collect();
                if symbol_matches.is_empty() {
                    continue;
                }
                let valid_number: u64 = number_match.as_str().parse().unwrap();
                total += valid_number;
                break;
            }
        }
    }

    total
}

fn gear_ratio_sum(input: impl AsRef<Path>) -> u64 {
    let digits = Regex::new(r"\d+").unwrap();
    let gear = Regex::new(r"[*]").unwrap();

    let engine_schema = get_entire_puzzle(input);
    let puzzle_width = engine_schema[0].len();
    let puzzle_length = engine_schema.len();

    let mut total = 0;
    let padding = ".".repeat(puzzle_width);

    for (i, row) in engine_schema.iter().enumerate() {
        let gear_matches: Vec<_> = gear.find_iter(row).collect();

        for gear_match in gear_matches {
            let gear_range = {
                let start = gear_match.start().saturating_sub(1);
                let end = puzzle_width.min(gear_match.start() + 1);
                start..end
            };

            let row_range = {
                let row_above = i.saturating_sub(1);
                let row_below = puzzle_length.min(i + 1);
                row_above..=row_below
            };

            let mut valid_digits: Vec<String> = vec![];
            for j in row_range {
                let test_row = engine_schema.get(j).unwrap_or(&padding);
                for digit_match in digits.find_iter(test_row) {
                    if overlapping(&digit_match.range(), &gear_range) {
                        valid_digits.push(digit_match.as_str().to_string());
                    }
                }
            }

            if valid_digits.len() == 2 {
                let first: u64 = valid_digits.first().unwrap().parse().unwrap();
                let second: u64 = valid_digits.last().unwrap().parse().unwrap();
                total += first * second;
            }
        }
    }

    total
}

fn overlapping(digit_range: &Range<usize>, gear_range: &Range<usize>) -> bool {
    digit_range.start <= gear_range.end && gear_range.start < digit_range.end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let result = part_number_sum("test_part1");
        assert_eq!(result, 4361);
    }

    #[test]
    fn part_two() {
        let result = gear_ratio_sum("test_part1");
        assert_eq!(result, 467835);
    }
}
