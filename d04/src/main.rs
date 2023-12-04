use std::{collections::HashSet, path::Path};

use aoc_utils::{puzzle_input_lines, Cli};

fn main() {
    let part_two = Cli::parse_args().part_two;

    if part_two {
        todo!()
    } else {
        let result = scratchcards_value("input");
        println!("Scratchcard pile value is: {result}");
    }
}

fn scratchcards_value(input: impl AsRef<Path>) -> u64 {
    let lines = puzzle_input_lines(input);

    let mut total = 0;
    for line in lines {
        let line = line.unwrap();

        let numbers: Vec<_> = line.split(':').last().unwrap().split('|').collect();

        let winning_numbers = numbers.first().unwrap();
        let our_numbers = numbers.last().unwrap();

        let mut winning_numbers: HashSet<_> = winning_numbers.split(' ').collect();
        winning_numbers.remove(&"");

        let mut our_numbers: HashSet<_> = our_numbers.split(' ').collect();
        our_numbers.remove(&"");

        let common_numbers: HashSet<_> = winning_numbers.intersection(&our_numbers).collect();

        let matches = common_numbers.len();

        if matches != 0 {
            total += u64::pow(2, (matches - 1) as u32);
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let result = scratchcards_value("test_part1");
        assert_eq!(result, 13);
    }
}
