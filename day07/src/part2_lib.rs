//! Library code for Part 2 of Day07 of Advent of Code 2024.

use tracing::{Level, instrument};

#[expect(unused)]
use crate::{Result, parse::parse_input};

#[instrument(skip_all, ret(level = Level::DEBUG))]
pub fn process_part2(input: &str) -> Result<u64> {
        tracing::event!(Level::TRACE, %input);
        // let _parsed_input = parse_input(input)?;
        todo!();
}

// #[cfg(test)]
// mod tests {
//         use indoc::indoc;
//         use quickcheck::TestResult;
//         use quickcheck_macros::quickcheck;
//         use rand::Rng;
//         use test_log::test;
//         use tracing::{instrument};

//         use super::*;
//         use crate::EXAMPLE_INPUT;
// //         use crate::FINAL_INPUT;

//         #[test]
//         #[instrument]
//         fn test_process_example() -> Result<()> {
//                 let input = EXAMPLE_INPUT;
//                 let expected = todo!();
//                 assert_eq!(process_part2(input)?, expected);
//                 Ok(())
//         }

//         // /// Test's expected value to be populated after solution verification.
//         // #[test]
//         // #[instrument]
//         // fn test_process_problem_input() -> Result<()> {
//         //         let input = FINAL_INPUT;
//         //         let expected = todo!();
//         //         assert_eq!(process_part2(input)?, expected);
//         //         Ok(())
//         // }
// }
