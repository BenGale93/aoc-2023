use std::{path::Path, io::BufReader, fs::File};

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub part_two: bool
}

impl Cli {
    pub fn parse_args() -> Self {
        Cli::parse()
    }
}

pub fn input_buffer_reader<P: AsRef<Path>>(input: P) -> BufReader<File> {
    let file = File::open(input).expect("Could not find file {input}");
    BufReader::new(file)
}