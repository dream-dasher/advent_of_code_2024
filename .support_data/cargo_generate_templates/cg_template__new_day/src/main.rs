//! CLI interface to run Parts 1 & 2 of {{ project-name | title_case }} of Advent of Code 2024.

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use {{ project-name | snake_case }}::{Result, generate_tracing_subscriber, process_part1, process_part2};
use tracing::{info, instrument, trace};

/// Choose to run Part 1 or 2 of {{ project-name | title_case }} of Advent of Code 2024.
#[derive(Parser, Debug)]
#[command(version, about, long_about)]
pub struct Args {
        /// Which Part to Run
        part:  Option<Part>,
        /// Input to use.
        #[command(subcommand)]
        input: Option<Input>,
}
#[derive(Debug, Clone, ValueEnum)]
pub enum Part {
        /// Part 1, of day {{ project-name | title_case }}
        #[value(alias = "1", alias = "i", alias = "I", alias = "one")]
        Part1,
        /// Part 2, of day {{ project-name | title_case }}
        #[value(alias = "2", alias = "ii", alias = "II", alias = "two")]
        Part2,
}
#[derive(Debug, Clone, Subcommand)]
pub enum Input {
        /// Use the example input.
        Example,
        /// Run the full problem's input.
        Full,
        /// Run some custom input, please give a path.
        Other { path: PathBuf },
}



#[instrument]
fn main() -> Result<()> {
        tracing::subscriber::set_global_default(generate_tracing_subscriber())?;
        trace!("tracing subscriber set");
        let cli_user_args = Args::parse();
        trace!(?cli_user_args);

        // match cli_user_args.part {
        //         Some(1) => todo!("part1"),
        //         Some(2) => todo!("part2"),
        //         Some(_) => panic!("no such part"),
        //         // Some(_) => Err(Error::InvalidPart(args.part)),
        //         None => println!("No Part Specified"),
        // }

        trace!("finishing main()");
        Ok(())
}

/// Run Part1_Lib code on binary-bound input1.txt
#[instrument]
pub fn part1() -> Result<()> {
        let input1 = include_str!("../data/input1.txt");
        let result = process_part1(input1)?;
        info!(?result, "Part 1 Process result.");
        Ok(())
}

/// Run Part2_Lib code on binary-bound input2.txt
#[instrument]
pub fn part2() -> Result<()> {
        let input2 = include_str!("../data/input2.txt");
        let result = process_part2(input2)?;
        info!(?result, "Part 2 Process result.");
        Ok(())
}
