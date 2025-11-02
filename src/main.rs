mod common;
mod days;

use clap::Parser;

/// Advent of Code 2019
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The day to run.
    day: u8,
}

fn main() {
    days::run(Cli::parse().day);
}
