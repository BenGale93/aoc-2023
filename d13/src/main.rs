use std::{fs::read_to_string, path::Path};

use aoc_utils::Cli;

fn main() {
    let part_two = Cli::parse_args().part_two;

    let result = if part_two {
        smudged_mirror_number("input")
    } else {
        mirror_number("input")
    };
    println!("Puzzle result: {result}");
}

fn mirror_number(input: impl AsRef<Path>) -> usize {
    let input = read_to_string(input).unwrap();

    parse_puzzle(&input).iter().map(find_reflections).sum()
}

fn smudged_mirror_number(input: impl AsRef<Path>) -> usize {
    let input = read_to_string(input).unwrap();

    parse_puzzle(&input)
        .iter()
        .map(find_smudged_reflections)
        .sum()
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

    fn other(&self) -> Self {
        match self {
            Self::Ash => Self::Rocks,
            Self::Rocks => Self::Ash,
        }
    }
}

type TerrainPattern = Vec<Vec<Terrain>>;

fn parse_puzzle(input: &str) -> Vec<TerrainPattern> {
    let input = input.strip_suffix('\n').unwrap();
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
    let row_reflection = find_reflection(pattern, 100, None);
    let transpose_pattern = transpose(pattern);
    let column_reflection = find_reflection(&transpose_pattern, 1, None);

    match (row_reflection, column_reflection) {
        (Some(r), None) => r,
        (None, Some(c)) => c,
        _ => panic!("Expected only 1 reflection"),
    }
}

fn find_smudged_reflections(pattern: &TerrainPattern) -> usize {
    let row_reflection = find_reflection(pattern, 100, None);
    let transpose_pattern = transpose(pattern);
    let column_reflection = find_reflection(&transpose_pattern, 1, None);

    let mut smudge_pattern = pattern.clone();
    let new_row_reflection = find_smudged_reflection(&mut smudge_pattern, 100, row_reflection);

    let mut transposed_smudge_pattern = transpose_pattern.clone();
    let new_column_reflection =
        find_smudged_reflection(&mut transposed_smudge_pattern, 1, column_reflection);

    match (new_row_reflection, new_column_reflection) {
        (Some(r), None) => r,
        (None, Some(c)) => c,
        _ => panic!("Expected only 1 reflection"),
    }
}

fn find_smudged_reflection(
    pattern: &mut TerrainPattern,
    multiple: usize,
    current_reflection: Option<usize>,
) -> Option<usize> {
    for i in 0..pattern.len() {
        for j in 0..pattern[0].len() {
            let current = pattern[i][j];
            pattern[i][j] = current.other();
            let new_reflection = find_reflection(pattern, multiple, current_reflection);
            pattern[i][j] = current;
            if new_reflection.is_some() && new_reflection != current_reflection {
                return new_reflection;
            }
        }
    }
    None
}

fn find_reflection(
    pattern: &TerrainPattern,
    multiple: usize,
    current_reflection: Option<usize>,
) -> Option<usize> {
    for i in 1..pattern.len() {
        let (upper, lower) = pattern.split_at(i);
        let symmetrical = upper
            .iter()
            .rev()
            .zip(lower)
            .map(|(u, l)| u == l)
            .all(|x| x);
        if symmetrical {
            let new_reflection = Some(i * multiple);
            if new_reflection != current_reflection {
                return new_reflection;
            }
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

    #[test]
    fn part_two() {
        let result = smudged_mirror_number("test_part1");
        assert_eq!(result, 400);
    }
}
