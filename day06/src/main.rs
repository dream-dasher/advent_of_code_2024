//! CLI interface to run Parts 1 & 2 of Day06 of Advent of Code 2024.

use clap::{Parser, ValueEnum};
use day06::{CUSTOM_INPUT, EXAMPLE_INPUT, FINAL_INPUT, Result, activate_global_default_tracing_subscriber,
            process_part1, process_part2};
use tracing::{self as tea, Level, instrument, level_filters::LevelFilter};

/// Choose to run Part 1 or 2 of Day06 of Advent of Code 2024.
#[derive(Parser, Debug)]
#[command(version, about, long_about, disable_help_subcommand = true, subcommand_help_heading = "input source")]
pub struct Args {
        /// Which Part to Run
        part:      Part,
        /// Input to use.
        input:     Option<Input>,
        /// Set level for active logging.
        #[arg(long, short, value_enum)]
        log:       Option<LevelFilter>,
        /// Set level of logs that errors will collect.
        #[arg(long, short, value_enum)]
        error_log: Option<LevelFilter>,
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
#[derive(Debug, Clone, ValueEnum)]
pub enum Input {
        /// Use the example input.
        Example,
        /// Use the full problem input.
        Full,
        /// Use a custom input.
        Custom,
}

fn main() -> Result<()> {
        let cli_user_args = Args::try_parse()?;

        // #[cfg(debug_assertions)]
        // skip setting up subscriber if both passed log values are `OFF`
        let _mb_writer_guard: Option<tracing_appender::non_blocking::WorkerGuard> =
                match (cli_user_args.log, cli_user_args.error_log) {
                        (Some(LevelFilter::OFF), Some(LevelFilter::OFF)) => None,
                        (_, _) => Some(activate_global_default_tracing_subscriber()
                                .maybe_env_default_level(cli_user_args.log)
                                .maybe_trace_error_level(cli_user_args.error_log)
                                .call()?),
                };

        let _enter = tea::debug_span!("main()").entered();
        tea::trace!("tracing subscriber set");
        tea::trace!(?cli_user_args);
        let part = cli_user_args.part;
        let inp = cli_user_args.input.unwrap_or_else(|| {
                tea::warn!("-- No input given.  Using Example input. -- ");
                Input::Example
        });
        tea::trace!(?part, ?inp);

        let solution = match (part, inp) {
                (Part::Part1, inp) => main_part1(inp),
                (Part::Part2, inp) => main_part2(inp),
        }?;
        println!("Calculated solution: {}", solution);
        tea::trace!("finishing main()");
        Ok(())
}

/// Run Part1_Lib code on binary-bound input1.txt
#[instrument(skip_all, ret(level = Level::DEBUG))]
pub fn main_part1(input: Input) -> Result<usize> {
        let input = match input {
                Input::Example => EXAMPLE_INPUT,
                Input::Full => FINAL_INPUT,
                Input::Custom => CUSTOM_INPUT,
        };
        let val = process_part1(input)?;
        tea::info!(?val, "Part 1 Process result.");
        Ok(val)
}

/// Run Part2_Lib code on binary-bound input2.txt
#[instrument(skip_all, ret(level = Level::DEBUG))]
pub fn main_part2(input: Input) -> Result<usize> {
        let input = match input {
                Input::Example => EXAMPLE_INPUT,
                Input::Full => FINAL_INPUT,
                Input::Custom => CUSTOM_INPUT,
        };
        let val = process_part2(input)?;
        tea::info!(?val, "Part 2 Process result.");
        Ok(val)
}
