//! Library code for Day02 of Advent of Code 2024.

mod part1_lib;
mod part2_lib;
mod support;

pub use part1_lib::process_part1;
pub use part2_lib::process_part2;
pub use support::{Error, Result, generate_tracing_subscriber};

pub const FINAL_INPUT_1: &str = include_str!("../data/final_input1.txt");
pub const FINAL_INPUT_2: &str = include_str!("../data/final_input2.txt");
pub const EXAMPLE_INPUT_1: &str = include_str!("../data/example_input1.txt");
pub const EXAMPLE_INPUT_2: &str = include_str!("../data/example_input2.txt");

/// Safe: all levels same sign and (1..=3).contains()
#[derive(Debug, PartialEq, Eq)]
pub enum ReportStatus {
    Safe,
    Unsafe,
}
