//! CLI interface to run Parts 1 & 2 of Day06 of Advent of Code 2024.

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use day06::{CUSTOM_INPUT, EXAMPLE_INPUT, FINAL_INPUT, Result, activate_global_default_tracing_subscriber,
            process_part1, process_part2};
use tracing::{self as tea, Level, instrument};

/// Choose to run Part 1 or 2 of Day06 of Advent of Code 2024.
#[derive(Parser, Debug)]
#[command(version, about, long_about, disable_help_subcommand = true, subcommand_help_heading = "input source")]
pub struct Args {
        /// Which Part to Run
        part:  Part,
        /// Input to use.
        #[command(subcommand)]
        input: Option<Input>,
}
#[derive(Debug, Clone, ValueEnum)]
pub enum Part {
        /// Part 1, of day Day06
        #[value(alias = "1", alias = "i", alias = "I", alias = "one")]
        Part1,
        /// Part 2, of day Day06
        #[value(alias = "2", alias = "ii", alias = "II", alias = "two")]
        Part2,
}
#[derive(Debug, Clone, Subcommand)]
pub enum Input {
        /// Use the example input.
        Example,
        /// Use the full problem input.
        Full,
        /// Use a custom input.
        Custom,
        /// Use custom input; please give a path.
        Other { path: PathBuf },
}

fn main() -> Result<()> {
        let _writer_guard = activate_global_default_tracing_subscriber()?;
        let _enter = tea::debug_span!("main()").entered();
        tea::trace!("tracing subscriber set");
        let cli_user_args = Args::try_parse()?;
        tea::trace!(?cli_user_args);
        let part = cli_user_args.part;
        let inp = cli_user_args.input.unwrap_or_else(|| {
                tea::warn!("-- No input given.  Using Example input. -- ");
                Input::Example
        });
        tea::trace!(?part, ?inp);

        match (part, inp) {
                (Part::Part1, inp) => main_part1(inp),
                (Part::Part2, inp) => main_part2(inp),
        }?;
        tea::trace!("finishing main()");
        Ok(())
}

/// Run Part1_Lib code on binary-bound input1.txt
#[instrument(skip_all, ret(level = Level::DEBUG))]
pub fn main_part1(input: Input) -> Result<u64> {
        let input = match input {
                Input::Example => EXAMPLE_INPUT,
                Input::Full => FINAL_INPUT,
                Input::Custom => CUSTOM_INPUT,
                Input::Other { path } => &std::fs::read_to_string(path)?,
        };
        let val = process_part1(input)?;
        tea::info!(?val, "Part 1 Process result.");
        Ok(val)
}

/// Run Part2_Lib code on binary-bound input2.txt
#[instrument(skip_all, ret(level = Level::DEBUG))]
pub fn main_part2(input: Input) -> Result<u64> {
        let input = match input {
                Input::Example => EXAMPLE_INPUT,
                Input::Full => FINAL_INPUT,
                Input::Custom => CUSTOM_INPUT,
                Input::Other { path } => &std::fs::read_to_string(path)?,
        };
        let val = process_part2(input)?;
        tea::info!(?val, "Part 2 Process result.");
        Ok(val)
}
