#![feature(iter_map_windows)]
use std::path::Path;

use aoc_utils::{get_entire_puzzle, Cli};
use rayon::prelude::*;

fn main() {
    let part_two = Cli::parse_args().part_two;

    let result = if part_two {
        oasis_sum_reverse("input")
    } else {
        oasis_sum("input")
    };
    println!("Puzzle result: {result}");
}

fn oasis_sum(input: impl AsRef<Path>) -> isize {
    get_entire_puzzle(input)
        .par_iter()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<isize>>()
        })
        .map(|l| next_value(&l))
        .sum()
}

fn oasis_sum_reverse(input: impl AsRef<Path>) -> isize {
    get_entire_puzzle(input)
        .par_iter()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<isize>>()
        })
        .map(|mut l| {
            l.reverse();
            l
        })
        .map(|l| next_value(&l))
        .sum()
}

fn next_value(line: &[isize]) -> isize {
    let mut lasts = vec![*line.last().unwrap()];
    let mut current_line = line.to_vec();
    while current_line.iter().map_windows(|&[a, b]| a != b).any(|x| x) {
        current_line = current_line.iter().map_windows(|&[a, b]| b - a).collect();
        lasts.push(*current_line.last().unwrap());
    }
    lasts.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_one() {
        let result = oasis_sum("test_part1");
        assert_eq!(result, 114);
    }

    #[test]
    fn part_two() {
        let result = oasis_sum_reverse("test_part1");
        assert_eq!(result, 2);
    }
}
