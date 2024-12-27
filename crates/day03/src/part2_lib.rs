//! Library code for Part 2 of Day03 of Advent of Code 2024.
//! `bin > part2_bin.rs` will run this code along with content of `input2.txt`

use tracing::{self as tea, Level, instrument};

use crate::{MulPair, Result, parse::parse_input_2};

#[instrument(skip_all, ret(level = Level::DEBUG))]
pub fn process_part2(input: &str) -> Result<u64> {
        tea::trace!(%input);
        let parsed_input = parse_input_2(input)?;
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
        use crate::{EXAMPLE_INPUT_2, FINAL_INPUT};

        #[test]
        #[instrument]
        fn part2_example_input_test() -> Result<()> {
                let input = EXAMPLE_INPUT_2;
                let expected = 48;
                assert_eq!(process_part2(input)?, expected);
                Ok(())
        }

        #[test]
        #[instrument]
        fn part2_final_input_test() -> Result<()> {
                let input = FINAL_INPUT;
                let expected = 90_044_227;
                assert_eq!(process_part2(input)?, expected);
                Ok(())
        }
}
