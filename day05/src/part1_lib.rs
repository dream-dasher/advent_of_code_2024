//! Library code for Part 1 of Day05 of Advent of Code 2024.
//! `bin > part1_bin.rs` will run this code along with content of `input1.txt`

use tracing::{self as tea, Level, instrument};

use crate::{Result, parse::parse_input, support::ErrKindDay05};

#[instrument(skip_all, ret(level = Level::DEBUG))]
pub fn process_part1(input: &str) -> Result<u64> {
        tea::trace!(%input);
        let mut total = 0;
        let (page_relations, to_check) = parse_input(input)?;
        match page_relations.is_total_ordering_shape() {
                true => tea::info!("Total ordering shape detected.  Local detection of order sufficient."),
                false => {
                        tea::error!("Non-total ordering shape detected. Code not implemented.");
                        Err(ErrKindDay05::NonTotalOrderingShape)?;
                }
        }
        for seq in to_check {
                if seq.windows(2).all(|page_slice| {
                        match page_slice {
                                &[l, r] => page_relations.say_pair_are_ordered((l, r)),
                                _ => unreachable!(),
                        }
                        .unwrap_or(true)
                }) {
                        // add value of middle element
                        total += *seq[seq.len() / 2] as u64;
                        debug_assert!(seq.len() % 2 == 1)
                }
        }
        Ok(total)
}

#[cfg(test)]
mod tests {
        use indoc::indoc;
        use test_log::test;
        use tracing::instrument;

        use super::*;
        use crate::{EXAMPLE_INPUT, FINAL_INPUT};

        #[test]
        #[instrument]
        fn spot_test() -> Result<()> {
                let input = indoc!("
                        1|2
                        2|3
                        1|4

                        1,2,3
                        3,2,1");
                let expected = 2;
                assert_eq!(process_part1(input)?, expected);
                Ok(())
        }

        #[test]
        #[instrument]
        fn test_process_example() -> Result<()> {
                let input = EXAMPLE_INPUT;
                let expected = 143;
                assert_eq!(process_part1(input)?, expected);
                Ok(())
        }

        /// Test's expected value to be populated after solution verification.
        /// NOTE: `#[ignore]` is set for this test by default.
        #[test]
        #[instrument]
        fn test_process_problem_input() -> Result<()> {
                let input = FINAL_INPUT;
                let expected = 5268;
                assert_eq!(process_part1(input)?, expected);
                Ok(())
        }
}
