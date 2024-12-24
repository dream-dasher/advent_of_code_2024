//! Library code for Part 1 of Day06 of Advent of Code 2024.
//! `bin > part1_bin.rs` will run this code along with content of `input1.txt`

use derive_more::derive::Index;
use itertools::Itertools as _;
use owo_colors::OwoColorize as _;
use tracing::{Level, instrument};

use crate::{Result,
            parse::{Direction, Guard, Maze, Point2D, PositionState, parse_input},
            support::{dirty_terminal::*, error::ErrKindDay06}};

#[instrument(skip_all, ret(level = Level::INFO))]
pub fn process_part1(input: &str) -> Result<u64> {
        let (maze, mb_guard) = parse_input(input)?;
        let guard = mb_guard.ok_or(ErrKindDay06::NoGuardFound {
                source_input: Some(input.to_string()),
        })?;
        let mut pop_maze = PopulatedMaze::new(maze, guard)?;
        // moving guard

        todo!();
}

#[derive(Index, Debug, Clone, PartialEq, Eq)]
pub struct PopulatedMaze {
        #[index]
        pub maze:            Maze,
        pub guard_time_path: Vec<Guard>,
}
impl PopulatedMaze {
        /// Create a new PopulatedMaze instance. Checking for guard being within bounds and placed in an empty space.
        pub fn new(maze: Maze, guard: Guard) -> Result<Self> {
                if !(0..maze.max_dims.x).contains(&guard.pos.x) || !(0..maze.max_dims.y).contains(&guard.pos.y) {
                        Err(ErrKindDay06::GuardOutOfBounds {
                                guard_pos: guard.pos.into(),
                                maze_max:  maze.max_dims.into(),
                        })?
                }
                if maze.get(guard.pos)
                        .expect("maze filled, and guard position overlap checked")
                        != PositionState::Empty
                {
                        Err(ErrKindDay06::GuardOnNonEmptySpace {
                                guard_pos:      guard.pos,
                                position_state: maze.get(guard.pos).expect("some maze position state checked already"),
                        })?
                }
                Ok(Self {
                        maze,
                        guard_time_path: vec![guard],
                })
        }

        pub fn update(&mut self) -> Option<Guard> {
                let guard = self
                        .guard_time_path
                        .last()
                        .expect("guard_time_path should not be empty");
                let dir = guard.dir;
                let x = guard.pos.x;
                let y = guard.pos.y;

                todo!()
        }
}
impl std::fmt::Display for PopulatedMaze {
        #[instrument(skip_all)]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let maze = &self.maze;
                let guard = &self
                        .guard_time_path
                        .last()
                        .expect("guard_time_path should not be empty");
                for (r, c) in (0..maze.max_dims.y).cartesian_product(0..maze.max_dims.x) {
                        if c == 0 {
                                writeln!(f)?;
                        }

                        if r == guard.pos.y && c == guard.pos.x {
                                write!(f, "{}", guard.dir.to_string().on_red().green().bold())?;
                        } else {
                                write!(f, "{}", &maze.positions[r * maze.max_dims.x + c].to_string())?;
                        }
                }
                Ok(())
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
