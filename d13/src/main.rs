use std::{fs::read_to_string, path::Path};

use aoc_utils::Cli;

fn main() {
    let part_two = Cli::parse_args().part_two;

    let result = if part_two {
        todo!()
    } else {
        mirror_number("input")
    };
    println!("Puzzle result: {result}");
}

fn mirror_number(input: impl AsRef<Path>) -> usize {
    let input = read_to_string(input).unwrap();
    let input = input.strip_suffix('\n').unwrap();

    parse_puzzle(input).iter().map(find_reflections).sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Terrain {
    Ash,
    Rocks,
}

impl Terrain {
    fn from_char(c: &char) -> Self {
        match c {
            '#' => Self::Rocks,
            '.' => Self::Ash,
            _ => panic!("Unrecognised pattern."),
        }
    }
}

type TerrainPattern = Vec<Vec<Terrain>>;

fn parse_puzzle(input: &str) -> Vec<TerrainPattern> {
    let patterns: Vec<&str> = input.split("\n\n").collect();

    patterns
        .into_iter()
        .map(|p| {
            p.split('\n')
                .map(|l| l.chars().map(|c| Terrain::from_char(&c)).collect())
                .collect()
        })
        .collect()
}

fn find_reflections(pattern: &TerrainPattern) -> usize {
    let row_reflection = find_reflection(pattern, 100);
    let transpose_pattern = transpose(pattern);
    let column_reflection = find_reflection(&transpose_pattern, 1);

    match (row_reflection, column_reflection) {
        (Some(r), None) => r,
        (None, Some(c)) => c,
        _ => panic!("Expected only 1 reflection"),
    }
}

fn find_reflection(pattern: &TerrainPattern, multiple: usize) -> Option<usize> {
    for i in 1..pattern.len() {
        let (upper, lower) = pattern.split_at(i);
        let symmetrical = upper
            .iter()
            .rev()
            .zip(lower)
            .map(|(u, l)| u == l)
            .all(|x| x);
        if symmetrical {
            return Some(i * multiple);
        }
    }

    None
}

fn transpose(pattern: &TerrainPattern) -> TerrainPattern {
    let mut transpose_pattern = vec![vec![]; pattern[0].len()];
    for i in 0..pattern.len() {
        for j in 0..pattern[0].len() {
            transpose_pattern[j].push(pattern[i][j]);
        }
    }

    transpose_pattern
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let result = mirror_number("test_part1");
        assert_eq!(result, 405);
    }
}
