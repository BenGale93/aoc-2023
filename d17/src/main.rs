use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fs::read_to_string,
    path::Path,
};

use aoc_utils::Cli;

fn main() {
    let part_two = Cli::parse_args().part_two;

    let result = minimum_heat_loss("input", part_two);
    println!("Puzzle result: {result}");
}

fn minimum_heat_loss(input: impl AsRef<Path>, ultra: bool) -> usize {
    let (graph, goal) = parse_puzzle(input);
    let start: Coord = (0, 0);

    let start_state1 = State {
        cost: 0,
        position: start,
        dir_count: 0,
        direction: Direction::Right,
    };
    let start_state2 = State {
        cost: 0,
        position: start,
        dir_count: 0,
        direction: Direction::Down,
    };

    let mut dist: HashMap<(Coord, usize, Direction), usize> = HashMap::from([
        ((start, 0, Direction::Right), 0),
        ((start, 0, Direction::Down), 0),
    ]);
    let mut heap = BinaryHeap::new();

    heap.push(start_state1);
    heap.push(start_state2);

    while let Some(current_state) = heap.pop() {
        if current_state.position == goal {
            return current_state.cost;
        }

        if current_state.cost
            > *dist
                .get(&current_state.without_cost())
                .unwrap_or(&usize::MAX)
        {
            continue;
        }

        for valid_move in valid_moves(&graph, &current_state, ultra) {
            if ultra && valid_move.position == goal && valid_move.dir_count < 4 {
                continue;
            }
            let next = State {
                cost: current_state.cost + valid_move.cost,
                position: valid_move.position,
                dir_count: valid_move.dir_count,
                direction: valid_move.direction,
            };

            if next.cost < *dist.get(&next.without_cost()).unwrap_or(&usize::MAX) {
                heap.push(next);
                dist.insert(next.without_cost(), next.cost);
            }
        }
    }

    usize::MAX
}

fn valid_moves(graph: &HashMap<Coord, usize>, current_state: &State, ultra: bool) -> Vec<State> {
    let mut valid_moves = vec![];
    for direction in current_state
        .direction
        .valid_directions(current_state.dir_count, ultra)
    {
        let next_coord = direction.next_coord(&current_state.position);
        let Some(next_cost) = graph.get(&next_coord) else {
            continue;
        };
        let dir_count = if current_state.direction == direction {
            current_state.dir_count + 1
        } else {
            1
        };
        valid_moves.push(State {
            cost: *next_cost,
            position: next_coord,
            dir_count,
            direction,
        });
    }
    valid_moves
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn valid_directions(self, dir_count: usize, ultra: bool) -> Vec<Self> {
        let mut potential_directions = match self {
            Self::Up => vec![Self::Left, Self::Right, Self::Up],
            Self::Left => vec![Self::Up, Self::Down, Self::Left],
            Self::Right => vec![Self::Up, Self::Down, Self::Right],
            Self::Down => vec![Self::Left, Self::Right, Self::Down],
        };
        if ultra {
            if dir_count < 4 {
                return vec![self];
            } else if dir_count >= 10 {
                potential_directions.pop();
            }
        } else if dir_count >= 3 {
            potential_directions.pop();
        }
        potential_directions
    }

    const fn next_coord(self, location: &Coord) -> Coord {
        match self {
            Self::Up => (location.0 - 1, location.1),
            Self::Right => (location.0, location.1 + 1),
            Self::Down => (location.0 + 1, location.1),
            Self::Left => (location.0, location.1 - 1),
        }
    }
}

type Coord = (isize, isize);

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Coord,
    dir_count: usize,
    direction: Direction,
}

impl State {
    const fn without_cost(&self) -> (Coord, usize, Direction) {
        (self.position, self.dir_count, self.direction)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_puzzle(input: impl AsRef<Path>) -> (HashMap<Coord, usize>, Coord) {
    let input = read_to_string(input).unwrap();
    let input = input.strip_suffix('\n').unwrap();

    let map: Vec<Vec<usize>> = input
        .split('\n')
        .map(|p| p.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect();

    let mut graph = HashMap::new();

    for (i, row) in map.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            graph.insert((i as isize, j as isize), *value);
        }
    }
    (graph, (map.len() as isize - 1, map[0].len() as isize - 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let result = minimum_heat_loss("test_part1", false);
        assert_eq!(result, 102);
    }

    #[test]
    fn short_input() {
        let result = minimum_heat_loss("short_test", false);
        assert_eq!(result, 7);
    }

    #[test]
    fn part_two() {
        let result = minimum_heat_loss("test_part1", true);
        assert_eq!(result, 94);
    }

    #[test]
    fn part_two_short_input() {
        let result = minimum_heat_loss("test_part2", true);
        assert_eq!(result, 71);
    }
}
