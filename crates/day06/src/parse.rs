//! Raw-input parsing code for Day06 of Advent of Code 2024.

mod objects;
pub use objects::*;
use tracing::{Level, instrument};

use crate::Result;

/// Parse txt input ...
#[instrument(skip_all, ret(level = Level::TRACE))]
pub fn parse_input(raw_input: &str) -> Result<(Maze, Option<Guard>)> {
        let (maze, mb_guard) = Maze::from_input_string(raw_input)?;
        tracing::event![tracing::Level::TRACE, %maze, ?mb_guard];

        Ok((maze, mb_guard))
}

#[cfg(test)]
mod tests {
        use indoc::indoc;
        use pretty_assertions::assert_eq;
        #[expect(unused)]
        use quickcheck::TestResult;
        #[expect(unused)]
        use quickcheck_macros::quickcheck;
        use rand::Rng;
        use test_log::test;
        use tracing::{instrument, trace};

        use super::*;

        #[test]
        #[instrument]
        fn input_parse_test() -> Result<()> {
                let input = indoc!("
                        ....#.....
                        .........#
                        ..........
                        ..#.......
                        .......#..
                        ..........
                        .#..^.....
                        ........#.
                        #.........
                        ......#...");
                let expected_dims = (10, 10);
                let expected_guard = Guard::new(Point2D::new(4, 6), Direction::Up);

                let (maze, mb_guard) = parse_input(input)?;
                let found_dims = maze.max_dims.into();
                let found_guard = mb_guard.expect("guard should found");
                assert_eq!(expected_dims, found_dims);
                assert_eq!(expected_guard, found_guard);
                Ok(())
        }

        /// Generates a square maze as a string of '.' and '#' characters.
        ///
        /// Returns a string representing a maze with:
        /// - Random size between 1x1 and 300x300
        /// - Random obstacles ('#') with 1-30% probability
        /// - Empty spaces ('.') for remaining cells
        /// - Newline character after each row
        #[instrument]
        fn input_maze_generator() -> String {
                let mut rng = rand::thread_rng();
                let side_len = rng.gen_range(1..=300);
                trace!(side_len);
                let chance_of_obstacle: f64 = rng.gen_range(0.01..=0.3);
                let is_obstacle_gen = std::iter::from_fn(move || Some(rng.gen_bool(chance_of_obstacle)));
                let row_iter = is_obstacle_gen
                        .take(side_len)
                        .map(|is_obstacle| if is_obstacle { '#' } else { '.' })
                        .chain(std::iter::once('\n'));
                let maze_string: String = row_iter.cycle().take(side_len * (side_len + 1)).collect();
                maze_string
        }

        // #[quickcheck]
        // #[instrument]
        // fn qc_example_quickcheck(sum: u16, step_range_inclusive: (u8, u8)) -> TestResult {
        //         let maze_input =
        //         let Some(vector) = example_iw_generator(sum, step_range_inclusive) else {
        //                 return TestResult::discard();
        //         };
        //         let vector_sum: i64 = vector.iter().sum();
        //         TestResult::from_bool(sum as i64 == vector_sum)
        // }
}
