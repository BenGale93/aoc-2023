use std::{fs::read_to_string, path::Path};

use aoc_utils::Cli;

fn main() {
    let part_two = Cli::parse_args().part_two;

    let result = if part_two {
        total_beam_load_spin_cycle("input")
    } else {
        total_beam_load("input")
    };
    println!("Puzzle result: {result}");
}

fn total_beam_load(input: impl AsRef<Path>) -> usize {
    let input = read_to_string(input).unwrap();
    let platform = parse_puzzle(&input);
    let rotated_platform = rotate_counter_clockwise(&platform);
    let titled_platform = tilt_platform(&rotated_platform);
    load_sum(&titled_platform)
}

fn total_beam_load_spin_cycle(input: impl AsRef<Path>) -> usize {
    let input = read_to_string(input).unwrap();
    let mut platform = rotate_counter_clockwise(&parse_puzzle(&input));
    let mut load_sums = vec![];
    for i in 0..1000 {
        for j in 0..4 {
            platform = tilt_platform(&platform);
            platform = rotate_clockwise(&platform);
        }
        load_sums.push(load_sum(&platform));
    }

    *load_sums.last().unwrap_or(&0)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Terrain {
    Round,
    Cube,
    Ground,
}

impl Terrain {
    fn from_char(c: char) -> Self {
        match c {
            'O' => Self::Round,
            '#' => Self::Cube,
            '.' => Self::Ground,
            _ => panic!("Unrecognised pattern."),
        }
    }
}

type Row = Vec<Terrain>;
type Platform = Vec<Vec<Terrain>>;

fn parse_puzzle(input: &str) -> Platform {
    let input = input.strip_suffix('\n').unwrap();

    input
        .split('\n')
        .map(|p| p.chars().map(Terrain::from_char).collect())
        .collect()
}

fn rotate_counter_clockwise(platform: &Platform) -> Platform {
    let mut new_platform = vec![vec![]; platform[0].len()];
    let platform_length = platform.len();
    for row in platform {
        for (j, value) in row.iter().enumerate() {
            new_platform[platform_length - j - 1].push(*value);
        }
    }

    new_platform
}

fn rotate_clockwise(platform: &Platform) -> Platform {
    let mut new_platform = vec![vec![]; platform[0].len()];
    for row in platform.iter().rev() {
        for (j, value) in row.iter().enumerate() {
            new_platform[j].push(*value);
        }
    }

    new_platform
}

fn tilt_platform(platform: &Platform) -> Platform {
    platform.iter().map(tilt_row).collect()
}

fn tilt_row(row: &Row) -> Row {
    let mut cube_positions = vec![];
    let mut round_counts = vec![];
    let mut round_count: usize = 0;
    for (i, terrain) in row.iter().enumerate() {
        match *terrain {
            Terrain::Ground => (),
            Terrain::Cube => {
                cube_positions.push(i);
                round_counts.push(round_count);
                round_count = 0
            }
            Terrain::Round => round_count += 1,
        }
    }
    round_counts.push(round_count);

    round_counts.reverse();

    let mut titled_row = vec![];
    let mut current_round_count = round_counts.pop().unwrap_or(0);
    for i in 0..row.len() {
        if cube_positions.contains(&i) {
            titled_row.push(Terrain::Cube);
            current_round_count = round_counts.pop().unwrap_or(0);
        } else if current_round_count == 0 {
            titled_row.push(Terrain::Ground);
        } else {
            current_round_count -= 1;
            titled_row.push(Terrain::Round);
        }
    }
    titled_row
}

fn load_sum(platform: &Platform) -> usize {
    let mut load_sum = 0;
    let length = platform[0].len();
    for row in platform {
        for (i, point) in row.iter().enumerate() {
            let multiplier = length - i;
            if matches!(point, Terrain::Round) {
                load_sum += multiplier;
            }
        }
    }
    load_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let result = total_beam_load("test_part1");
        assert_eq!(result, 136);
    }

    #[test]
    fn part_two() {
        let result = total_beam_load_spin_cycle("test_part1");
        assert_eq!(result, 64);
    }
}
