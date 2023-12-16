use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
    path::Path,
};

use aoc_utils::Cli;

fn main() {
    let part_two = Cli::parse_args().part_two;

    let result = if part_two {
        todo!()
    } else {
        energized_tiles("input")
    };
    println!("Puzzle result: {result}");
}

fn energized_tiles(input: impl AsRef<Path>) -> usize {
    let contraption = parse_puzzle(&read_to_string(input).unwrap());
    let size = contraption.len() as isize;

    let mut queue = VecDeque::new();
    let mut energized = HashSet::new();

    let root_beam = Beam {
        location: (0, 0),
        direction: Direction::Right,
    };
    queue.push_back(root_beam);

    while !queue.is_empty() {
        let current_beam = queue.pop_front().unwrap();
        if out_of_bounds(&current_beam.location, size) {
            continue;
        }
        if energized.contains(&current_beam) {
            continue;
        }
        energized.insert(current_beam);
        let current_tile =
            contraption[current_beam.location.0 as usize][current_beam.location.1 as usize];
        let next_beams = current_tile.next_beams(&current_beam);
        queue.extend(next_beams);
    }

    energized
        .iter()
        .map(|b| b.location)
        .collect::<HashSet<_>>()
        .len()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    RightMirror,
    LeftMirror,
    HorizontalSplitter,
    VerticalSplitter,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '/' => Self::RightMirror,
            '\\' => Self::LeftMirror,
            '-' => Self::HorizontalSplitter,
            '|' => Self::VerticalSplitter,
            _ => panic!("Unrecognised pattern."),
        }
    }

    fn next_beams(&self, beam: &Beam) -> Vec<Beam> {
        let location = &beam.location;
        let beam_dir = beam.direction;
        match self {
            Self::Empty => vec![beam_dir.next_beam(location)],
            Self::RightMirror => vec![beam_dir.reflect_right().next_beam(location)],
            Self::LeftMirror => vec![beam_dir.reflect_left().next_beam(location)],
            Self::HorizontalSplitter => beam_dir
                .split_horizontal()
                .iter()
                .map(|d| d.next_beam(location))
                .collect(),
            Self::VerticalSplitter => beam_dir
                .split_vertical()
                .iter()
                .map(|d| d.next_beam(location))
                .collect(),
        }
    }
}

type Coord = (isize, isize);
type Contraption = Vec<Vec<Tile>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    location: Coord,
    direction: Direction,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn next_beam(&self, location: &Coord) -> Beam {
        let new_location = match self {
            Self::Up => (location.0 - 1, location.1),
            Self::Right => (location.0, location.1 + 1),
            Self::Down => (location.0 + 1, location.1),
            Self::Left => (location.0, location.1 - 1),
        };

        Beam {
            location: new_location,
            direction: *self,
        }
    }

    fn reflect_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Up,
            Self::Down => Self::Left,
            Self::Left => Self::Down,
        }
    }

    fn reflect_left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Down,
            Self::Down => Self::Right,
            Self::Left => Self::Up,
        }
    }

    fn split_horizontal(&self) -> Vec<Direction> {
        match self {
            Self::Up | Self::Down => vec![Self::Left, Self::Right],
            Self::Right | Self::Left => vec![*self],
        }
    }

    fn split_vertical(&self) -> Vec<Direction> {
        match self {
            Self::Left | Self::Right => vec![Self::Up, Self::Down],
            Self::Up | Self::Down => vec![*self],
        }
    }
}

fn parse_puzzle(input: &str) -> Contraption {
    let input = input.strip_suffix('\n').unwrap();

    input
        .split('\n')
        .map(|r| r.chars().map(Tile::from_char).collect())
        .collect()
}

fn out_of_bounds(coord: &Coord, size: isize) -> bool {
    coord.0 < 0 || coord.1 < 0 || coord.0 >= size || coord.1 >= size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let result = energized_tiles("test_part1");
        assert_eq!(result, 46);
    }
}
