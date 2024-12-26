//! CLI interface to run Parts 1 & 2 of {{ project-name | upper_camel_case }} of Advent of Code 2024.

use clap::{Parser, ValueEnum};
use {{ project-name | snake_case }}::{CUSTOM_INPUT, EXAMPLE_INPUT, FINAL_INPUT, Result, activate_global_default_tracing_subscriber, process_part1, process_part2};
use tracing::{Level, instrument, level_filters::LevelFilter};

/// Choose to run Part 1 or 2 of {{ project-name | upper_camel_case }} of Advent of Code 2024.
#[derive(Parser, Debug)]
#[command(
        version, about, long_about,
)]
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
/// Part 1 or 2 of {{ project-name | upper_camel_case }} of Advent of Code 2024.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Part {
        /// Part 1, of day {{ project-name | upper_camel_case }}
        #[value(alias = "1", alias = "i", alias = "I", alias = "one")]
        Part1,
        /// Part 2, of day {{ project-name | upper_camel_case }}
        #[value(alias = "2", alias = "ii", alias = "II", alias = "two")]
        Part2,
}
/// Data to use as input.
#[derive(Debug, Clone, Copy, ValueEnum)]
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
        let _enter = tracing::debug_span!("main()").entered();
        tracing::event!(Level::TRACE, "tracing subscriber set");
        tracing::event!(Level::TRACE, ?cli_user_args);
        let part = cli_user_args.part;
        let inp = cli_user_args.input.unwrap_or_else(|| {
                tracing::event!(Level::WARN, "-- No input given.  Using Example input. -- ");
                Input::Example
        });
        tracing::event!(Level::TRACE, ?part, ?inp);

        let solution = match (part, inp) {
                (Part::Part1, inp) => main_part1(inp),
                (Part::Part2, inp) => main_part2(inp),
        }?;
        println!("Calculated solution: {}", solution);
        tracing::event!(Level::TRACE, "finishing main()");
        Ok(())
}

/// Run Part1_Lib code on binary-bound input1.txt
#[instrument(ret(level = Level::DEBUG))]
pub fn main_part1(input: Input) -> Result<u64> {
        let input = match input {
                Input::Example => EXAMPLE_INPUT,
                Input::Full => FINAL_INPUT,
                Input::Custom => CUSTOM_INPUT,
        };
        let val = process_part1(input)?;
        tracing::event!(Level::INFO, ?val, "Part 1 Process result.");
        Ok(val)
}

/// Run Part2_Lib code on binary-bound input2.txt
#[instrument(ret(level = Level::DEBUG))]
pub fn main_part2(input: Input) -> Result<u64> {
        let input = match input {
                Input::Example => EXAMPLE_INPUT,
                Input::Full => FINAL_INPUT,
                Input::Custom => CUSTOM_INPUT,
        };
        let val = process_part2(input)?;
        tracing::event!(Level::INFO, ?val, "Part 2 Process result.");
        Ok(val)
}
