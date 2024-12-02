//! Library code for {{ project-name | title_case }} of Advent of Code 2024.

pub mod part1_lib;
pub mod part2_lib;
pub mod support;

pub use part1_lib::process_part1;
pub use part2_lib::process_part2;
pub use support::{Error, Result, generate_tracing_subscriber};
