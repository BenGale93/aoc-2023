use std::{
    collections::{HashSet, VecDeque},
    fmt::Debug,
    path::Path,
};

use aoc_utils::{out_of_bounds, Cli, Coord, Direction, FromChar};

fn main() {
    let part_two = Cli::parse_args().part_two;

    let result = if true {
        infinite_garden_plots("input", 26501365)
    } else {
        garden_plots("input", 64)
    };
    println!("Puzzle result: {result}");
}

fn garden_plots(input: impl AsRef<Path>, step_limit: usize) -> isize {
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

    reached.len() as isize
}

fn infinite_garden_plots(input: impl AsRef<Path>, step_limit: isize) -> isize {
    let input = std::fs::read_to_string(input).unwrap();
    let (start, garden) = parse_puzzle(&input);

    let size = garden.len();
    let modulo = modulo(step_limit, size);

    let search_steps = (modulo, modulo + size, modulo + 2 * size);

    let (first, second, third) = infinite_garden_search(&garden, &start, search_steps, size);

    let first_diff1 = second - first;
    let first_diff2 = third - second;
    let second_diff = first_diff2 - first_diff1;

    let a = second_diff / 2;
    let b = first_diff1 - 3 * a;
    let c = first - b - a;

    let n = (step_limit as f64 / size as f64).ceil() as isize;
    a * n.pow(2) + b * n + c
}

fn modulo(a: isize, b: usize) -> usize {
    if a >= 0 {
        (a as usize) % b
    } else {
        let r = (!a as usize) % b;
        b - r - 1
    }
}

fn infinite_garden_search(
    garden: &Garden,
    start: &Coord,
    search_steps: (usize, usize, usize),
    size: usize,
) -> (isize, isize, isize) {
    let directions = &[
        Direction::Up,
        Direction::Left,
        Direction::Down,
        Direction::Right,
    ];

    let mut queue = VecDeque::new();
    let mut first_reached: HashSet<Coord> = HashSet::new();
    let mut second_reached: HashSet<Coord> = HashSet::new();
    let mut third_reached: HashSet<Coord> = HashSet::new();
    queue.push_back((*start, 0));

    while let Some((position, steps)) = queue.pop_front() {
        if steps == search_steps.0 {
            first_reached.insert(position);
        } else if steps == search_steps.1 {
            second_reached.insert(position);
        } else if steps == search_steps.2 {
            third_reached.insert(position);
            continue;
        }
        for direction in directions {
            let next_position = direction.next_coord(&position);
            let next_row = modulo(next_position.0, size);
            let next_column = modulo(next_position.1, size);
            let next_terrain = garden[next_row][next_column];
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

    (
        first_reached.len() as isize,
        second_reached.len() as isize,
        third_reached.len() as isize,
    )
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

    #[test]
    fn part_two() {
        let result = infinite_garden_plots("test_part1", 10);
        assert_eq!(result, 50);
    }
}
