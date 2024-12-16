//! Raw-input parsing code for Day06 of Advent of Code 2024.

use std::io;

// use derive_more::derive::{Constructor, Deref, DerefMut, From, Into};
use tracing::{self as tea, Level, instrument};

use crate::Result;

/// Parse txt input ...
#[instrument(skip_all, ret(level = Level::TRACE))]
pub fn parse_input(raw_input: &str) -> Result<()> {
        println!("\n{}\n", raw_input);
        dirty_pause()?;
        todo!()
}

/// Quick and dirty pause button so I can watch as program runs.
#[instrument]
fn dirty_pause() -> Result<()> {
        println!("Press Enter to continue...");
        let mut _input = String::new();
        let read_in = io::stdin().read_line(&mut _input)?;
        tea::debug!(?read_in);
        Ok(())
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
        fn test_example() -> Result<()> {
                tea::warn!("--------------Running test_example---------------");
                let input = indoc!("
                        0 6 4 2 1
                        1 2 7 8 9
                        2 7 6 2 1
                        3 3 2 4 5
                        4 6 4 4 1
                        5 3 6 7 9");
                let expected = 6;
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
