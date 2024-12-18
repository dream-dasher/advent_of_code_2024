//! Library code for Part 2 of Day04 of Advent of Code 2024.
//! `bin > part2_bin.rs` will run this code along with content of `input2.txt`
//!
//! ## Reference Strings
//! Can't be used with `format!` as format only takes string literals.
//!
//! const REGEX_MAS_TEMPLATE: &str = r"(M.M(.|\n){{{r_minus_one}}}A(.|\n){{{r_minus_one}}}S.S|M.S(.|\n){{{r_minus_one}}}A(.|\n){{{r_minus_one}}}M.S|S.M(.|\n){{{r_minus_one}}}A(.|\n){{{r_minus_one}}}S.M|S.S(.|\n){{{r_minus_one}}}A(.|\n){{{r_minus_one}}}M.M)";
//! const REGEX_MM_TEMPLATE: &str = r"M.M(.|\n){{{r_minus_one}}}A(.|\n){{{r_minus_one}}}S.S";
//! const REGEX_MS_TEMPLATE: &str = r"M.S(.|\n){{{r_minus_one}}}A(.|\n){{{r_minus_one}}}M.S";
//! const REGEX_SM_TEMPLATE: &str = r"S.M(.|\n){{{r_minus_one}}}A(.|\n){{{r_minus_one}}}S.M";
//! const REGEX_SS_TEMPLATE: &str = r"S.S(.|\n){{{r_minus_one}}}A(.|\n){{{r_minus_one}}}M.M";

use std::{sync::Arc, thread};

use regex::Regex;
use tracing::{self as tea, Level, instrument};

use crate::{ErrKindDay04, Result};

#[instrument(skip_all, ret(level = Level::DEBUG))]
pub fn process_part2(input: &str) -> Result<u64> {
        let row_length = input
                .lines()
                .next()
                .ok_or(ErrKindDay04::NoInputLines {
                        source_input: (input.to_string()),
                })?
                .len();
        cross_mas_regex_count(input, row_length)
}

/// Count 'cross-mas' patterns in rectangular (by char) input text.
/// Treats patterns as 1D and works on assumption of a consistent row-length across rows.
///
/// Spawns 4 threads, one for each of the 4 'cross-mas' patterns.
///
/// ## Warning:
/// A zero-width match will break the loop logic, advancing the slice index despite potentially past the end of the &str.
/// As this is designed for a hard-coded regex I am not adding bounds or found pattern checking to address.
/// (Any empty matches would be a logic error in the context of this function.)
///
/// e.g. `r"|..."` has a zero-width pattern on the left. (this typo may manifest as an indexing error)
///
/// ## Possible Improvements:
/// Would be fun to:
/// - Rayon & manual threads over:
///   - each regex
///   - run on iter, and split input [start_x+1..start_y], [start_y+1..start_z], etc.-- allowing parallelization in search with a single regex
///   - have some channel or other shared counting mechanism (allowing function frames & info to be dropped? <-- not sure how this works entirely!; but perhaps with thread spawning
/// - Adaptation of the custom state machine used in Part_1
/// - Is error handling code (which bubbles up the recursing callers) part of the issue?
#[instrument(skip(raw_input), ret(level = Level::TRACE))]
fn cross_mas_regex_count(raw_input: &str, row_length: usize) -> Result<u64> {
        let (re_mm, re_ms, re_sm, re_ss) = compile_mas_regexes(row_length);
        // let shared_input = Arc::new(raw_input.to_string());
        let shareable_input_string = Arc::new(raw_input.to_string());
        let mut handles = Vec::with_capacity(4);
        tea::debug!(?handles);
        tea::trace!(?shareable_input_string);
        for (i, re) in [re_mm, re_ms, re_sm, re_ss].into_iter().enumerate() {
                let tea_span = tea::info_span!("Generating thread", ?re, i);
                let _tea = tea_span.enter();
                tea::info!("hi");
                let shared_input = shareable_input_string.clone();
                let tea_span = tea_span.clone();
                let thread_handle = thread::spawn(move || {
                        let _tea = tea_span.clone().entered();
                        let mut total = 0;
                        let mut match_start_position = 0;
                        while let Some(found_match) = re.find(&shared_input[match_start_position..])
                        {
                                total += 1;
                                match_start_position += found_match.start() + 1;
                                tea::debug!(match_start_position, total, i);
                                tea::trace!(?found_match);
                        }
                        tea::debug!("No more matches found.");
                        total
                });
                handles.push(thread_handle);
        }
        let total = handles.into_iter().map(|t| t.join().unwrap()).sum();
        Ok(total)
}

/// Convenience function to compile regexes for the cross-mas pattern.
///
/// ## Return order:
/// ```text
/// (0)     (1)     (2)     (3)
/// M M  |  M S  |  S M  |  S S
///  A   |   A   |   A   |   A
/// S S  |  M S  |  S M  |  M M
/// ```
#[instrument(ret(level = Level::TRACE))]
fn compile_mas_regexes(row_length: usize) -> (Regex, Regex, Regex, Regex) {
        let regex_mm_sized = format!(
                r"M.M(.|\n){{{r_minus_one}}}A(.|\n){{{r_minus_one}}}S.S",
                r_minus_one = row_length - 1
        );
        let regex_ms_sized = format!(
                r"M.S(.|\n){{{r_minus_one}}}A(.|\n){{{r_minus_one}}}M.S",
                r_minus_one = row_length - 1
        );
        let regex_sm_sized = format!(
                r"S.M(.|\n){{{r_minus_one}}}A(.|\n){{{r_minus_one}}}S.M",
                r_minus_one = row_length - 1
        );
        let regex_ss_sized = format!(
                r"S.S(.|\n){{{r_minus_one}}}A(.|\n){{{r_minus_one}}}M.M",
                r_minus_one = row_length - 1
        );

        let re_mm = Regex::new(&regex_mm_sized).unwrap();
        let re_ms = Regex::new(&regex_ms_sized).unwrap();
        let re_sm = Regex::new(&regex_sm_sized).unwrap();
        let re_ss = Regex::new(&regex_ss_sized).unwrap();
        (re_mm, re_ms, re_sm, re_ss)
}
#[cfg(test)]
mod tests {
        use indoc::indoc;
        use test_log::test;
        use tracing::{self as tea, instrument};

        use super::*;
        use crate::{EXAMPLE_INPUT_2, FINAL_INPUT};

        #[test]
        #[instrument]
        fn mas_test() -> Result<()> {
                let input = indoc!("
                        M.M...
                        .A....
                        S.S.S.
                        ...A..
                        ..M.M.
                        ");
                let expected = 2;
                assert_eq!(process_part2(input)?, expected);
                Ok(())
        }

        #[test]
        #[instrument]
        fn part2_example_input_mas_test() -> Result<()> {
                let input = EXAMPLE_INPUT_2;
                let expected = 9;
                assert_eq!(process_part2(input)?, expected);
                Ok(())
        }

        // #[ignore = "Long runtime without release optimization"]
        #[test]
        #[instrument]
        fn part2_final_input_mas_test() -> Result<()> {
                tea::warn!("Long runtime without release optimization");
                let input = FINAL_INPUT;
                let expected = 1_910;
                assert_eq!(process_part2(input)?, expected);
                Ok(())
        }
}
