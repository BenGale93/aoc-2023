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

pub fn puzzle_input_lines<P: AsRef<Path>>(input: P) -> PuzzleLines {
    let file = File::open(input).expect("Could not find file {input}");
    let reader = BufReader::new(file);
    reader.lines()
}

pub fn get_entire_puzzle(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
