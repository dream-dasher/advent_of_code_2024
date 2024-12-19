//! Library code for Part 1 of Day06 of Advent of Code 2024.
//! `bin > part1_bin.rs` will run this code along with content of `input1.txt`

use itertools::Itertools as _;
use owo_colors::OwoColorize as _;
use tracing::{self as tea, Level, instrument};

use crate::{Result,
            parse::{Guard, Maze, parse_input},
            support::error::ErrKindDay06};

#[instrument(skip_all, ret(level = Level::INFO))]
pub fn process_part1(input: &str) -> Result<u64> {
        let (maze, mb_guard) = parse_input(input)?;
        let guard = mb_guard.ok_or(ErrKindDay06::NoGuardFound {
                source_input: Some(input.to_string()),
        })?;
        // tea::trace![%maze, %guard];
        let pm = populated_maze_rep(&maze, &guard);
        println!("pm:\n{}", pm);

        todo!();
}
#[instrument(skip_all)]
pub fn populated_maze_rep(maze: &Maze, guard: &Guard) -> String {
        let mut output = String::new();
        for (r, c) in (0..maze.max_dims.y).cartesian_product(0..maze.max_dims.x) {
                if c == 0 && r != 0 {
                        output.push('\n');
                }

                if r == guard.pos.y && c == guard.pos.x {
                        // Use guard's direction character in bright cyan
                        output.push_str(&format!("{}", guard.dir.to_string().bright_cyan()));
                } else {
                        output.push_str(&maze.positions[r * maze.max_dims.x + c].to_string());
                }
        }
        output
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
//         use crate::{EXAMPLE_INPUT, FINAL_INPUT};

//         #[test]
//         #[instrument]
//         fn test_process_example() -> Result<()> {
//                 let input = EXAMPLE_INPUT;
//                 let expected = todo!();
//                 assert_eq!(process_part1(input)?, expected);
//                 Ok(())
//         }

//         // /// Test's expected value to be populated after solution verification.
//         // /// NOTE: `#[ignore]` is set for this test by default.
//         // #[ignore]
//         // #[test]
//         // #[instrument]
//         // fn test_process_problem_input() -> Result<()> {
//         //         let input = FINAL_INPUT;
//         //         let expected = todo!();
//         //         assert_eq!(process_part1(input)?, expected);
//         //         Ok(())
//         // }
// }
