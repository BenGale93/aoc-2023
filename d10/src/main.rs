use std::path::Path;

use aoc_utils::{get_entire_puzzle, Cli};

fn main() {
    let part_two = Cli::parse_args().part_two;

    let result = if part_two {
        todo!()
    } else {
        loop_steps("input")
    };
    println!("Puzzle result: {result}");
}

#[derive(Debug, Clone, Copy)]
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
    fn from_char(input: &char) -> Self {
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

#[derive(Debug, Clone, Copy)]
struct Cell {
    pipe: Pipe,
    coords: Coords,
}

impl Cell {
    fn next_coords(&self) -> Option<(Coords, Coords)> {
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
            Pipe::Ground => return None,
            Pipe::Start => return None,
        };
        Some(d)
    }

    fn next_pipe(&self, current_coord: Coords) -> Coords {
        let options = match self.next_coords() {
            Some(o) => o,
            None => {
                println!("{current_coord:?}");
                panic!("Expected a direction");
            }
        };
        if options.0 == current_coord {
            options.1
        } else {
            options.0
        }
    }
}

type Map = Vec<Vec<Cell>>;

fn loop_steps(input: impl AsRef<Path>) -> usize {
    let puzzle = get_entire_puzzle(input);

    let map: Map = puzzle
        .into_iter()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| Cell {
                    pipe: Pipe::from_char(&c),
                    coords: (i as isize, j as isize),
                })
                .collect()
        })
        .collect();

    let start = find_start(&map);

    let (mut dir_a, mut dir_b) = starting_directions(&map, &start);
    let (mut incoming_a, mut incoming_b) = (start, start);
    let mut steps = 1;
    while dir_a != dir_b {
        let cell_a = map[dir_a.0 as usize][dir_a.1 as usize];
        dir_a = cell_a.next_pipe(incoming_a);
        incoming_a = cell_a.coords;
        let cell_b = map[dir_b.0 as usize][dir_b.1 as usize];
        dir_b = cell_b.next_pipe(incoming_b);
        incoming_b = cell_b.coords;
        steps += 1;
    }

    steps
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

fn starting_directions(map: &Map, starting_coords: &Coords) -> (Coords, Coords) {
    let lookup = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut matches = vec![];
    for l in lookup {
        let new_coords = (starting_coords.0 + l.0, starting_coords.1 + l.1);
        let row = match map.get(new_coords.0 as usize) {
            Some(c) => c,
            None => continue,
        };
        let cell = match row.get(new_coords.1 as usize) {
            Some(c) => c,
            None => continue,
        };
        let next_cells = cell.next_coords();
        if next_cells.is_some()
            && (next_cells.unwrap().0 == *starting_coords
                || next_cells.unwrap().1 == *starting_coords)
        {
            matches.push(cell.coords);
        }
    }

    (*matches.first().unwrap(), *matches.last().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_one() {
        let result = loop_steps("test_part1");
        assert_eq!(result, 8);
    }
}
