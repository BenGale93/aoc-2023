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