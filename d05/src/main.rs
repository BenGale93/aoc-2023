use std::{fs, ops::Range, path::Path};

use aoc_utils::Cli;

fn main() {
    let part_two = Cli::parse_args().part_two;

    if part_two {
        let result = lowest_seed_location_part2("input");
        println!("Lowest location is: {result}");
    } else {
        let result = lowest_seed_location_part1("input");
        println!("Lowest location is: {result}");
    }
}

fn lowest_seed_location_part1(input: impl AsRef<Path>) -> u64 {
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

fn lowest_seed_location_part2(input: impl AsRef<Path>) -> u64 {
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

    let mut seed_ranges: Vec<Range<u64>> = vec![];
    for i in (0..seeds.len() - 1).step_by(2) {
        let start = *seeds.get(i).unwrap();
        let length = *seeds.get(i + 1).unwrap();

        seed_ranges.push(start..start + length);
    }

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
    for map in maps.iter().rev() {
        let mut processed_map = Vec::new();
        for line in map.iter().rev() {
            let source_start = *line.get(1).unwrap();
            let destination_start = *line.first().unwrap();
            let range_length = *line.last().unwrap();

            let source_range = source_start..source_start + range_length;
            let destination_range = destination_start..destination_start + range_length;
            processed_map.push(vec![destination_range, source_range]);
        }
        processed_maps.push(processed_map);
    }

    for location in 0..u64::MAX {
        let mut current_number = location;
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
        for seed_range in &seed_ranges {
            if seed_range.contains(&current_number) {
                return location;
            };
        }
    }
    u64::MAX
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let result = lowest_seed_location_part1("test_part1");
        assert_eq!(result, 35);
    }

    #[test]
    fn part_two() {
        let result = lowest_seed_location_part2("test_part1");
        assert_eq!(result, 46);
    }
}
