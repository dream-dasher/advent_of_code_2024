//! Library code for Part 2 of Day06 of Advent of Code 2024.
//! `bin > part2_bin.rs` will run this code along with content of `input2.txt`

use std::collections::HashSet;

use derive_more::derive::Index;
use itertools::Itertools as _;
use owo_colors::OwoColorize as _;
use tracing::{Level, instrument};

use crate::{Result,
            parse::{Direction, Guard, Maze, PositionState, parse_input},
            support::error::ErrKindDay06};

#[instrument(skip_all, ret(level = Level::INFO))]
pub fn process_part2(input: &str) -> Result<usize> {
        let (maze, mb_guard) = parse_input(input)?;
        let guard_initial = mb_guard.ok_or(ErrKindDay06::NoGuardFound {
                source_input: Some(input.to_string()),
        })?;
        let mut pop_maze = PopulatedMazeWHSet::new(maze, guard_initial)?;
        for _ in 0.. {
                match pop_maze.update() {
                        Ok(_) => {
                                continue;
                        }
                        Err(UpdateError::LoopDetected) => {
                                tracing::event![Level::INFO, "Loop detected."];
                                break;
                        }
                        Err(UpdateError::GuardOutOfBounds) => {
                                tracing::event![Level::INFO, "Guard Exited Maze."];
                                break;
                        }
                        Err(UpdateError::NoMoveAvailable) => {
                                tracing::event![Level::WARN, "Guard trapped on a single tile."];
                                break;
                        }
                }
        }

        let distinct_positions = pop_maze.guard_states.iter().map(|guard| guard.pos).unique().count();
        Ok(distinct_positions)
}

#[derive(Index, Debug, Clone, PartialEq, Eq)]
pub struct PopulatedMazeWHSet {
        #[index]
        pub maze:                Maze,
        pub guard_states:        HashSet<Guard>,
        pub guard_current_state: Guard,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, derive_more::Error, derive_more::Display)]
pub enum UpdateError {
        GuardOutOfBounds,
        LoopDetected,
        NoMoveAvailable,
}
impl PopulatedMazeWHSet {
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

                let mut guard_states = HashSet::new();
                guard_states.insert(guard);

                Ok(Self {
                        maze,
                        guard_states,
                        guard_current_state: guard,
                })
        }

        #[instrument()]
        pub fn update(&mut self) -> core::result::Result<Guard, UpdateError> {
                // circularly ordered by right-turns
                const DIR_ARRAY: [Direction; 4] = [Direction::Up, Direction::Right, Direction::Down, Direction::Left];
                let bounds = self.maze.max_dims;
                let guard_now = self.guard_current_state;
                let cycle_start_idx: usize = match guard_now.dir {
                        Direction::Up => 0,
                        Direction::Right => 1,
                        Direction::Down => 2,
                        Direction::Left => 3,
                };

                for i in 0..4 {
                        let dir = DIR_ARRAY[(cycle_start_idx + i).rem_euclid(4)];
                        let Some(pos_to_check) = guard_now.pos.try_move(dir, Some(bounds)) else {
                                // 'None' Means guard has escaped maze
                                return Err(UpdateError::GuardOutOfBounds);
                        };

                        if self.maze
                                .get(pos_to_check)
                                .expect("check pos checked for being in bounds")
                                == PositionState::Empty
                        {
                                let new_guard = Guard::new(pos_to_check, dir);
                                if self.guard_states.contains(&new_guard) {
                                        return Err(UpdateError::LoopDetected);
                                }
                                self.guard_states.insert(new_guard);
                                self.guard_current_state = new_guard;
                                return Ok(new_guard);
                        }
                }
                // This should only occur if all four adjacent tiles are obstacles.  Unexpected, hence warn flag.
                tracing::event![Level::WARN, ?guard_now, "Guard trapped on single tile."];
                Err(UpdateError::NoMoveAvailable)
        }
}
impl std::fmt::Display for PopulatedMazeWHSet {
        #[instrument(skip_all)]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let maze = &self.maze;
                let guard = &self.guard_current_state;
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
//                 assert_eq!(process_part2(input)?, expected);
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
//         //         assert_eq!(process_part2(input)?, expected);
//         //         Ok(())
//         // }
// }
