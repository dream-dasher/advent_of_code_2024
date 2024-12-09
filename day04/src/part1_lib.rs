//! Library code for Part 1 of Day04 of Advent of Code 2024.
//! `bin > part1_bin.rs` will run this code along with content of `input1.txt`

use derive_more::derive::Display;
use tracing::{self as tea, Level, instrument};

use crate::{Result,
            parse::{CWordChar, CWordLine, CWordPuzzle, parse_input}};

#[instrument(skip_all, ret(level = Level::DEBUG))]
pub fn process_part1(input: &str) -> Result<u64> {
        tea::trace!(%input);
        let puzzle = parse_input(input)?;
        let (h, v, dbl, dbr) = puzzle.count_rotations();
        Ok(h + v + dbl + dbr)
}

impl CWordPuzzle {
        fn count_rotations(&self) -> (u64, u64, u64, u64) {
                let hor_count = SearchStateMachine::new().count_xmas(self.get_horizontal_view());
                let vert_count = SearchStateMachine::new().count_xmas(self.get_vertical_view());
                let diag_bltr_count = SearchStateMachine::new().count_xmas(self.get_diagonal_view_bltr());
                let diag_brtl_count = SearchStateMachine::new().count_xmas(self.get_diagonal_view_brtl());
                tea::info!(hor_count, vert_count, diag_bltr_count, diag_brtl_count);
                (hor_count, vert_count, diag_bltr_count, diag_brtl_count)
        }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub struct SearchStateMachine {
        state: SearchState,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
#[expect(clippy::upper_case_acronyms)]
enum SearchState {
        Null,
        X,
        XM,
        XMA,
        FoundXMAS,
        S,
        SA,
        SAM,
        FoundSAMX,
}
impl SearchState {
        /// Returns true if the state is in either `FoundXMAS` or `FoundSAMX`.
        #[instrument(ret(level = Level::DEBUG))]
        fn is_found(&self) -> bool {
                matches!(self, SearchState::FoundXMAS | SearchState::FoundSAMX)
        }
}
impl SearchStateMachine {
        /// Counts the occurrences of `XMAS` and `SAMX` on each line in a vector of `CWordLine`s.
        #[instrument(skip_all, ret(level = Level::DEBUG))]
        pub fn count_xmas(&self, cw_lines: &[CWordLine]) -> u64 {
                let mut total_finds = 0;
                for (i, line) in cw_lines.iter().enumerate() {
                        let _enter = tea::debug_span!("Processing, line", ?i, ?total_finds).entered();
                        let mut search_state_machine = SearchStateMachine::new();
                        for cw_char in line.into_iter() {
                                let new_state = search_state_machine.next(cw_char);
                                if new_state.is_found() {
                                        total_finds += 1;
                                }
                                tea::debug!(?cw_char, ?new_state, ?i);
                        }
                }
                total_finds
        }

        /// Start a new `XMAS|SMAX` search state machine from a null value.
        #[instrument(ret(level = Level::DEBUG))]
        fn new() -> Self {
                SearchStateMachine {
                        state: SearchState::Null,
                }
        }

        /// Advance the SearchStaateMachine by one char. Return the new state of the machine.
        ///
        /// ## Note
        /// `Found` & `Null` are equivalent for traversal logic.
        /// It's up to the caller to operate based on the distinction.
        #[instrument(ret(level = Level::DEBUG))]
        fn next(&mut self, cw_char: &CWordChar) -> SearchState {
                let new_state = match (&cw_char, self.state) {
                        (CWordChar::X, SearchState::Null | SearchState::FoundXMAS | SearchState::FoundSAMX) => {
                                SearchState::X
                        }
                        (CWordChar::M, SearchState::X | SearchState::FoundSAMX) => SearchState::XM,
                        (CWordChar::A, SearchState::XM) => SearchState::XMA,
                        (CWordChar::S, SearchState::XMA) => SearchState::FoundXMAS,
                        //
                        (CWordChar::S, SearchState::Null | SearchState::FoundSAMX | SearchState::FoundXMAS) => {
                                SearchState::S
                        }
                        (CWordChar::A, SearchState::S | SearchState::FoundXMAS) => SearchState::SA,
                        (CWordChar::M, SearchState::SA) => SearchState::SAM,
                        (CWordChar::X, SearchState::SAM) => SearchState::FoundSAMX,
                        //
                        _ => SearchState::Null,
                };

                self.state = new_state;
                new_state
        }

        /// Show the state that the SearchStateMachine would enter
        ///
        /// ## Note
        /// `Found` & `Null` are equivalent for traversal logic.
        /// It's up to the caller to operate based on the distinction.
        #[instrument(ret(level = Level::DEBUG))]
        fn preview_next(&self, cw_char: &CWordChar) -> SearchState {
                match (&cw_char, self.state) {
                        (CWordChar::X, SearchState::Null | SearchState::FoundXMAS) => SearchState::X,
                        (CWordChar::M, SearchState::X | SearchState::FoundSAMX) => SearchState::XM,
                        (CWordChar::A, SearchState::XM) => SearchState::XMA,
                        (CWordChar::S, SearchState::XMA) => SearchState::FoundXMAS,
                        //
                        (CWordChar::S, SearchState::Null | SearchState::FoundSAMX) => SearchState::S,
                        (CWordChar::A, SearchState::S | SearchState::FoundXMAS) => SearchState::SA,
                        (CWordChar::M, SearchState::SA) => SearchState::SAM,
                        (CWordChar::X, SearchState::SAM) => SearchState::FoundSAMX,
                        //
                        _ => SearchState::Null,
                }
        }

        /// Consume a SearchStateMachine and CWordChar and produce a new, advanced, SearchStateMachine.
        ///
        /// ## Note
        /// `Found` & `Null` are equivalent for traversal logic.
        /// It's up to the caller to operate based on the distinction.
        #[instrument(ret(level = Level::DEBUG))]
        fn evolve(mut self, cw_char: &CWordChar) -> SearchStateMachine {
                self.state = match (&cw_char, self.state) {
                        (CWordChar::X, SearchState::Null | SearchState::FoundXMAS) => SearchState::X,
                        (CWordChar::M, SearchState::X | SearchState::FoundSAMX) => SearchState::XM,
                        (CWordChar::A, SearchState::XM) => SearchState::XMA,
                        (CWordChar::S, SearchState::XMA) => SearchState::FoundXMAS,
                        //
                        (CWordChar::S, SearchState::Null | SearchState::FoundSAMX) => SearchState::S,
                        (CWordChar::A, SearchState::S | SearchState::FoundXMAS) => SearchState::SA,
                        (CWordChar::M, SearchState::SA) => SearchState::SAM,
                        (CWordChar::X, SearchState::SAM) => SearchState::FoundSAMX,
                        //
                        _ => SearchState::Null,
                };
                self
        }
}
#[cfg(test)]
mod tests {
        use indoc::indoc;
        use test_log::test;
        #[expect(unused)]
        use tracing::{self as tea, instrument};

        use super::*;
        #[expect(unused)]
        use crate::{EXAMPLE_INPUT, FINAL_INPUT};

        #[test]
        #[instrument]
        fn horizontal_count_example_test() -> Result<()> {
                let input = indoc!("
                        SAMXMASS
                        XMASAMXX
                        XMASSAMX
                        XSSXXXXX
                        XXXXXMAS
                        ");
                let expected_horizontal_count = 7;
                let horizontal_view = parse_input(input)?.canonical_view();
                let horizontal_count = SearchStateMachine::new().count_xmas(&horizontal_view);

                assert_eq!(horizontal_count, expected_horizontal_count);
                Ok(())
        }

        #[test]
        #[instrument]
        fn part1_example_input_test() -> Result<()> {
                let input = EXAMPLE_INPUT;
                let hor_expected = 5;
                let vert_expected = 3;
                let diag_bltr_expected = 5;
                let diag_brtl_expected = 0;
                let (h, v, dbl, dbr) = parse_input(input)?.count_rotations();

                assert_eq!(
                        (hor_expected, vert_expected, diag_bltr_expected, diag_brtl_expected),
                        (h, v, dbl, dbr)
                );
                let expected = 18;
                assert_eq!(process_part1(input)?, expected);
                Ok(())
        }

        // #[test]
        // #[instrument]
        // fn part1_final_input_test() -> Result<()> {
        //         let input = FINAL_INPUT;
        //         let expected = 184_511_516;
        //         assert_eq!(process_part1(input)?, expected);
        //         Ok(())
        // }
}
