//! Library code for Day04 of Advent of Code 2024.

mod parse;
mod part1_lib;
mod part2_lib;
mod support;

pub use parse::parse_input_1;
pub use part1_lib::process_part1;
pub use part2_lib::process_part2;
pub use support::{ErrKindDay04, Error, Result, generate_tracing_subscriber};

pub const FINAL_INPUT: &str = include_str!("../data/final_input.txt");
pub const EXAMPLE_INPUT_1: &str = include_str!("../data/example_input_1.txt");
pub const EXAMPLE_INPUT_2: &str = include_str!("../data/example_input_2.txt");
pub const CUSTOM_INPUT: &str = include_str!("../data/custom_input.txt");
