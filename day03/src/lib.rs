//! Library code for Day03 of Advent of Code 2024.

mod part1_lib;
mod part2_lib;
mod support;

pub use part1_lib::process_part1;
pub use part2_lib::process_part2;
pub use support::{Error, Result, generate_tracing_subscriber};

pub const FINAL_INPUT: &str = include_str!("../data/final_input.txt");
pub const EXAMPLE_INPUT: &str = include_str!("../data/example_input.txt");
pub const CUSTOM_INPUT: &str = include_str!("../data/custom_input.txt");

mod parse {
        use derive_more::derive::{Add, Constructor, Display, Div, From, Into, Mul, Sub};
        use indoc::indoc;
        use regex::Regex;
        use tracing::{self as tea, Level, instrument};

        use crate::Result;

        #[derive(
                Debug, Add, Sub, Mul, Div, Display, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Constructor, From, Into,
        )]
        #[display("({}, {})", left_num, right_num)]
        pub struct MulPair {
                left_num:  u64,
                right_num: u64,
        }
        impl MulPair {
                pub fn self_multiply(&self) -> Result<u64> {
                        self.left_num.checked_mul(self.right_num).into()
                }
        }

        /// Parse txt input: extracting number pairs from text.
        /// No attention is paid to individual lines.
        ///
        /// ## External:
        /// [regex101](https://regex101.com)
        #[instrument(skip_all, ret(level = Level::TRACE))]
        pub fn parse_input(raw_input: &str) -> Result<()> {
                const REGEX_MUL_PAIR: &str = r"mul\((?<left_num>\d+),(?<right_num>\d+)\)";
                let re = Regex::new(REGEX_MUL_PAIR).expect("string should be valid regex");
                let mult_pairs: Vec<_> = {
                        let _enter = tea::debug_span!("Parsing").entered();
                        re.captures_iter(raw_input)
                                .enumerate()
                                .map(|(i2, cap)| {
                                        let (raw, [left_num, right_num]) = cap.extract();
                                        tea::info!(?raw, ?left_num, ?right_num, i2);
                                        (left_num, right_num)
                                })
                                .collect()
                };
                tea::info!(?mult_pairs);
                todo!()
        }

        /// Example use of regex crate capture for parsing.
        #[instrument(skip_all, ret(level = Level::INFO))]
        pub fn example_parse() -> Result<Vec<[String; 3]>> {
                const EXAMPLE_PATH_SPLIT_REGEX: &str = r"^(?m)^([^:]+):([0-9]+):(.+)$";
                let re = Regex::new(EXAMPLE_PATH_SPLIT_REGEX).expect("string should be valid regex");
                tea::info!(?re);

                const HAY: &str = indoc!("\
                path/to/foo:54:Blue Harvest
                path/to/bar:90:Something, Something, Something, Dark Side
                path/to/baz:3:It's a Trap!
                path/topos/babos:36:ZZzzaZZZaaaZalooong!
                ");
                tea::info!(?HAY);

                let mut out = Vec::new();
                {
                        let _enter = tea::info_span!("Parsing").entered();
                        for (i, line) in HAY.lines().enumerate() {
                                let (raw, [path, lineno, line]) = re.captures(line).unwrap().extract();
                                tea::info!(path, lineno, line, raw, i);
                                out.push([path.to_string(), lineno.to_string(), line.to_string()]);
                        }
                }
                Ok(out)
        }
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
