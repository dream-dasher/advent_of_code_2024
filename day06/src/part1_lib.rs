//! Library code for Part 1 of Day06 of Advent of Code 2024.
//! `bin > part1_bin.rs` will run this code along with content of `input1.txt`

use dirty_terminal::*;
use itertools::Itertools as _;
use owo_colors::OwoColorize as _;
use tracing::{Level, instrument};

use crate::{Result,
            parse::{Guard, Maze, Point2D, parse_input},
            support::error::ErrKindDay06};

#[instrument(skip_all, ret(level = Level::INFO))]
pub fn process_part1(input: &str) -> Result<u64> {
        let (maze, mb_guard) = parse_input(input)?;
        let mut guard = mb_guard.ok_or(ErrKindDay06::NoGuardFound {
                source_input: Some(input.to_string()),
        })?;
        // moving guard
        for _ in 0..10 {
                clear_screen_ansi();
                let pm = populated_maze_rep(&maze, &guard);
                println!("pm:\n{}", pm);
                dirty_pause()?;
                if !(0..maze.max_dims.x).contains(&guard.pos.x) || !(0..maze.max_dims.y).contains(&guard.pos.y) {
                        break;
                }
                guard.pos += Point2D::new(1, 1);
        }

        todo!();
}
mod dirty_terminal {
        use std::{io, io::Write as _};

        use tracing::event;

        use super::*;
        /// Clear terminal screen using ANSI escape code.
        ///
        /// Not the most robust, but decent in a pinch.
        #[instrument]
        pub fn clear_screen_ansi() {
                // There are ANSI escape codes that can be used to clear the screen!
                const ANSI_CLEAR_SCREEN: &str = "\x1B[2J\x1B[H";
                print!("{}", ANSI_CLEAR_SCREEN);
                std::io::stdout().flush().unwrap();
        }

        /// Quick and dirty pause button so I can watch as program runs.
        #[instrument]
        pub fn dirty_pause() -> Result<()> {
                println!("Press Enter to continue...");
                let mut _input = String::new();
                let read_in = io::stdin().read_line(&mut _input)?;
                event![Level::DEBUG, ?read_in];
                Ok(())
        }
}
#[instrument(skip_all)]
pub fn populated_maze_rep(maze: &Maze, guard: &Guard) -> String {
        let mut output = String::new();
        for (r, c) in (0..maze.max_dims.y).cartesian_product(0..maze.max_dims.x) {
                if c == 0 {
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
