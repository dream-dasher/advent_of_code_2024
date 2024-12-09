//! Library code for Part 1 of Day04 of Advent of Code 2024.
//! `bin > part1_bin.rs` will run this code along with content of `input1.txt`

use derive_more::derive::Display;
use tracing::{self as tea, Level, instrument};

use crate::{Result,
            parse::{CWordChar, CWordLine, parse_input}};

#[instrument(skip_all, ret(level = Level::DEBUG))]
pub fn process_part1(input: &str) -> Result<u64> {
        tea::trace!(%input);
        let puzzle = parse_input(input)?;
        let simplistic_input = puzzle.horizontal_view;
        let mut hor_count = 0;

        for (i, line) in simplistic_input.iter().enumerate() {
                let _enter = tea::debug_span!("Processing, line", ?i, ?hor_count).entered();
                let mut search_state_machine = SearchStateMachine::new();
                for cw_char in line.into_iter() {
                        let new_state = search_state_machine.next(cw_char);
                        if new_state == SearchState::FoundXMAS || new_state == SearchState::FoundSAMX {
                                hor_count += 1;
                        }
                        tea::debug!(?cw_char, ?new_state, ?i);
                }
        }
        let hor_count2 = SearchStateMachine::new().count_xmas(simplistic_input);
        println!("{}", hor_count);
        println!("{}", hor_count2);
        todo!();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub struct SearchStateMachine {
        state: SearchState,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
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
impl SearchStateMachine {
        /// Counts the occurrences of `XMAS` and `SAMX` on each line in a vector of `CWordLine`s.
        pub fn count_xmas(&self, cw_lines: Vec<CWordLine>) -> u64 {
                let mut total_finds = 0;
                for (i, line) in cw_lines.iter().enumerate() {
                        let _enter = tea::debug_span!("Processing, line", ?i, ?total_finds).entered();
                        let mut search_state_machine = SearchStateMachine::new();
                        for cw_char in line.into_iter() {
                                let new_state = search_state_machine.next(cw_char);
                                if new_state == SearchState::FoundXMAS || new_state == SearchState::FoundSAMX {
                                        total_finds += 1;
                                }
                                tea::debug!(?cw_char, ?new_state, ?i);
                        }
                }
                total_finds
        }

        /// Start a new `XMAS|SMAX` search state machine from a null value.
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
        fn preview_next(&self, cw_char: &CWordChar) -> SearchState {
                let new_state = match (&cw_char, self.state) {
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

                new_state
        }

        /// Consume a SearchStateMachine and CWordChar and produce a new, advanced, SearchStateMachine.
        ///
        /// ## Note
        /// `Found` & `Null` are equivalent for traversal logic.
        /// It's up to the caller to operate based on the distinction.
        fn evolve(mut self, cw_char: &CWordChar) -> SearchStateMachine {
                let new_state = match (&cw_char, self.state) {
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

                self.state = new_state;
                self
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
                let horizontal_count = SearchStateMachine::new().count_xmas(horizontal_view);

                assert_eq!(horizontal_count, expected_horizontal_count);
                Ok(())
        }

        // /// Test's expected value to be populated after solution verification.
        // /// NOTE: `#[ignore]` is set for this test by default.
        // #[ignore]
        // #[test]
        // fn test_process_problem_input() -> Result<()> {
        //         tracing_subscriber::fmt::init();
        //         let input = FINAL_INPUT;
        //         let expected = todo!();
        //         assert_eq!(process_part1(input)?, expected);
        //         Ok(())
        // }
}
