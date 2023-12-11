#![feature(iter_map_windows)]
use std::path::Path;

use aoc_utils::{puzzle_input_lines, Cli};
use itertools::Itertools;

fn main() {
    let part_two = Cli::parse_args().part_two;

    let result = if part_two {
        galaxy_distance_sum("input", 1000000)
    } else {
        galaxy_distance_sum("input", 2)
    };
    println!("Puzzle result: {result}");
}

type Universe = Vec<Vec<char>>;

fn galaxy_distance_sum(input: impl AsRef<Path>, expansion: usize) -> usize {
    let universe: Universe = puzzle_input_lines(input)
        .map(Result::unwrap)
        .map(|c| c.chars().collect())
        .collect();
    let (empty_rows, empty_columns) = empty_rows_and_columns(&universe);

    let mut galaxies = get_galaxies(&universe);
    expand_galaxies(&mut galaxies, &empty_rows, &empty_columns, expansion);
    galaxies
        .iter()
        .combinations(2)
        .map(|comb| distance(comb.first().unwrap(), comb.last().unwrap()))
        .sum()
}

fn empty_rows_and_columns(universe: &Universe) -> (Vec<usize>, Vec<usize>) {
    let mut empty_rows = vec![];
    for (i, row) in universe.iter().enumerate() {
        if row.iter().map_windows(|&[a, b]| a == b).all(|x| x) {
            empty_rows.push(i);
        }
    }

    let mut empty_columns = vec![];
    let universe_width = universe.first().unwrap().len();

    for j in 0..universe_width {
        if universe
            .iter()
            .map(|r| r.get(j).unwrap())
            .map_windows(|&[a, b]| a == b)
            .all(|x| x)
        {
            empty_columns.push(j);
        }
    }

    (empty_rows, empty_columns)
}

type Coord = (usize, usize);

fn get_galaxies(universe: &Universe) -> Vec<Coord> {
    let mut galaxies = vec![];
    for (i, row) in universe.iter().enumerate() {
        for (j, location) in row.iter().enumerate() {
            if location == &'#' {
                galaxies.push((i, j));
            }
        }
    }
    galaxies
}

fn expand_galaxies(
    galaxies: &mut [Coord],
    empty_rows: &[usize],
    empty_columns: &[usize],
    expansion: usize,
) {
    let expansion = expansion - 1;
    for galaxy in galaxies.iter_mut() {
        let new_rows = empty_rows.iter().filter(|x| **x < galaxy.0).count();
        let new_columns = empty_columns.iter().filter(|x| **x < galaxy.1).count();

        *galaxy = (
            galaxy.0 + (new_rows * expansion),
            galaxy.1 + (new_columns * expansion),
        );
    }
}

fn distance(galaxy_a: &Coord, galaxy_b: &Coord) -> usize {
    let row_distance = galaxy_a.0.abs_diff(galaxy_b.0);
    let column_distance = galaxy_a.1.abs_diff(galaxy_b.1);

    row_distance + column_distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let result = galaxy_distance_sum("test_part1", 2);
        assert_eq!(result, 374);
    }

    #[test]
    fn part_two_one() {
        let result = galaxy_distance_sum("test_part1", 10);
        assert_eq!(result, 1030);
    }

    #[test]
    fn part_two_two() {
        let result = galaxy_distance_sum("test_part1", 100);
        assert_eq!(result, 8410);
    }
}
