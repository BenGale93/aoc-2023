use std::{path::Path, str::FromStr};

use aoc_utils::{puzzle_input_lines, Cli, Coord, Direction};

fn main() {
    let part_two = Cli::parse_args().part_two;

    let result = if part_two {
        lava_volume_hex("input")
    } else {
        lava_volume("input")
    };
    println!("Puzzle result: {result}");
}

fn lava_volume(input: impl AsRef<Path>) -> isize {
    let dig_plan = parse_puzzle(input);
    let mut coordinates: Vec<Coord> = vec![];
    let mut current_position: Coord = (0, 0);
    let mut boundary_length = 0;
    for record in &dig_plan {
        current_position = record
            .direction
            .next_coord_far(&current_position, record.distance);
        boundary_length += record.distance;
        coordinates.push(current_position);
    }
    let area = shoelace_formula(&coordinates);

    // Pick's theorem
    let internal_points = area + 1 - (boundary_length / 2);

    boundary_length + internal_points
}

fn lava_volume_hex(input: impl AsRef<Path>) -> isize {
    let dig_plan = parse_puzzle(input);
    let mut coordinates: Vec<Coord> = vec![];
    let mut current_position: Coord = (0, 0);
    let mut boundary_length = 0;
    for record in &dig_plan {
        let (direction, distance) = record.hex_instruction();
        current_position = direction.next_coord_far(&current_position, distance);
        boundary_length += distance;
        coordinates.push(current_position);
    }
    let area = shoelace_formula(&coordinates);

    // Pick's theorem
    let internal_points = area + 1 - (boundary_length / 2);

    boundary_length + internal_points
}

fn shoelace_formula(coordinates: &[Coord]) -> isize {
    let result: isize = coordinates
        .windows(2)
        .map(|win| (win[0], win[1]))
        .map(|(a, b)| (a.0 * b.1) - (a.1 * b.0))
        .sum();

    (result / 2).abs()
}

#[derive(Debug)]
struct Record {
    direction: Direction,
    distance: isize,
    colour: String,
}

impl FromStr for Record {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();

        let direction = match *parts.first().unwrap() {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "L" => Direction::Left,
            "D" => Direction::Down,
            _ => panic!("Unrecognised pattern"),
        };

        Ok(Record {
            direction,
            distance: parts.get(1).unwrap().parse().unwrap(),
            colour: parts.last().unwrap().to_string(),
        })
    }
}

impl Record {
    fn hex_instruction(&self) -> (Direction, isize) {
        let mut colour = self.colour.trim_matches(&['(', ')']).to_string();
        let last_char = colour.pop().unwrap();

        let colour = colour.strip_prefix('#').unwrap();
        let distance = isize::from_str_radix(colour, 16).unwrap();

        let direction = match last_char {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _ => panic!("Unrecognised pattern"),
        };

        (direction, distance)
    }
}

fn parse_puzzle(input: impl AsRef<Path>) -> Vec<Record> {
    puzzle_input_lines(input)
        .map(Result::unwrap)
        .map(|s| Record::from_str(&s))
        .map(Result::unwrap)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let result = lava_volume("test_part1");
        assert_eq!(result, 62);
    }

    #[test]
    fn part_two() {
        let result = lava_volume_hex("test_part1");
        assert_eq!(result, 952408144115);
    }
}
