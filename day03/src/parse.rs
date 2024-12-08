use derive_more::derive::{Add, Constructor, Display, Div, From, Into, Mul, Sub};
use regex::Regex;
use tracing::{self as tea, Level, instrument};

use crate::Result;

/// Multiplies pair internally.
/// Does **NOT** check for overflow.
#[derive(Debug, Add, Sub, Mul, Div, Display, PartialEq, Eq, Clone, Copy, Constructor, From, Into)]
#[display("({}, {})", left_num, right_num)]
pub struct MulPair {
        left_num:  u64,
        right_num: u64,
}
impl MulPair {
        pub fn self_multiply(&self) -> u64 {
                self.left_num * self.right_num
        }
}

/// Parse txt input: extracting number pairs from text.
/// No attention is paid to individual lines.
///
/// ## External:
/// [regex101](https://regex101.com)
#[instrument(skip_all, ret(level = Level::TRACE))]
pub fn parse_input(raw_input: &str) -> Result<Vec<MulPair>> {
        tea::trace!(?raw_input);
        const REGEX_MUL_PAIR: &str = r"mul\((?<left_num>\d+),(?<right_num>\d+)\)";
        let re = Regex::new(REGEX_MUL_PAIR).expect("string should be valid regex");
        let mult_pairs: Result<Vec<_>> = {
                let _enter = tea::debug_span!("Parsing").entered();
                re.captures_iter(raw_input)
                        .enumerate()
                        .map(|(i2, cap)| {
                                let (raw, [left_num_str, right_num_str]) = cap.extract();
                                tea::trace!(?raw, ?left_num_str, ?right_num_str, i2);
                                (left_num_str, right_num_str)
                        })
                        .map(|(left_str, right_str)| {
                                let left_num = left_str.parse::<u64>()?;
                                let right_num = right_str.parse::<u64>()?;
                                let mul_pair = MulPair::new(left_num, right_num);
                                tea::debug!(%mul_pair);
                                Ok(mul_pair)
                        })
                        .collect()
        };
        tea::info!(?mult_pairs);
        mult_pairs
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
