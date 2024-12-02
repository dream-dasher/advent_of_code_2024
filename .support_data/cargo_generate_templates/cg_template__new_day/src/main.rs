//! CLI interface to run Parts 1 & 2 of {{ project-name | title_case }} of Advent of Code 2024.

use clap::Parser;
use {{ project-name | snake_case }}::{process_part1, process_part2,
            support::{Error, Result, generate_tracing_subscriber}};
use tracing::{Level, debug, error, info, info_span, level_filters::LevelFilter, span, trace, warn};
use tracing_subscriber::{EnvFilter, prelude::*};

/// Choose to run Part 1 or 2 of {{ project-name | title_case }} of Advent of Code 2024.
#[derive(Parser, Debug)]
#[command(version, about, long_about)]
pub struct Args {
        /// Which Part to Run
        part: Option<u8>,
}

fn main() -> Result<()> {
        tracing::subscriber::set_global_default(generate_tracing_subscriber())?;
        trace!("tracing subscriber set");
        let cli_user_args = Args::parse();
        trace!(?cli_user_args);

        match cli_user_args.part {
                Some(1) => todo!("part1"),
                Some(2) => todo!("part2"),
                Some(_) => panic!("no such part"),
                // Some(_) => Err(Error::InvalidPart(args.part)),
                None => println!("No Part Specified"),
        }

        trace!("finishing main()");
        Ok(())
}

/// Run Part1_Lib code on binary-bound input1.txt
pub fn part1() -> Result<()> {
        let input1 = include_str!("../data/input1.txt");
        let result = process_part1(input1)?;
        info!(?result, "Part 1 Process result.");
        Ok(())
}

/// Run Part2_Lib code on binary-bound input2.txt
pub fn part2() -> Result<()> {
        let input2 = include_str!("../data/input1.txt");
        let result = process_part1(input2)?;
        info!(?result, "Part 2 Process result.");
        Ok(())
}
