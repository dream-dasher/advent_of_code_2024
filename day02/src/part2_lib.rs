//! Library code for Part 2 of Day02 of Advent of Code 2024.
//! `bin > part2_bin.rs` will run this code along with content of `input2.txt`

mod parse2;
use parse2::example_parse;
use tracing::{instrument, trace};

#[expect(unused)]
use crate::{EXAMPLE_INPUT_2, FINAL_INPUT_2, support::Result};

#[instrument(skip(input))]
pub fn process_part2(input: &str) -> Result<u64> {
        trace!(%input);
        example_parse()?;
        // let input = parse1(input)?;
        todo!();
}

// #[cfg(test)]
// mod tests {
//         use indoc::indoc;

//         use super::*;

//         #[test]
//         fn test_process_example() -> Result<()> {
//                 let input = EXAMPLE_INPUT_2
//                 let expected = todo!();
//                 assert_eq!(process_part2(input)?, expected);
//                 Ok(())
//         }

//         // /// Test's expected value to be populated after solution verification.
//         // /// NOTE: `#[ignore]` is set for this test by default.
//         // #[ignore]
//         // #[test]
//         // fn test_process_problem_input() -> Result<()> {
//         //         tracing_subscriber::fmt::init();
//         //         let file_input = include_str!("../input1.txt");
//         //         let expected = todo!();
//         //         assert_eq!(process(file_input)?, expected);
//         //         Ok(())
//         // }
// }
