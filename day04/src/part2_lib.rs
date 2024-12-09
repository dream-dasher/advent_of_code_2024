//! Library code for Part 2 of Day04 of Advent of Code 2024.
//! `bin > part2_bin.rs` will run this code along with content of `input2.txt`

use std::cell::LazyCell;

use regex::Regex;
use tracing::{self as tea, Level, instrument};

use crate::{ErrKindDay04, Result};

// we can't use `format!` with the const value.
#[expect(unused)]
const REGEX_MAS_TEMPLATE: &str = r"(M.M(.|\n){{{r_minus_one}}}A(.|\n){{{r_minus_one}}}S.S|M.S(.|\n){{{r_minus_one}}}A(.|\n){{{r_minus_one}}}M.S|S.M(.|\n){{{r_minus_one}}}A(.|\n){{{r_minus_one}}}S.M|S.S(.|\n){{{r_minus_one}}}A(.|\n){{{r_minus_one}}}M.M)";

#[instrument(skip_all, ret(level = Level::DEBUG))]
pub fn process_part2(input: &str) -> Result<u64> {
        let row_length = input
                .lines()
                .next()
                .ok_or(ErrKindDay04::NoInputLines {
                        source_input: (input.to_string()),
                })?
                .len();
        recursive_regex_search(input, row_length)
}

/// Count patterns using a simple regex, recursively re-calling it with start position shifted to allow pattern overlap.
///
/// ## Possible Improvements:
/// Remarkably, and interestingly, slow.
/// (Use on a smaller input size siginficicantly reduce runtime; so not regex compilation.)
/// Would be fun to:
/// - split out the OR in regex and run each serially
/// - Rayon & manual threads over:
///   - each regex
///   - run on iter, and split input [start_x+1..start_y], [start_y+1..start_z], etc.-- allowing parallelization in search with a single regex
///   - have some channel or other shared counting mechanism (allowing function frames & info to be dropped? <-- not sure how this works entirely!; but perhaps with thread spawning
/// - Adaptation of the custom state machine used in Part_1
/// - Is error handling code (which bubbles up the recursing callers) part of the issue?
#[instrument(ret(level = Level::TRACE))]
pub fn recursive_regex_search(raw_input: &str, row_length: usize) -> Result<u64> {
        let regex_mas_sized = format!(
                r"(M.M(.|\n){{{r_minus_one}}}A(.|\n){{{r_minus_one}}}S.S|M.S(.|\n){{{r_minus_one}}}A(.|\n){{{r_minus_one}}}M.S|S.M(.|\n){{{r_minus_one}}}A(.|\n){{{r_minus_one}}}S.M|S.S(.|\n){{{r_minus_one}}}A(.|\n){{{r_minus_one}}}M.M)",
                r_minus_one = row_length - 1
        );
        tea::trace!(?regex_mas_sized, ?row_length, "Regex set for given row_length.");
        let re = LazyCell::new(|| Regex::new(&regex_mas_sized).unwrap());
        tea::trace!(?re, "Regex Lazy Cell compiled.");

        let Some(found_match) = re.find(raw_input) else {
                tea::debug!("No more matches found. Starting return sequence.");
                return Ok(0);
        };

        let mut count = 1;
        let match_start_position = found_match.start();
        tea::info!(match_start_position);
        count += recursive_regex_search(&raw_input[match_start_position + 1..], row_length)?;
        tea::info!(?found_match);
        Ok(count)
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
//                 assert_eq!(process_part2(input)?, expected);
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
//         //         assert_eq!(process_part2(input)?, expected);
//         //         Ok(())
//         // }
// }
