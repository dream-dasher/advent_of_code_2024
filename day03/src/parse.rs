//! Raw-input parsing code for Day03 of Advent of Code 2024.
//!
//! ## NOTE:
//! u64::MAX == 18_446_744_073_709_551_615

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
        use test_log::test;
        use tracing::{self as tea, instrument};

        use super::*;

        #[test]
        #[instrument]
        fn parse_example_test() -> Result<()> {
                tea::warn!("--------------Running test_example---------------");
                let input = indoc!("
                        xmul(1,2)%&mul[3,7]!@^do_not_mul(3,4)+mul(32,64]then(mul(5,6)mul(7,8))
                        mul(11,22)%&mul[3,7]!@^do_not_mul(33,44)+mul(53,65]then(mul(55,66)mul(77,88))
                        mul(111,222)
                        mul(1111,2222)
                        mul(8,
                        8)
                        mu
                        l(9,
                        9)
                        mul(11111111111,222222222)
                        mul(001,002)
                        ");
                let expected: Vec<MulPair> = vec![
                        (1, 2),
                        (3, 4),
                        (5, 6),
                        (7, 8),
                        (11, 22),
                        (33, 44),
                        (55, 66),
                        (77, 88),
                        (111, 222),
                        (1111, 2222),
                        (11111111111, 222222222),
                        (1, 2),
                ]
                .into_iter()
                .map(|(left, right)| MulPair::new(left, right))
                .collect();
                assert_eq!(parse_input(input)?, expected);
                Ok(())
        }

        #[quickcheck]
        #[instrument]
        fn qc_example_quickcheck_test(inp_pairs: Vec<(u64, u64)>) -> TestResult {
                tea::debug!("--------------Running qc_example_quickcheck---------------");
                tea::debug!(?inp_pairs);
                let inp_string = clean_input_generator(inp_pairs.clone()).unwrap_or_else(|e| {
                        tea::error!(?e, "input error");
                        panic!();
                });
                tea::debug!(?inp_string);

                let parsed_and_deconstructed: Vec<(u64, u64)> = parse_input(&inp_string)
                        .unwrap()
                        .into_iter()
                        .map(|mul_pair| mul_pair.into())
                        .collect();
                tea::debug!(?parsed_and_deconstructed);

                let all_pairs_equal = inp_pairs
                        .iter()
                        .zip(parsed_and_deconstructed.iter())
                        .all(|(expected, actual)| expected == actual);
                TestResult::from_bool(all_pairs_equal)
        }
        #[instrument]
        fn clean_input_generator(inp_pairs: Vec<(u64, u64)>) -> Result<String> {
                let inp_string = inp_pairs
                        .iter()
                        .map(|(left, right)| format!("mul({}, {})", left, right))
                        .collect::<Vec<_>>()
                        .join("\n");
                Ok(inp_string)
        }
}
