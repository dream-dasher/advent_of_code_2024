//! Raw-input parsing code for Day03 of Advent of Code 2024.
//!
//! ## NOTE:
//! u64::MAX == 18_446_744_073_709_551_615

use derive_more::derive::{Add, Constructor, Display, Div, From, Into, Mul, Sub};
use regex::Regex;
use tracing::{self as tea, Level, instrument};

use crate::Result;

const REGEX_MUL_PAIR: &str = r"mul\((?<left_num>\d+),(?<right_num>\d+)\)";

/// Multiplies pair internally.
/// Does **NOT** check for overflow.
#[derive(
        Debug, Add, Sub, Mul, Div, Display, PartialEq, Eq, Clone, Copy, Constructor, From, Into,
)]
#[display("({}, {})", left_num, right_num)]
pub struct MulPair {
        left_num:  u64,
        right_num: u64,
}
impl MulPair {
        pub fn self_multiply(&self) -> u64 { self.left_num * self.right_num }
}

/// Parse txt input: extracting number pairs from text.
/// No attention is paid to individual lines.
///
/// ## External:
/// [regex101](https://regex101.com)
#[instrument(skip_all, ret(level = Level::TRACE))]
pub fn parse_input_1(raw_input: &str) -> Result<Vec<MulPair>> {
        tea::trace!(?raw_input);
        let re = Regex::new(REGEX_MUL_PAIR).expect("regex compilation");
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

/// Parse txt input: extracting number pairs from text.
///
/// ## Implementation:
/// Splitting at "do()", so all substrings are start of string or `do()` prefixed.
///
/// ## External:
/// [regex101](https://regex101.com)
#[instrument(skip_all, ret(level = Level::TRACE))]
pub fn parse_input_2(raw_input: &str) -> Result<Vec<MulPair>> {
        tea::trace!(?raw_input);
        let re = Regex::new(REGEX_MUL_PAIR).expect("regex compilation");

        let do_sections = {
                let _enter = tea::debug_span!("Splitting Do() Sections").entered();
                raw_input
                        .split("do()")
                        .map(|do_prefix| do_prefix.split("don't()").next().unwrap())
        };
        tea::debug!(?do_sections);

        let mult_pairs_do: Result<Vec<_>> = {
                let _enter = tea::debug_span!("Parsing").entered();
                do_sections
                        .flat_map(|do_prefixed_str| re.captures_iter(do_prefixed_str))
                        .map(|cap| {
                                let (raw, [left_num_str, right_num_str]) = cap.extract();
                                tea::trace!(?raw, ?left_num_str, ?right_num_str);
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
        mult_pairs_do
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
        fn parse_1_example_test() -> Result<()> {
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
                assert_eq!(parse_input_1(input)?, expected);
                Ok(())
        }

        #[test]
        #[instrument]
        fn parse_2_example_test() -> Result<()> {
                tea::warn!("--------------Running test_example---------------");
                let input = indoc!("
                        mul(1,1)don't()_mul(9,9)do()+mul(2,2)(mul(3,3)undo()?mul(4,4))don't()mul(9,9)on't()_mul(9,9)do()+mul(5,5)
                        mul(11,11)
                        don't()_mul(99,99)
                        do()+mul(22,22)(mul(33,33)undo()?mul(44,44))don't()mul(99,99)
                        on't()_mul(99,99)do()+mul(55,55)
                        ");
                let expected: Vec<MulPair> = vec![
                        (1, 1),
                        (2, 2),
                        (3, 3),
                        (4, 4),
                        (5, 5),
                        (11, 11),
                        (22, 22),
                        (33, 33),
                        (44, 44),
                        (55, 55),
                ]
                .into_iter()
                .map(|(left, right)| MulPair::new(left, right))
                .collect();
                let x = parse_input_2(input);
                tea::warn!(?x);
                assert_eq!(parse_input_2(input)?, expected);
                Ok(())
        }

        #[quickcheck]
        #[instrument]
        fn qc_example_parse_1_test(inp_pairs: Vec<(u64, u64)>) -> TestResult {
                tea::debug!("--------------Running qc_example_quickcheck---------------");
                tea::debug!(?inp_pairs);
                let inp_string = clean_input_generator(inp_pairs.clone()).unwrap_or_else(|e| {
                        tea::error!(?e, "input error");
                        panic!();
                });
                tea::debug!(?inp_string);

                let parsed_and_deconstructed: Vec<(u64, u64)> = parse_input_1(&inp_string)
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

        #[quickcheck]
        #[instrument]
        fn qc_example_parse_2_test(inp_pairs: Vec<(u64, u64)>) -> TestResult {
                tea::debug!("--------------Running qc_example_quickcheck---------------");
                tea::debug!(?inp_pairs);
                let inp_string = do_dont_input_generator(inp_pairs.clone()).unwrap_or_else(|e| {
                        tea::error!(?e, "input error");
                        panic!();
                });
                tea::debug!(?inp_string);

                let parsed_and_deconstructed: Vec<(u64, u64)> = parse_input_2(&inp_string)
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
        fn do_dont_input_generator(inp_pairs: Vec<(u64, u64)>) -> Result<String> {
                let inp_string = inp_pairs
                        .iter()
                        .map(|(left, right)| {
                                format!("mul({left}, {right})don't()mul({left},{right})mul({left},{right})do()")
                        })
                        .collect::<Vec<_>>()
                        .join("\n");
                Ok(inp_string)
        }
}
