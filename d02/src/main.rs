use core::panic;
use std::{io::BufRead, path::Path};

use aoc_utils::input_buffer_reader;

fn main() {
    let result = cube_conundrum("input");
    println!("Cube game value is: {result}")
}

struct GameSubset {
    red: u64,
    green: u64,
    blue: u64,
}

impl GameSubset {
    fn new(red: u64, green: u64, blue: u64) -> Self {
        Self { red, green, blue }
    }

    fn parse(input: &str) -> Self {
        let fragments: Vec<&str> = input.split(',').collect();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for fragment in fragments {
            let result: Vec<&str> = fragment.trim().split(' ').collect();
            let num: u64 = result.first().unwrap().parse().unwrap();
            let colour = result.last().unwrap();
            match *colour {
                "red" => red = num,
                "blue" => blue = num,
                "green" => green = num,
                _ => panic!("Unrecognised colour."),
            }
        }

        Self::new(red, green, blue)
    }
}

struct GameResult {
    id: u64,
    subsets: Vec<GameSubset>,
}

impl GameSubset {
    fn is_valid(&self, test_case: &GameSubset) -> bool {
        !(self.red > test_case.red || self.green > test_case.green || self.blue > test_case.blue)
    }
}

impl GameResult {
    fn parse(input: &str) -> Self {
        let game: Vec<&str> = input.split(':').collect();
        let id: u64 = game
            .first()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let subsets: Vec<GameSubset> = game
            .last()
            .unwrap()
            .split(';')
            .map(GameSubset::parse)
            .collect();

        Self { id, subsets }
    }

    fn is_valid(&self, test_case: &GameSubset) -> bool {
        self.subsets
            .iter()
            .map(|s| s.is_valid(test_case))
            .all(|x| x)
    }
}

fn cube_conundrum<P: AsRef<Path>>(input: P) -> u64 {
    let reader = input_buffer_reader(input);
    let test_case = GameSubset::new(12, 13, 14);

    let mut total = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let game_result = GameResult::parse(&line);
        if game_result.is_valid(&test_case) {
            total += game_result.id;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let result = cube_conundrum("test_part1");
        assert_eq!(result, 8);
    }

    #[test]
    fn parse_game_subset() {
        let result = GameSubset::parse("3 green");

        assert_eq!(result.blue, 0);
        assert_eq!(result.red, 0);
        assert_eq!(result.green, 3);
    }

    #[test]
    fn parse_game_result() {
        let result = GameResult::parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");

        assert_eq!(result.id, 1);
        assert_eq!(result.subsets.len(), 3);
    }
}
