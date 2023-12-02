use core::panic;
use std::str::FromStr;

use aoc_utils::{puzzle_input_lines, Cli, PuzzleLines};
use nom::{
    bytes::complete::{tag, take_till},
    character::complete::{alpha1, digit1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

fn main() {
    let cli = Cli::parse_args();
    let lines = puzzle_input_lines("input");
    if cli.part_two {
        let result = cube_conundrum_part2(lines);
        println!("Cube game power is: {result}")
    } else {
        let result = cube_conundrum(lines);
        println!("Cube game value is: {result}")
    }
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

    fn parse(input: &str) -> IResult<&str, Self> {
        let (next_input, subset) = take_till(|c| c == ';')(input)?;
        let fragments: Vec<&str> = subset.split(',').collect();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for fragment in fragments {
            let (_, (num, colour)) =
                separated_pair(map_res(digit1, u64::from_str), tag(" "), alpha1)(fragment.trim())?;
            match colour {
                "red" => red = num,
                "blue" => blue = num,
                "green" => green = num,
                _ => panic!("Unrecognised colour."),
            }
        }

        Ok((next_input, Self::new(red, green, blue)))
    }

    fn is_valid(&self, test_case: &GameSubset) -> bool {
        !(self.red > test_case.red || self.green > test_case.green || self.blue > test_case.blue)
    }

    fn power(&self) -> u64 {
        self.red * self.green * self.blue
    }
}

struct GameResult {
    id: u64,
    subsets: Vec<GameSubset>,
}

impl GameResult {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (next_input, game_id) =
            delimited(tag("Game "), map_res(digit1, u64::from_str), tag(": "))(input)?;
        let (next_input, subsets) = separated_list1(tag("; "), GameSubset::parse)(next_input)?;

        Ok((
            next_input,
            Self {
                id: game_id,
                subsets,
            },
        ))
    }

    fn is_valid(&self, test_case: &GameSubset) -> bool {
        self.subsets
            .iter()
            .map(|s| s.is_valid(test_case))
            .all(|x| x)
    }

    fn minimum(&self) -> GameSubset {
        let red = self.subsets.iter().map(|x| x.red).max().unwrap();
        let green = self.subsets.iter().map(|x| x.green).max().unwrap();
        let blue = self.subsets.iter().map(|x| x.blue).max().unwrap();

        GameSubset { red, green, blue }
    }
}

fn cube_conundrum(lines: PuzzleLines) -> u64 {
    let test_case = GameSubset::new(12, 13, 14);

    let mut total = 0;

    for line in lines {
        let line = line.unwrap();
        let (_, game_result) = GameResult::parse(&line).unwrap();
        if game_result.is_valid(&test_case) {
            total += game_result.id;
        }
    }

    total
}

fn cube_conundrum_part2(lines: PuzzleLines) -> u64 {
    let mut total = 0;

    for line in lines {
        let line = line.unwrap();
        let (_, game_result) = GameResult::parse(&line).unwrap();
        let min_power = game_result.minimum().power();
        total += min_power;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let lines = puzzle_input_lines("test_part1");
        let result = cube_conundrum(lines);
        assert_eq!(result, 8);
    }

    #[test]
    fn part_two() {
        let lines = puzzle_input_lines("test_part1");
        let result = cube_conundrum_part2(lines);
        assert_eq!(result, 2286);
    }

    #[test]
    fn parse_game_subset() {
        let (_, result) = GameSubset::parse("3 green, 1 blue, 2 red").unwrap();

        assert_eq!(result.blue, 1);
        assert_eq!(result.red, 2);
        assert_eq!(result.green, 3);
    }

    #[test]
    fn parse_game_result() {
        let (_, result) =
            GameResult::parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap();

        assert_eq!(result.id, 1);
        assert_eq!(result.subsets.len(), 3);
    }
}
