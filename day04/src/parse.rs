//! Raw-input parsing code for Day04 of Advent of Code 2024.

use std::ascii;

use derive_more::derive::{Constructor, Display, FromStr, Index, IntoIterator};
use indoc::indoc;
use regex::Regex;
use tracing::{self as tea, Level, instrument};

use crate::{ErrKindDay04, Result};

/// Parse txt input ...
#[instrument(skip_all, ret(level = Level::TRACE))]
pub fn parse_input(raw_input: &str) -> Result<CWordPuzzle> {
        CWordPuzzle::from_str(raw_input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CWordPuzzle {
        pub horizontal_view: Vec<CWordLine>,
        vertical_view:       Vec<CWordLine>,
        diagonal_view:       Vec<CWordLine>,
}
impl CWordPuzzle {
        /// Parse a string into a CWordPuzzle.
        pub fn from_str<S>(strable_inp: S) -> Result<Self>
        where
                S: AsRef<str>,
        {
                let str_input = strable_inp.as_ref();
                // matches structures lf `.lines()`
                let horizontal_view: Vec<CWordLine> =
                        str_input.lines().map(CWordLine::from_str).collect::<Result<_>>()?;

                // let num_chars = str_input.chars().count(); // len would work for ascii assumption
                let num_chars = str_input.len(); // assuming ascii chars
                let num_rows = horizontal_view.len();
                let num_cols = (num_chars / num_rows) - 1; // -1 for newline
                tea::info!(num_chars, num_rows, num_cols);

                // transpose of horizontal
                let mut vertical_view: Vec<CWordLine> = Vec::new();
                for col in 0..num_cols {
                        let mut vview_line = CWordLine::new_empty(Some(num_rows));
                        #[expect(clippy::needless_range_loop)]
                        for row in 0..num_rows {
                                vview_line.push(horizontal_view[row][col]);
                        }
                        vertical_view.push(vview_line);
                }

                // walk bottom-left perimeter
                // 00       03
                // 10
                // 20
                // 30
                // 40 41 42 43
                let mut diagonal_view: Vec<CWordLine> = Vec::new();

                // walk left rows
                for row_head in 0..num_rows {
                        let (base_row, base_col) = (row_head, 0);
                        let diag_len = (row_head + 1).min(num_cols);
                        let mut dview_line = CWordLine::new_empty(Some(diag_len));
                        // 00
                        // 10 01
                        // 20 11 02
                        // 30 21 12 03
                        // 40 31 22 13
                        for offset in 0..diag_len {
                                dview_line.push(horizontal_view[base_row - offset][base_col + offset]);
                        }
                        diagonal_view.push(dview_line);
                }
                // walk bottom columns (excluding the first, which was cisted)
                for col_end in 1..num_cols {
                        let (base_row, base_col) = (num_rows - 1, col_end);
                        let diag_len = (num_cols - col_end).min(num_rows);
                        let mut dview_line = CWordLine::new_empty(Some(diag_len));
                        // 41 32 23
                        // 42 33
                        // 43
                        for offset in 0..diag_len {
                                dview_line.push(horizontal_view[base_row - offset][base_col + offset]);
                        }
                        diagonal_view.push(dview_line);
                }
                Ok(CWordPuzzle {
                        horizontal_view,
                        vertical_view,
                        diagonal_view,
                })
        }

        /// Provides a copy of the puzzle in canonical (row, column) form.
        pub fn canonical_view(&self) -> Vec<CWordLine> {
                self.horizontal_view.clone()
        }
}

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

// #[cfg(test)]
// mod tests {
//         use indoc::indoc;
//         use quickcheck::TestResult;
//         use quickcheck_macros::quickcheck;
//         use rand::Rng;
//         use test_log::test;
//         use tracing::{self as tea, instrument};

//         use super::*;

//         #[instrument]
//         fn example_input_generator(sum: u16, step_range_inclusive: (u8, u8)) -> Option<Vec<i64>> {
//                 let (low_step, high_step) = step_range_inclusive;
//                 let low_step = low_step as i64;
//                 let high_step = high_step as i64;
//                 let mut sum = sum as i64;

//                 if low_step >= high_step {
//                         tea::trace!(?low_step, ?high_step);
//                         return None;
//                 }
//                 let mut rng = rand::thread_rng();
//                 let mut out = Vec::new();
//                 while sum > 0 {
//                         let step = rng.gen_range(low_step..=high_step).min(sum);
//                         out.push(step);
//                         sum -= step;
//                         tea::debug!(?step, ?sum);
//                 }
//                 Some(out)
//         }

//         #[quickcheck]
//         #[instrument]
//         fn qc_example_quickcheck(sum: u16, step_range_inclusive: (u8, u8)) -> TestResult {
//                 let Some(vector) = example_input_generator(sum, step_range_inclusive) else {
//                         return TestResult::discard();
//                 };
//                 let vector_sum: i64 = vector.iter().sum();
//                 TestResult::from_bool(sum as i64 == vector_sum)
//         }
// }
