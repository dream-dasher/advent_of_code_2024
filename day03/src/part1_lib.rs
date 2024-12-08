//! Library code for Part 1 of Day03 of Advent of Code 2024.
//! `bin > part1_bin.rs` will run this code along with content of `input1.txt`

use tracing::{self as tea, Level, instrument};

use crate::{MulPair, Result, parse::parse_input};

#[instrument(skip_all, ret(level = Level::DEBUG))]
pub fn process_part1(input: &str) -> Result<u64> {
        tea::trace!(%input);
        let parsed_input = parse_input(input)?;
        Ok(calculate_solution(parsed_input))
}

/// Process solution on prased input.
#[instrument(skip_all, ret(level = Level::DEBUG))]
fn calculate_solution(pairs_vec: Vec<MulPair>) -> u64 {
        pairs_vec
                .iter()
                .map(|pair| pair.self_multiply())
                .inspect(|mul_pair| tea::debug!(mul_pair))
                .sum()
}

// #[cfg(test)]
// mod tests {
//         use indoc::indoc;
//         use quickcheck::TestResult;
//         use quickcheck_macros::quickcheck;
//         use rand::Rng;
//         use test_log::test;
//         use tracing::{self as tea, instrument};

//         use super::*;
//         use crate::{EXAMPLE_INPUT, FINAL_INPUT};

//         #[test]
//         #[instrument]
//         fn test_process_example() -> Result<()> {
//                 let input = EXAMPLE_INPUT;
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
//         //         let input = FINAL_INPUT;
//         //         let expected = todo!();
//         //         assert_eq!(process_part1(input)?, expected);
//         //         Ok(())
//         // }
// }
