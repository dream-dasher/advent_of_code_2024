//! Library code for Part 1 of Day02 of Advent of Code 2024.
//! `bin > part1_bin.rs` will run this code along with content of `input1.txt`

mod parse1;
use derive_more::derive::{Deref, DerefMut};
use parse1::parse_input1;
use tracing::{Level, instrument, trace};

#[expect(unused)]
use crate::{EXAMPLE_INPUT_1, FINAL_INPUT_1, ReportStatus, support::Result};

#[instrument(skip_all, ret(level = Level::INFO))]
pub fn process_part1(input: &str) -> Result<u64> {
        trace!(%input);
        let lines = parse_input1(input)?;
        let safe_lines_count = lines
                .iter()
                .map(|line| {
                        let first_derivative: Vec<Difference> = line
                                .windows(2)
                                .map(|w| match w {
                                        &[a, b] => Difference(b - a),
                                        _ => unreachable!("windows(2)"),
                                })
                                .collect();
                        first_derivative
                })
                .map(safety_status)
                .filter(|x| *x == ReportStatus::Safe)
                .count();
        Ok(safe_lines_count.try_into()?)
}

/// Difference between two values.  (Discrete First Derivative)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deref, DerefMut)]
pub struct Difference(i64);

/// Takes Vectors of Differences and returns a ReactorStatus
#[instrument(ret(level = Level::INFO))]
fn safety_status(diffs: Vec<Difference>) -> ReportStatus {
        // WARN: assuming no empty diffs
        let first_elem = diffs[0];
        for diff in diffs {
                tracing::debug!(?first_elem, ?diff);
                let is_out_of_magnitude = !(1..=3).contains(&diff.abs());
                let is_sign_change = (first_elem.is_positive() && diff.is_negative())
                        || (first_elem.is_negative() && diff.is_positive());
                tracing::debug!(is_out_of_magnitude, is_sign_change);
                if is_out_of_magnitude || is_sign_change {
                        return ReportStatus::Unsafe;
                }
        }
        // tracing::trace!("safe");
        ReportStatus::Safe
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
