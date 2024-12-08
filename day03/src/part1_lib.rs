//! Library code for Part 1 of Day03 of Advent of Code 2024.
//! `bin > part1_bin.rs` will run this code along with content of `input1.txt`

use tracing::{self as tea, Level, instrument};

use crate::{MulPair, Result, parse::parse_input_1};

#[instrument(skip_all, ret(level = Level::DEBUG))]
pub fn process_part1(input: &str) -> Result<u64> {
        tea::trace!(%input);
        let parsed_input = parse_input_1(input)?;
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

#[cfg(test)]
mod tests {
        use test_log::test;
        use tracing::instrument;

        use super::*;
        use crate::{EXAMPLE_INPUT_1, FINAL_INPUT};

        #[test]
        #[instrument]
        fn part1_example_input_test() -> Result<()> {
                let input = EXAMPLE_INPUT_1;
                let expected = 161;
                assert_eq!(process_part1(input)?, expected);
                Ok(())
        }

        #[test]
        #[instrument]
        fn part1_final_input_test() -> Result<()> {
                let input = FINAL_INPUT;
                let expected = 184_511_516;
                assert_eq!(process_part1(input)?, expected);
                Ok(())
        }
}
