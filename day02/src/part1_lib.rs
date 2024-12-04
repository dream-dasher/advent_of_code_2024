//! Library code for Part 1 of Day02 of Advent of Code 2024.
//! `bin > part1_bin.rs` will run this code along with content of `input1.txt`

mod parse1;
use parse1::parse_input1;
use tracing::{instrument, trace};

#[expect(unused)]
use crate::{EXAMPLE_INPUT_1, FINAL_INPUT_1, Status, support::Result};

#[instrument(skip(input))]
pub fn process_part1(input: &str) -> Result<u64> {
        trace!(%input);
        let mut statuses = Vec::new();
        let lines = parse_input1(input)?;
        for line in lines {
                let wins = line.windows(2);
                let diffs: Vec<i64> = wins.map(|x| x[0] - x[1]).collect();
                tracing::info!(?diffs);
                statuses.push(is_safe_1(diffs));
        }
        tracing::info!(?statuses);
        let sum_safes = statuses.iter().filter(|x| **x == Status::Safe).count().try_into()?;
        Ok(sum_safes)
}

fn is_safe_1(diffs: Vec<i64>) -> Status {
        // WARN: assuming no empty diffs
        let first_elem = diffs[0];
        for diff in diffs {
                tracing::debug!(?first_elem, ?diff);
                let is_out_of_magnitude = !(1..=3).contains(&diff.abs());
                let is_sign_change = (first_elem.is_positive() && diff.is_negative())
                        || (first_elem.is_negative() && diff.is_positive());
                tracing::debug!(is_out_of_magnitude, is_sign_change);
                if is_out_of_magnitude || is_sign_change {
                        return Status::Unsafe;
                }
        }
        tracing::trace!("safe");
        Status::Safe
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
