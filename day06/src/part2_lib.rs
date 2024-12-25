//! Library code for Part 2 of Day06 of Advent of Code 2024.
//! `bin > part2_bin.rs` will run this code along with content of `input2.txt`

use std::collections::{HashMap, HashSet};

use tracing::{self as tea, Level, event, instrument};

use crate::{Result,
            parse::{Direction, Point2D, parse_input},
            part1_lib::PopulatedMaze,
            support::error::ErrKindDay06};

#[instrument(skip_all, ret(level = Level::DEBUG))]
pub fn process_part2(input: &str) -> Result<usize> {
        tea::trace!(%input);
        // Repeating Part1 to get vector of Guard-states
        let (maze, mb_guard) = parse_input(input)?;
        let guard_initial = mb_guard.ok_or(ErrKindDay06::NoGuardFound {
                source_input: Some(input.to_string()),
        })?;
        let mut pop_maze = PopulatedMaze::new(maze, guard_initial)?;
        for _i in 0.. {
                let opt_guard_update = pop_maze.update();
                if let Some(guard_update) = opt_guard_update {
                        if guard_update != guard_initial {
                                continue;
                        }
                }
                // repeat or None
                break;
        }

        let mut pos_hmap: HashMap<Point2D, HashSet<Direction>> = HashMap::new();
        let mut obstacle_positions: HashSet<Point2D> = HashSet::new();
        for guard_state in pop_maze.guard_time_path.iter() {
                let pos = guard_state.pos;
                let dir = guard_state.dir;
                let entry = pos_hmap.entry(pos).or_default();
                entry.insert(dir); // ??? `entry` is listed as &mut HashSet, but is not set as mutable...
                println!("entry: {:?}", entry);
                if entry.contains(&dir.rightward()) {
                        if let Some(pos) = pos.try_move(dir, Some(pop_maze.maze.max_dims)) {
                                event![Level::ERROR, "I need to check if obstacle there"];
                                obstacle_positions.insert(pos);
                        }
                }
        }

        println!("obstacle_positions: {:?}", obstacle_positions);
        Ok(obstacle_positions.len())
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
