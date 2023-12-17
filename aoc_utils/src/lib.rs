use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::Path,
};

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub part_two: bool,
}

impl Cli {
    #[must_use]
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

pub type PuzzleLines = Lines<BufReader<File>>;

#[allow(clippy::missing_panics_doc)]
pub fn puzzle_input_lines<P: AsRef<Path>>(input: P) -> PuzzleLines {
    let file = File::open(input).expect("Could not find file {input}");
    let reader = BufReader::new(file);
    reader.lines()
}

#[allow(clippy::missing_panics_doc)]
pub fn get_entire_puzzle(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub trait FromChar {
    fn from_char(c: char) -> Self;
}

/// Parses the puzzle input as a matrix of type T.
///
/// Type T is usually an enum and implements `FromChar` which
/// takes each character in the input and maps it to an enum variant.
///
/// # Panics
/// Assumes the puzzle has a blank line at the bottom.
#[must_use]
pub fn puzzle_matrix<T: FromChar>(input: &str) -> Vec<Vec<T>> {
    let input = input.strip_suffix('\n').unwrap();

    input
        .split('\n')
        .map(|p| p.chars().map(T::from_char).collect())
        .collect()
}

pub type Coord = (isize, isize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub const fn next_coord(self, location: &Coord) -> Coord {
        match self {
            Self::Up => (location.0 - 1, location.1),
            Self::Right => (location.0, location.1 + 1),
            Self::Down => (location.0 + 1, location.1),
            Self::Left => (location.0, location.1 - 1),
        }
    }
}
