//! Library code for Part 1 of Day06 of Advent of Code 2024.

use derive_more::derive::Index;
use itertools::Itertools as _;
use owo_colors::OwoColorize as _;
use tracing::{Level, instrument};

use crate::{Result,
            parse::{Direction, Guard, Maze, PositionState, parse_input},
            support::error::ErrKindDay06};

#[cfg(not(feature = "loop-checking-hashset_p1"))]
#[instrument(skip_all, ret(level = Level::INFO))]
pub fn process_part1(input: &str) -> Result<usize> {
        #[cfg(not(feature = "loop-checking-vec_p1"))]
        {
                tracing::event!(
                        Level::WARN,
                        "Loop checking disabled.  Loops total loops (those including initial position) are checked.  Use `--features ...` to enable general loop checking (may be multiple methods)."
                );
        }
        let (maze, mb_guard) = parse_input(input)?;
        let guard_initial = mb_guard.ok_or(ErrKindDay06::NoGuardFound { source_input: Some(input.to_string()) })?;
        let mut pop_maze = PopulatedMaze::new(maze, guard_initial)?;
        for _ in 0.. {
                let opt_guard_update = pop_maze.update();
                #[cfg(feature = "manual-walkthrough_p1")]
                {
                        use crate::support::dirty_terminal;
                        dirty_terminal::dirty_pause()?;
                        dirty_terminal::clear_screen_ansi();
                        println!("update: {:?}", opt_guard_update);
                        println!("{}", pop_maze);
                        println!("guard_time_path: {:?}", pop_maze.guard_time_path);
                }

                if let Some(guard_update) = opt_guard_update {
                        #[cfg(not(feature = "loop-checking-vec_p1"))]
                        if guard_update != guard_initial {
                                continue;
                        }
                        #[cfg(feature = "loop-checking-vec_p1")]
                        {
                                // skip last element, as what we're checking was already pushed on vec
                                if !pop_maze.guard_time_path.iter().rev().skip(1).contains(&guard_update) {
                                        continue;
                                }
                        }
                }
                // repeat or None
                break;
        }

        let distinct_positions = pop_maze.guard_time_path.iter().map(|guard| guard.pos).unique().count();
        Ok(distinct_positions)
}

/// Slightly slower (about 33% increase in runtime for full input) than non-exhaustive loop checked version.
/// But much quicker than exhaustive loop checking over vector save of guard states (which is about 100% increase).
/// (This is notable, as the max length of the vector is under 5_000 items.)
#[cfg(feature = "loop-checking-hashset_p1")]
#[instrument(skip_all, ret(level = Level::INFO))]
pub fn process_part1(input: &str) -> Result<usize> {
        use crate::{PopulatedMazeWHSet, UpdateError};
        let (maze, mb_guard) = parse_input(input)?;
        let guard_initial = mb_guard.ok_or(ErrKindDay06::NoGuardFound { source_input: Some(input.to_string()) })?;
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
                Ok(Self { maze, guard_time_path: vec![guard] })
        }

        #[instrument()]
        pub fn update(&mut self) -> Option<Guard> {
                // circularly ordered by right-turns
                const DIR_ARRAY: [Direction; 4] = [Direction::Up, Direction::Right, Direction::Down, Direction::Left];
                let bounds = self.maze.max_dims;
                let guard_now = *self
                        .guard_time_path
                        .last()
                        .expect("guard_time_path should not be empty");
                let cycle_start_idx: usize = match guard_now.dir {
                        Direction::Up => 0,
                        Direction::Right => 1,
                        Direction::Down => 2,
                        Direction::Left => 3,
                };

                for i in 0..4 {
                        let dir = DIR_ARRAY[(cycle_start_idx + i).rem_euclid(4)];
                        let Some(check_pos) = guard_now.pos.try_move(dir, Some(bounds)) else {
                                // 'None' Means guard has escaped maze
                                return None;
                        };

                        if self.maze.get(check_pos).expect("check pos checked for being in bounds")
                                == PositionState::Empty
                        {
                                let new_guard = Guard::new(check_pos, dir);
                                self.guard_time_path.push(new_guard);
                                return Some(new_guard);
                        }
                }

                self.guard_time_path.push(guard_now);
                Some(guard_now)
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

#[cfg(test)]
mod tests {
        use pretty_assertions::assert_eq;
        use test_log::test;
        use tracing::instrument;

        use super::*;
        use crate::{EXAMPLE_INPUT, FINAL_INPUT};

        #[test]
        #[cfg_attr(feature = "manual-walkthrough_p1", ignore = "Manual interaction required")]
        #[instrument]
        fn test_process_example() -> Result<()> {
                let input = EXAMPLE_INPUT;
                let expected = 41;
                assert_eq!(process_part1(input)?, expected);
                Ok(())
        }

        /// Test's expected value to be populated after solution verification.
        #[test]
        #[cfg_attr(feature = "manual-walkthrough_p1", ignore = "Manual interaction required")]
        #[instrument]
        fn test_process_problem_input() -> Result<()> {
                let input = FINAL_INPUT;
                let expected = 4_711;
                assert_eq!(process_part1(input)?, expected);
                Ok(())
        }
}
