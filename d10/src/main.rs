use std::path::Path;

use aoc_utils::{get_entire_puzzle, Cli};

fn main() {
    let part_two = Cli::parse_args().part_two;

    let result = if part_two {
        loop_area("input")
    } else {
        loop_steps("input")
    };
    println!("Puzzle result: {result}");
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pipe {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl Pipe {
    fn from_char(input: char) -> Self {
        match input {
            '|' => Self::NorthSouth,
            '-' => Self::EastWest,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!("Unexpected character"),
        }
    }
}

type Coords = (isize, isize);

#[derive(Debug, Clone, Copy, PartialEq)]
struct Cell {
    pipe: Pipe,
    coords: Coords,
}

impl Cell {
    const fn next_coords(&self) -> Option<(Coords, Coords)> {
        let d = match self.pipe {
            Pipe::NorthSouth => (
                (self.coords.0 - 1, self.coords.1),
                (self.coords.0 + 1, self.coords.1),
            ),
            Pipe::EastWest => (
                (self.coords.0, self.coords.1 + 1),
                (self.coords.0, self.coords.1 - 1),
            ),
            Pipe::NorthEast => (
                (self.coords.0 - 1, self.coords.1),
                (self.coords.0, self.coords.1 + 1),
            ),
            Pipe::NorthWest => (
                (self.coords.0 - 1, self.coords.1),
                (self.coords.0, self.coords.1 - 1),
            ),
            Pipe::SouthWest => (
                (self.coords.0 + 1, self.coords.1),
                (self.coords.0, self.coords.1 - 1),
            ),
            Pipe::SouthEast => (
                (self.coords.0, self.coords.1 + 1),
                (self.coords.0 + 1, self.coords.1),
            ),
            Pipe::Ground | Pipe::Start => return None,
        };
        Some(d)
    }

    fn next_pipe(&self, current_coord: &Coords) -> Coords {
        let options = self
            .next_coords()
            .map_or_else(|| panic!("Expected a direction {current_coord:?}"), |o| o);
        if options.0 == *current_coord {
            options.1
        } else {
            options.0
        }
    }
}

type Map = Vec<Vec<Cell>>;

fn loop_steps(input: impl AsRef<Path>) -> usize {
    let puzzle = get_entire_puzzle(input);

    let map = create_map(&puzzle);

    let start = find_start(&map);

    let loop_spec = loop_cells(&map, &start);

    loop_spec.len() / 2
}

fn loop_area(input: impl AsRef<Path>) -> usize {
    let puzzle = get_entire_puzzle(input);

    let mut map = create_map(&puzzle);

    let start = find_start(&map);

    let loop_spec = loop_cells(&map, &start);

    map[start.0 as usize][start.1 as usize] = *loop_spec.first().unwrap();

    let mut inside_loop = false;
    let mut area = 0;

    for row in &map {
        for cell in row {
            let is_loop_cell = loop_spec.contains(cell);
            if is_loop_cell
                && (matches!(cell.pipe, Pipe::NorthEast)
                    || matches!(cell.pipe, Pipe::NorthSouth)
                    || matches!(cell.pipe, Pipe::NorthWest))
            {
                inside_loop = !inside_loop;
            }
            if inside_loop && !is_loop_cell {
                area += 1;
            }
        }
    }
    area
}

fn create_map(puzzle: &[String]) -> Map {
    puzzle
        .iter()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| Cell {
                    pipe: Pipe::from_char(c),
                    coords: (i as isize, j as isize),
                })
                .collect()
        })
        .collect()
}

fn find_start(map: &Map) -> Coords {
    for row in map {
        for cell in row {
            if matches!(cell.pipe, Pipe::Start) {
                return cell.coords;
            }
        }
    }
    panic!("Could not find start");
}

fn starting_directions(map: &Map, starting_coords: &Coords) -> (Coords, Coords, Pipe) {
    let lookup = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut matches = vec![];
    let mut matching_lookups = vec![];
    for l in lookup {
        let new_coords = (starting_coords.0 + l.0, starting_coords.1 + l.1);
        let Some(row) = map.get(new_coords.0 as usize) else {
            continue;
        };
        let Some(cell) = row.get(new_coords.1 as usize) else {
            continue;
        };
        let next_cells = cell.next_coords();
        if next_cells.is_some()
            && (next_cells.unwrap().0 == *starting_coords
                || next_cells.unwrap().1 == *starting_coords)
        {
            matches.push(cell.coords);
            matching_lookups.push(l);
        }
    }

    let pipe = match (
        matching_lookups.first().unwrap(),
        matching_lookups.last().unwrap(),
    ) {
        ((-1, 0), (0, 1)) => Pipe::NorthEast,
        ((-1, 0), (1, 0)) => Pipe::NorthSouth,
        ((-1, 0), (0, -1)) => Pipe::NorthWest,
        ((0, 1), (1, 0)) => Pipe::SouthEast,
        ((0, 1), (0, -1)) => Pipe::EastWest,
        ((1, 0), (0, -1)) => Pipe::SouthWest,
        _ => panic!("Unexpected pipe config"),
    };

    (*matches.first().unwrap(), *matches.last().unwrap(), pipe)
}

fn loop_cells(map: &Map, start: &Coords) -> Vec<Cell> {
    let (mut dir_a, mut dir_b, pipe) = starting_directions(map, start);
    let start_cell = Cell {
        pipe,
        coords: *start,
    };
    let mut section_a = vec![start_cell];
    let mut section_b = vec![start_cell];
    while dir_a != dir_b {
        let cell_a = map[dir_a.0 as usize][dir_a.1 as usize];
        dir_a = cell_a.next_pipe(&section_a.last().unwrap().coords);
        section_a.push(cell_a);

        let cell_b = map[dir_b.0 as usize][dir_b.1 as usize];
        dir_b = cell_b.next_pipe(&section_b.last().unwrap().coords);
        section_b.push(cell_b);
    }
    section_a.push(map[dir_a.0 as usize][dir_a.1 as usize]);
    section_b.reverse();
    section_b.pop();
    section_a.extend(section_b);
    section_a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let result = loop_steps("test_part1");
        assert_eq!(result, 8);
    }

    #[test]
    fn part_two_one() {
        let result = loop_area("test1_part2");
        assert_eq!(result, 8);
    }

    #[test]
    fn part_two_two() {
        let result = loop_area("test2_part2");
        assert_eq!(result, 10);
    }
}
