use std::path::Path;

use aho_corasick::AhoCorasick;
use aoc_utils::{puzzle_input_lines, Cli};

fn main() {
    let cli = Cli::parse_args();

    let patterns = if cli.part_two {
        &PATTERNS
    } else {
        &PATTERNS[..9]
    };

    let result = calibration_value("input", patterns);
    println!("Calibration value is: {result}");
}

const PATTERNS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

fn calibration_value<P: AsRef<Path>>(input: P, patterns: &[&str]) -> u64 {
    let lines = puzzle_input_lines(input);
    let ac = AhoCorasick::new(patterns).unwrap();

    let mut total = 0;
    for line in lines {
        let line = line.unwrap();
        let matches: Vec<&str> = ac
            .find_overlapping_iter(&line)
            .map(|mat| &line[mat.range()])
            .collect();
        let value = get_first_last(&matches);
        total += value;
    }
    total
}

fn convert_natural_english_number(input: &str) -> &str {
    match input {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        i => i,
    }
}

fn get_first_last(input: &[&str]) -> u64 {
    let mut first = convert_natural_english_number(input.first().unwrap()).to_string();
    let last = convert_natural_english_number(input.last().unwrap());

    first.push_str(last);

    first.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let result = calibration_value("test_part1", &PATTERNS[..9]);
        assert_eq!(result, 142);
    }

    #[test]
    fn part_two() {
        let result = calibration_value("test_part2", &PATTERNS);
        assert_eq!(result, 281);
    }
}
