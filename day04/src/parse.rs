//! Raw-input parsing code for Day04 of Advent of Code 2024.

use std::ascii;

use derive_more::derive::{Constructor, Display, FromStr, Index, IntoIterator};
use indoc::indoc;
use regex::Regex;
use tracing::{self as tea, Level, instrument};

use crate::{ErrKindDay04, Result};

/// Only valid chars in the CrossWordInput.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromStr)]
pub enum CWordChar {
        X,
        M,
        A,
        S,
}
#[derive(Debug, Clone, PartialEq, Eq, Constructor, IntoIterator, Index)]
pub struct CWordLine {
        #[into_iterator(owned, ref, ref_mut)]
        chars: Vec<CWordChar>,
}
impl CWordLine {
        /// Turn a String into a CWordLine.
        fn from_str<S>(strable_inp: S) -> Result<Self>
        where
                S: AsRef<str>,
        {
                let str_input = strable_inp.as_ref();
                let mut cw_chars: Vec<CWordChar> = Vec::with_capacity(str_input.len());
                for c in str_input.chars() {
                        let cw_char = match c {
                                'X' => CWordChar::X,
                                'M' => CWordChar::M,
                                'A' => CWordChar::A,
                                'S' => CWordChar::S,
                                no_parse => {
                                        return Err(ErrKindDay04::CWCharParseError {
                                                uninterpretable_char: no_parse,
                                        })?;
                                }
                        };
                        cw_chars.push(cw_char);
                }

                Ok(CWordLine { chars: cw_chars })
        }

        /// Create a new empty CWordLine. Optionally specify the length.
        fn new_empty(length: Option<usize>) -> Self {
                match length {
                        Some(len) => CWordLine {
                                chars: Vec::with_capacity(len),
                        },
                        None => CWordLine { chars: Vec::new() },
                }
        }

        /// Push a CWordChar onto the CWordLine.
        fn push(&mut self, cw_char: CWordChar) {
                self.chars.push(cw_char);
        }
}

/// Parse txt input ...
#[instrument(skip_all, ret(level = Level::TRACE))]
pub fn parse_input(raw_input: &str) -> Result<Vec<CWordLine>> {
        raw_input
                .lines()
                .inspect(|line| tea::debug!(?line))
                .map(CWordLine::from_str)
                .inspect(|c_line| tea::debug!(?c_line))
                .collect()
}

/// Example use of regex crate capture for parsing.
///
/// ## External:
/// regex texting and expoloration site: [regex101](https://regex101.com)
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
