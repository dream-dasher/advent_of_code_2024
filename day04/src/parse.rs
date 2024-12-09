//! Raw-input parsing code for Day04 of Advent of Code 2024.

use derive_more::derive::{Constructor, FromStr, Index, IntoIterator};
pub use parse_2::parse_input_2;
use tracing::{self as tea, Level, instrument};

use crate::{ErrKindDay04, Result};

mod parse_2 {
        use std::cell::LazyCell;

        use regex::Regex;

        use super::*;

        // const REGEX_MM_EXAMPLE_FIXEDWIDTH: &str = r"M.M(.|\n){9}A(.|\n){9}S.S";
        // const REGEX_MS_EXAMPLE_FIXEDWIDTH: &str = r"M.S(.|\n){9}A(.|\n){9}M.S";
        // const REGEX_SM_EXAMPLE_FIXEDWIDTH: &str = r"S.M(.|\n){9}A(.|\n){9}S.M";
        // const REGEX_SS_EXAMPLE_FIXEDWIDTH: &str = r"S.S(.|\n){9}A(.|\n){9}M.M";
        // const REGEX_MAS_TEMPLATE: &str = r"(M.M(.|\n){{{r_minus_one}}}A(.|\n){{{r_minus_one}}}S.S|M.S(.|\n){{{r_minus_one}}}A(.|\n){{{r_minus_one}}}M.S|S.M(.|\n){{{r_minus_one}}}A(.|\n){{{r_minus_one}}}S.M|S.S(.|\n){{{r_minus_one}}}A(.|\n){{{r_minus_one}}}M.M)";

        /// Parse txt using simple regex.
        #[instrument(skip_all, ret(level = Level::DEBUG))]
        pub fn parse_input_2(raw_input: &str, row_length: usize, whoops: Option<usize>) -> Result<u64> {
                let whoops = whoops.unwrap_or(3_000_000_000) - 1;
                if whoops == 0 {
                        return Ok(0);
                }
                let regex_mas_sized = format!(
                        r"(M.M(.|\n){{{r_minus_one}}}A(.|\n){{{r_minus_one}}}S.S|M.S(.|\n){{{r_minus_one}}}A(.|\n){{{r_minus_one}}}M.S|S.M(.|\n){{{r_minus_one}}}A(.|\n){{{r_minus_one}}}S.M|S.S(.|\n){{{r_minus_one}}}A(.|\n){{{r_minus_one}}}M.M)",
                        r_minus_one = row_length - 1
                );
                let re = LazyCell::new(|| Regex::new(&regex_mas_sized).unwrap());
                tea::debug!(?raw_input);

                let Some(mat) = re.find(raw_input) else {
                        return Ok(0);
                };

                let mut count = 1;
                let start = mat.start();
                tea::info!(start);
                count += parse_input_2(&raw_input[start + 1..], row_length, Some(whoops))?;
                tea::info!(?mat);
                Ok(count)
        }
}

/// Parse txt input ...
#[instrument(skip_all, ret(level = Level::TRACE))]
pub fn parse_input_1(raw_input: &str) -> Result<CWordPuzzle> {
        CWordPuzzle::from_str(raw_input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CWordPuzzle {
        pub horizontal_view: Vec<CWordLine>,
        vertical_view:       Vec<CWordLine>,
        bltr_diagonal_view:  Vec<CWordLine>,
        brtl_diagonal_view:  Vec<CWordLine>,
}
impl CWordPuzzle {
        /// Parse a string into a CWordPuzzle.
        #[instrument(skip_all, ret(level = Level::TRACE))]
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
                {
                        for col in 0..num_cols {
                                let mut vview_line = CWordLine::new_empty(Some(num_rows));
                                #[expect(clippy::needless_range_loop)]
                                for row in 0..num_rows {
                                        vview_line.push(horizontal_view[row][col]);
                                }
                                vertical_view.push(vview_line);
                        }
                }

                // walk bottom-left perimeter
                // 00       03
                // 10
                // 20
                // 30
                // 40 41 42 43
                let mut bltr_diagonal_view: Vec<CWordLine> = Vec::new();
                {
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
                                bltr_diagonal_view.push(dview_line);
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
                                bltr_diagonal_view.push(dview_line);
                        }
                }

                // walk bottom-right perimeter
                // 00       03
                //          13
                //          23
                //          33
                // 40 41 42 43
                let mut brtl_diagonal_view: Vec<CWordLine> = Vec::new();
                {
                        // walk left rows
                        for row_head in 0..num_rows {
                                let (base_row, base_col) = (row_head, num_cols - 1);
                                let diag_len = (row_head + 1).min(num_cols);
                                let mut dview_line = CWordLine::new_empty(Some(diag_len));
                                // 03
                                // 13 02
                                // 23 12 01
                                // 33 22 11 00
                                // 43 32 21 10
                                for offset in 0..diag_len {
                                        dview_line.push(horizontal_view[base_row - offset][base_col - offset]);
                                }
                                brtl_diagonal_view.push(dview_line);
                        }
                        // walk bottom columns (excluding the first, which was cisted)
                        for col_end in 1..num_cols {
                                let (base_row, base_col) = (num_rows - 1, num_cols - 1 - col_end);
                                let diag_len = (num_cols - col_end).min(num_rows);
                                let mut dview_line = CWordLine::new_empty(Some(diag_len));
                                // 42 31 22
                                // 41 30
                                // 40
                                for offset in 0..diag_len {
                                        dview_line.push(horizontal_view[base_row - offset][base_col - offset]);
                                }
                                brtl_diagonal_view.push(dview_line);
                        }
                }
                Ok(CWordPuzzle {
                        horizontal_view,
                        vertical_view,
                        bltr_diagonal_view,
                        brtl_diagonal_view,
                })
        }

        /// Provides a reference to the horizontal view
        #[instrument(level = Level::TRACE, skip_all)]
        pub fn get_horizontal_view(&self) -> &Vec<CWordLine> {
                &self.horizontal_view
        }

        /// Provides a reference to the vertical view
        #[instrument(level = Level::TRACE, skip_all)]
        pub fn get_vertical_view(&self) -> &Vec<CWordLine> {
                &self.vertical_view
        }

        /// Provides a reference to the diagonal view (bottom-left to top-right)
        #[instrument(level = Level::TRACE, skip_all)]
        pub fn get_diagonal_view_bltr(&self) -> &Vec<CWordLine> {
                &self.bltr_diagonal_view
        }

        /// Provides a reference to the diagonal view (bottom-right to top-left)
        #[instrument(level = Level::TRACE, skip_all)]
        pub fn get_diagonal_view_brtl(&self) -> &Vec<CWordLine> {
                &self.brtl_diagonal_view
        }

        /// Provides a clone of the puzzle in canonical (row, column) form.
        #[instrument(level = Level::TRACE, skip_all)]
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
        #[instrument(level=Level::TRACE, skip_all, ret(level = Level::TRACE))]
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
                                        return Err(ErrKindDay04::CWCharParse {
                                                uninterpretable_char: no_parse,
                                        })?;
                                }
                        };
                        cw_chars.push(cw_char);
                }

                Ok(CWordLine { chars: cw_chars })
        }

        /// Create a new empty CWordLine. Optionally specify the length.
        #[instrument(level = Level::TRACE)]
        fn new_empty(length: Option<usize>) -> Self {
                match length {
                        Some(len) => CWordLine {
                                chars: Vec::with_capacity(len),
                        },
                        None => CWordLine { chars: Vec::new() },
                }
        }

        /// Push a CWordChar onto the CWordLine.
        #[instrument(level = Level::TRACE, ret(level = Level::TRACE))]
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
