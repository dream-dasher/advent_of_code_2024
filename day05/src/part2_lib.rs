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

        if cfg!(debug_assertions) {
                // throwing error: spantrace collection
                PAGE_RELATIONS.get().unwrap().verify_total_ordering_shape()?
        }
        let _tea = tea::info_span!(target: "q_pop", "abberant popping").entered();
        tea::trace!("hello?");
        for i in 0..to_check.len() {
                tea::trace!(target: "q_pop",i, ?to_check);
                let mut seq = to_check.pop().unwrap();
                tea::trace!(target: "q_pop",?seq);
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
                tea::warn!(
                        "OnceLock(|Cell) means that the single-process tests of default `Cargo test` will interfere with one another.  Use `Cargo Nextest`. (Unclear if mutex to force test serialization will be sufficient, as the OnceLock is static."
                );
                assert_eq!(process_part2(input)?, expected);
                Ok(())
        }

        /// Test's expected value to be populated after solution verification.
        #[test]
        fn test_process_problem_input() -> Result<()> {
                let input = FINAL_INPUT;
                let expected = 5799;
                tea::warn!(
                        "OnceLock(|Cell) means that the single-process tests of default `Cargo test` will interfere with one another.  Use `Cargo Nextest`. (Unclear if mutex to force test serialization will be sufficient, as the OnceLock is static."
                );
                assert_eq!(process_part2(input)?, expected);
                Ok(())
        }
}
