//! Library code for Part 2 of Day01 of Advent of Code 2024.
//! `bin > part2_bin.rs` will run this code along with content of `input2.txt`

mod parse2;
use std::collections::HashMap;

use parse2::parse_input2;
use tracing::{self as tea, instrument};

#[expect(unused)]
use crate::{EXAMPLE_INPUT_2, FINAL_INPUT_2, support::Result};

#[instrument(skip(input))]
pub fn process_part2(input: &str) -> Result<u64> {
        tea::trace!(%input);
        let (left, right) = parse_input2(input)?;
        tea::trace!(?left, ?right);
        let left_fcount: HashMap<u64, u64> = freq_count(left);
        let right_fcount: HashMap<u64, u64> = freq_count(right);
        tea::trace!(?left_fcount, ?right_fcount);
        let mut total = 0;
        for (left_k, left_v) in left_fcount.iter() {
                if let Some(right_v) = right_fcount.get(left_k) {
                        total += left_k * left_v * right_v;
                }
        }
        tea::info!(?total);

        Ok(total)
}

fn freq_count(input: Vec<u64>) -> HashMap<u64, u64> {
        let mut freq_count: HashMap<u64, u64> = HashMap::with_capacity(input.len());
        for i in input {
                let count = freq_count.entry(i).or_insert(0);
                *count += 1;
        }
        freq_count
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
