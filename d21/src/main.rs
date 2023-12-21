use std::{
    collections::{HashSet, VecDeque},
    fmt::Debug,
    path::Path,
};

use aoc_utils::{out_of_bounds, Cli, Coord, Direction, FromChar};

fn main() {
    let part_two = Cli::parse_args().part_two;

    let result = if part_two {
        todo!()
    } else {
        garden_plots("input", 64)
    };
    println!("Puzzle result: {result}");
}

fn garden_plots(input: impl AsRef<Path>, step_limit: usize) -> usize {
    let input = std::fs::read_to_string(input).unwrap();
    let (start, garden) = parse_puzzle(&input);

    let size = garden.len() as isize;
    let directions = &[
        Direction::Up,
        Direction::Left,
        Direction::Down,
        Direction::Right,
    ];

    let mut queue = VecDeque::new();
    let mut reached: HashSet<Coord> = HashSet::new();
    queue.push_back((start, 0));

    while let Some((position, steps)) = queue.pop_front() {
        if steps == step_limit {
            reached.insert(position);
            continue;
        }
        for direction in directions {
            let next_position = direction.next_coord(&position);
            if out_of_bounds(&next_position, size) {
                continue;
            }
            let next_terrain = garden[next_position.0 as usize][next_position.1 as usize];
            match next_terrain {
                Terrain::Garden => {
                    let next_instruction = (next_position, steps + 1);
                    if !queue.contains(&next_instruction) {
                        queue.push_back(next_instruction);
                    }
                }
                Terrain::Rock => continue,
            }
        }
    }

    reached.len()
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Terrain {
    Garden,
    Rock,
}

impl FromChar for Terrain {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Garden,
            '#' => Self::Rock,
            _ => panic!("Unrecognised symbol"),
        }
    }
}

impl Debug for Terrain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Garden => write!(f, "."),
            Self::Rock => write!(f, "#"),
        }
    }
}

type Garden = Vec<Vec<Terrain>>;

fn parse_puzzle(input: &str) -> (Coord, Garden) {
    let input = input.strip_suffix('\n').unwrap();

    let input: Vec<&str> = input.split('\n').collect();
    let mut start_coord: Coord = (0, 0);
    let mut garden = vec![];
    for (i, row) in input.iter().enumerate() {
        let mut new_row = vec![];
        for (j, char) in row.chars().enumerate() {
            if char == 'S' {
                start_coord = (i as isize, j as isize);
                new_row.push(Terrain::Garden);
            } else {
                new_row.push(Terrain::from_char(char));
            }
        }
        garden.push(new_row);
    }
    (start_coord, garden)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let result = garden_plots("test_part1", 6);
        assert_eq!(result, 16);
    }
}
