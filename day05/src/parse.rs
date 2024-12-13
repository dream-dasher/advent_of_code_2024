//! Raw-input parsing code for Day05 of Advent of Code 2024.
//!
//! ## Input patterns
//! - Two sections
//!   - Relations
//!     - <less_than>|<greater_than>
//!   - <blankline>
//!   - Sequences
//!     - n,n,n,n,n,n...   <--should be an odd number of values

// use derive_more::derive::{Constructor, Deref, DerefMut, From, Into};
use indoc::indoc;
use regex::Regex;
use tracing::{self as tea, Level, instrument};

use crate::Result;

/// Parse txt input ...
#[instrument(skip_all, ret(level = Level::TRACE))]
#[expect(unused)]
pub fn parse_input(raw_input: &str) -> Result<()> {
        const REGEX: &str = r"";
        todo!()
}

#[cfg(test)]
mod tests {
        use indoc::indoc;
        use quickcheck::TestResult;
        use quickcheck_macros::quickcheck;
        use rand::Rng;
        use test_log::test;
        use tracing::{self as tea, instrument};

        use super::*;

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
                assert_eq!(input.lines().count(), expected);
                Ok(())
        }

        #[instrument]
        fn example_input_generator(sum: u16, step_range_inclusive: (u8, u8)) -> Option<Vec<i64>> {
                let (low_step, high_step) = step_range_inclusive;
                let low_step = low_step as i64;
                let high_step = high_step as i64;
                let mut sum = sum as i64;

                if low_step >= high_step {
                        tea::trace!(?low_step, ?high_step);
                        return None;
                }
                let mut rng = rand::thread_rng();
                let mut out = Vec::new();
                while sum > 0 {
                        let step = rng.gen_range(low_step..=high_step).min(sum);
                        out.push(step);
                        sum -= step;
                        tea::debug!(?step, ?sum);
                }
                Some(out)
        }

        #[quickcheck]
        #[instrument]
        fn qc_example_quickcheck(sum: u16, step_range_inclusive: (u8, u8)) -> TestResult {
                let Some(vector) = example_input_generator(sum, step_range_inclusive) else {
                        return TestResult::discard();
                };
                let vector_sum: i64 = vector.iter().sum();
                TestResult::from_bool(sum as i64 == vector_sum)
        }
}
