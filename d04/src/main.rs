use std::{collections::HashSet, path::Path};

use aoc_utils::{get_entire_puzzle, puzzle_input_lines, Cli};

fn main() {
    let part_two = Cli::parse_args().part_two;

    if part_two {
        let result = total_scratchcards("input");
        println!("Total scratchcards: {result}");
    } else {
        let result = scratchcards_value("input");
        println!("Scratchcard pile value is: {result}");
    }
}

fn number_of_matches(numbers: &[&str]) -> usize {
    let winning_numbers = &numbers.first().unwrap();
    let our_numbers = &numbers.last().unwrap();

    let mut winning_numbers: HashSet<_> = winning_numbers.split(' ').collect();
    winning_numbers.remove(&"");

    let mut our_numbers: HashSet<_> = our_numbers.split(' ').collect();
    our_numbers.remove(&"");

    let common_numbers: HashSet<_> = winning_numbers.intersection(&our_numbers).collect();

    common_numbers.len()
}

fn scratchcards_value(input: impl AsRef<Path>) -> u64 {
    let lines = puzzle_input_lines(input);

    lines
        .into_iter()
        .map(Result::unwrap)
        .map(|l| number_of_matches(&l.split(':').last().unwrap().split('|').collect::<Vec<_>>()))
        .filter(|m| *m != 0)
        .map(|m| u64::pow(2, u32::try_from(m - 1).unwrap()))
        .sum()
}

fn total_scratchcards(input: impl AsRef<Path>) -> u64 {
    let lines = get_entire_puzzle(input);

    let puzzle_length = lines.len();

    // Allocate a vector to keep track of the count of each card.
    let mut card_counts = vec![1; puzzle_length];

    for (card, line) in lines.iter().enumerate() {
        let numbers: Vec<_> = line.split(':').last().unwrap().split('|').collect();
        let matches = number_of_matches(&numbers);
        let current_card_count = { *card_counts.get(card).unwrap() };

        for i in card + 1..=card + matches {
            if let Some(c) = card_counts.get_mut(i) {
                *c += current_card_count;
            }
        }
    }

    card_counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let result = scratchcards_value("test_part1");
        assert_eq!(result, 13);
    }

    #[test]
    fn part_two() {
        let result = total_scratchcards("test_part1");
        assert_eq!(result, 30);
    }
}
