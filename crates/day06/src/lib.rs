//! Library code for Day06 of Advent of Code 2024.

mod parse;
mod part1_lib;
mod part2_lib;
pub mod support;

pub use parse::parse_input;
pub use part1_lib::{PopulatedMaze, process_part1};
pub use part2_lib::{PopulatedMazeWHSet, UpdateError, process_part2};
pub use support::{Error, Result, activate_global_default_tracing_subscriber};

pub const FINAL_INPUT: &str = include_str!("../data/final_input.txt");
pub const EXAMPLE_INPUT: &str = include_str!("../data/example_input.txt");
pub const CUSTOM_INPUT: &str = include_str!("../data/custom_input.txt");
