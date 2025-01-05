//! Library code for Part 1 of Day07 of Advent of Code 2024.

use tracing::{Level, instrument};

#[expect(unused)]
use crate::{Result, parse::parse_input};

#[instrument(skip_all, ret(level = Level::DEBUG))]
pub fn process_part1(input: &str) -> Result<u64> {
        tracing::event!(Level::TRACE, %input);

        for line in input.lines() {
                // <solution>:
                // let before_colon_trimmed = line.split(':').next()?.trim();
                // Split on whitespace
                // todo: sep first num from rest
                // let numbers: Result<Vec<u128>> = line.split_whitespace().map(|s| s.parse::<u128>()).collect().into();
                // let _parsed_input = parse_input(input)?;
                todo!();
        }
        // : <n1> <n2> <n3> <n4> ...
        // let numbers: Result<Vec<u128>> = line.split_whitespace().map(|s| s.parse::<u128>()).collect();
        // let _parsed_input = parse_input(input)?;
        todo!();
}

struct equation {
        solution:   u64,
        components: Vec<u128>,
}
#[cfg(test)]
mod tests {
        // use indoc::indoc;
        // use quickcheck::TestResult;
        // use quickcheck_macros::quickcheck;
        // use rand::Rng;
        use test_log::test;
        use tracing::instrument;

        use super::*;
        use crate::EXAMPLE_INPUT;
        //         use crate::FINAL_INPUT;

        #[test]
        #[ignore]
        #[instrument]
        fn test_process_example() -> Result<()> {
                let input = EXAMPLE_INPUT;
                let expected = 3_749;
                assert_eq!(process_part1(input)?, expected);
                Ok(())
        }

        // /// Test's expected value to be populated after solution verification.
        // #[test]
        // #[instrument]
        // fn test_process_problem_input() -> Result<()> {
        //         let input = FINAL_INPUT;
        //         let expected = todo!();
        //         assert_eq!(process_part1(input)?, expected);
        //         Ok(())
        // }
}
