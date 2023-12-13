use std::{path::Path, str::FromStr};

use aoc_utils::{puzzle_input_lines, Cli};
use memoize::memoize;

fn main() {
    let part_two = Cli::parse_args().part_two;

    let result = if part_two {
        spring_puzzle_sum_part2("input")
    } else {
        spring_puzzle_sum("input")
    };
    println!("Puzzle result: {result}");
}

fn spring_puzzle_sum(input: impl AsRef<Path>) -> usize {
    puzzle_input_lines(input)
        .map(Result::unwrap)
        .map(|s| SpringRecord::from_str(&s))
        .map(Result::unwrap)
        .map(|s| count_combinations(s.row, s.groups))
        .sum()
}

fn spring_puzzle_sum_part2(input: impl AsRef<Path>) -> usize {
    puzzle_input_lines(input)
        .map(Result::unwrap)
        .map(|s| SpringRecord::from_str_part_2(&s))
        .map(|s| count_combinations(s.row, s.groups))
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Spring {
    Ok,
    Bad,
    Unknown,
}

impl Spring {
    fn from_char(input: char) -> Self {
        match input {
            '#' => Self::Bad,
            '.' => Self::Ok,
            '?' => Self::Unknown,
            _ => panic!("Unexpected character"),
        }
    }
}

type Springs = Vec<Spring>;
type Groups = Vec<usize>;

#[derive(Debug)]
struct SpringRecord {
    row: Springs,
    groups: Groups,
}

impl FromStr for SpringRecord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line_split: Vec<&str> = s.split(' ').collect();
        let row: Vec<Spring> = line_split
            .first()
            .unwrap()
            .chars()
            .map(Spring::from_char)
            .collect();
        let groups: Vec<usize> = line_split
            .last()
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();

        Ok(Self { row, groups })
    }
}

impl SpringRecord {
    fn from_str_part_2(s: &str) -> Self {
        let line_split: Vec<&str> = s.split(' ').collect();

        let springs = *line_split.first().unwrap();
        let springs = [springs; 5].join("?");

        let groupings = *line_split.last().unwrap();
        let groupings = [groupings; 5].join(",");

        let row: Vec<Spring> = springs.chars().map(Spring::from_char).collect();
        let groups: Vec<usize> = groupings.split(',').map(|n| n.parse().unwrap()).collect();

        Self { row, groups }
    }
}

#[memoize]
fn count_combinations(springs: Vec<Spring>, groups: Vec<usize>) -> usize {
    let no_more_springs = springs.is_empty();
    let no_more_groups = groups.is_empty();
    if no_more_springs && no_more_groups {
        return 1;
    } else if no_more_springs && !no_more_groups {
        return 0;
    }

    let spring_length = springs.len();
    let front_spring = springs[0];
    if matches!(front_spring, Spring::Ok) {
        count_combinations(springs[1..spring_length].to_vec(), groups)
    } else if matches!(front_spring, Spring::Unknown) {
        let mut broken = springs.clone();
        broken[0] = Spring::Bad;
        let mut fixed = springs;
        fixed[0] = Spring::Ok;
        let total_broken = count_combinations(broken, groups.clone());
        let total_fixed = count_combinations(fixed, groups);
        return total_broken + total_fixed;
    } else if no_more_groups
        || spring_length < groups[0]
        || springs[0..groups[0]]
            .iter()
            .filter(|s| matches!(s, Spring::Ok))
            .count()
            > 0
        || (groups.len() > 1 && groups[0] + 1 >= spring_length)
    {
        return 0;
    } else {
        let next_springs = springs[groups[0]..spring_length].to_vec();
        let next_groups = groups[1..groups.len()].to_vec();

        let total = if next_springs.is_empty() || next_groups.is_empty() {
            count_combinations(next_springs, next_groups)
        } else {
            match next_springs.first() {
                Some(Spring::Bad) => 0,
                Some(Spring::Ok) => count_combinations(next_springs, next_groups),
                Some(Spring::Unknown) => {
                    let mut fixed = next_springs.clone();
                    fixed[0] = Spring::Ok;
                    count_combinations(fixed, next_groups)
                }
                None => panic!("Expected next_springs not to be empty"),
            }
        };

        return total;
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn part_one() {
        let result = spring_puzzle_sum("test_part1");
        assert_eq!(result, 21);
    }

    #[test]
    fn part_two() {
        let result = spring_puzzle_sum_part2("test_part1");
        assert_eq!(result, 525152);
    }

    #[rstest]
    #[case("## 2", 1)]
    #[case("?? 1", 2)]
    #[case("?? 1,1", 0)]
    #[case("??. 1,1", 0)]
    #[case(".?? 1,1", 0)]
    #[case("?.? 1,1", 1)]
    #[case("??? 1,1", 1)]
    #[case("??# 1,1", 1)]
    fn basic_count(#[case] input: &str, #[case] expected: usize) {
        let spring_line = SpringRecord::from_str(input).unwrap();
        let count = count_combinations(spring_line.row, spring_line.groups);

        assert_eq!(count, expected);
    }
}
