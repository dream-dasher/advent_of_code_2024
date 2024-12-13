//! Library code for Part 2 of Day05 of Advent of Code 2024.
//! `bin > part2_bin.rs` will run this code along with content of `input2.txt`

use tracing::{self as tea, Level, instrument};

use crate::{Result,
            parse::{PAGE_RELATIONS, parse_input},
            support::ErrKindDay05};

#[instrument(skip_all, ret(level = Level::DEBUG))]
pub fn process_part2(input: &str) -> Result<u64> {
        tea::trace!(%input);
        let mut total = 0;
        let (page_relations, mut to_check) = parse_input(input)?;
        PAGE_RELATIONS
                .set(page_relations)
                .map_err(|_| ErrKindDay05::StaticPageRelationsSetFailure)?;
        debug_assert!(match PAGE_RELATIONS.get().unwrap().is_total_ordering_shape() {
                true => {
                        tea::info!("Total ordering shape detected.  Local detection of order sufficient.");
                        true
                }
                false => {
                        tea::error!("Non-total ordering shape detected. Code not implemented.");
                        Err(ErrKindDay05::NonTotalOrderingShape)?
                }
        });
        for _ in 0..to_check.len() {
                let mut seq = to_check.pop().unwrap();
                if !seq.windows(2).all(|page_slice| {
                        match page_slice {
                                &[l, r] => PAGE_RELATIONS.get().unwrap().say_pair_are_ordered((l, r)),
                                _ => unreachable!(),
                        }
                        .unwrap_or(true)
                }) {
                        seq.sort_unstable();
                        // add value of middle element
                        total += *seq[seq.len() / 2] as u64;
                        debug_assert!(seq.len() % 2 == 1)
                }
        }
        Ok(total)
}

#[cfg(test)]
mod tests {
        use test_log::test;
        use tracing::instrument;

        use super::*;
        use crate::{EXAMPLE_INPUT, FINAL_INPUT};

        #[test]
        #[instrument]
        fn test_process_example() -> Result<()> {
                let input = EXAMPLE_INPUT;
                let expected = 123;
                assert_eq!(process_part2(input)?, expected);
                Ok(())
        }

        /// Test's expected value to be populated after solution verification.
        /// NOTE: `#[ignore]` is set for this test by default.
        #[ignore]
        #[test]
        fn test_process_problem_input() -> Result<()> {
                let input = FINAL_INPUT;
                let expected = 5799;
                assert_eq!(process_part2(input)?, expected);
                Ok(())
        }
}
