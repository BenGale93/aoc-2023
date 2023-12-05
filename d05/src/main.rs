use std::{fs, ops::Range, path::Path};

use aoc_utils::Cli;

fn main() {
    let part_two = Cli::parse_args().part_two;

    if part_two {
        todo!()
    } else {
        let result = lowest_seed_location("input");
        println!("Lowest location is: {result}");
    }
}

fn lowest_seed_location(input: impl AsRef<Path>) -> u64 {
    let puzzle = fs::read_to_string(input).unwrap();

    let mut puzzle: Vec<_> = puzzle.split("\n\n").collect();
    let seeds: Vec<u64> = puzzle
        .remove(0)
        .split(':')
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let maps: Vec<_> = puzzle
        .iter()
        .map(|m| m.split(':').last().unwrap())
        .map(|m| {
            m.strip_prefix('\n')
                .unwrap()
                .split('\n')
                .collect::<Vec<_>>()
        })
        .map(|m| {
            m.iter()
                .map(|n| {
                    n.split_ascii_whitespace()
                        .map(|n| n.parse().unwrap())
                        .collect::<Vec<u64>>()
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut processed_maps: Vec<Vec<Vec<Range<u64>>>> = Vec::new();
    for map in maps {
        let mut processed_map = Vec::new();
        for line in map {
            let source_start = *line.get(1).unwrap();
            let destination_start = *line.first().unwrap();
            let range_length = *line.last().unwrap();

            let source_range = source_start..source_start + range_length;
            let destination_range = destination_start..destination_start + range_length;
            processed_map.push(vec![source_range, destination_range]);
        }
        processed_maps.push(processed_map);
    }

    let mut lowest: u64 = u64::MAX;
    for seed in seeds {
        let mut current_number = seed;
        for map in &processed_maps {
            for m in map {
                let source = m.first().unwrap();
                let destination = m.last().unwrap();
                if source.contains(&current_number) {
                    current_number = (current_number - source.start) + destination.start;
                    break;
                }
            }
        }
        lowest = current_number.min(lowest);
    }

    lowest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let result = lowest_seed_location("test_part1");
        assert_eq!(result, 35);
    }
}
