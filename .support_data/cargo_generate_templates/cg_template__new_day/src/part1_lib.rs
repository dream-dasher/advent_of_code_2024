//! Library code for Part 1 of {{ project-name | title_case }} of Advent of Code 2024.
//! `bin > part1_bin.rs` will run this code along with conent of `input1.txt`

use tracing::instrument;

#[expect(unused)]
use crate::{EXAMPLE_INPUT_1, FINAL_INPUT_1, support::Result};

#[instrument]
pub fn process_part1(input: &str) -> Result<u64> {
        todo!();
}

// #[cfg(test)]
// mod tests {
//         use indoc::indoc;

//         use super::*;

//         #[test]
//         fn test_process_example() -> Result<()> {
//                 let input = EXAMPLE_INPUT_1
//                 let expected = todo!();
//                 assert_eq!(process_part1(input)?, expected);
//                 Ok(())
//         }

//         // /// Test's expected value to be populated after solution verification.
//         // /// NOTE: `#[ignore]` is set for this test by default.
//         // #[ignore]
//         // #[test]
//         // fn test_process_problem_input() -> Result<()> {
//         //         tracing_subscriber::fmt::init();
//         //         let file_input = include_str!("../input1.txt");
//         //         let expected = todo!();
//         //         assert_eq!(process(file_input)?, expected);
//         //         Ok(())
//         // }
// }
