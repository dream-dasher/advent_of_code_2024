//! Raw-input parsing code for Day06 of Advent of Code 2024.

use std::io;

use tracing::{Level, debug, instrument};

use crate::Result;

mod brute_simulate {
        use std::str::FromStr;

        use super::*;

        #[derive(Debug, PartialEq, Eq)]
        pub struct Maze {
                maze:    Vec<Direction>,
                row_len: usize,
                col_len: usize,
        }
        impl std::fmt::Display for Maze {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        let row_len = self.row_len;
                        for (i, cell) in self.maze.iter().enumerate() {
                                write!(f, "{}", cell)?;
                                if i % row_len == row_len - 1 {
                                        writeln!(f)?;
                                }
                        }
                        debug_assert_eq!(self.maze.len(), (row_len * self.col_len));
                        Ok(())
                }
        }
        // impl FromStr for Maze {
        //         type Err = MazeParseError;

        //         #[instrument]
        //         fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        //                 let mut maze = Vec::new();
        //                 let mut row_len = 0;
        //                 let mut col_len = 0;
        //                 for (i, line) in s.lines().enumerate() {
        //                         let row: Vec<Cell> =
        //                                 line.chars().map(|c| c.try_into()).collect::<Result<Vec<Cell>>>()?;
        //                         if i == 0 {
        //                                 row_len = row.len();
        //                         } else {
        //                                 if row.len() != row_len {
        //                                         return Err(MazeParseError::RowLengthMismatch {
        //                                                 row_num:  i,
        //                                                 expected: row_len,
        //                                                 found:    row.len(),
        //                                         });
        //                                 }
        //                         }
        //                         maze.extend(row);
        //                         col_len += 1;
        //                 }
        //                 Ok(Self { maze, row_len, col_len })
        //         }
        // }

        #[derive(Copy, Clone, PartialEq, Eq, Debug, derive_more::Display)]
        pub enum Cell {
                #[display(".")]
                Empty,
                #[display("#")]
                Obstacle,
                // // Unneeded when there's a single guard and none can interact
                // Guard { dir: Direction },
        }
        impl TryFrom<char> for Cell {
                type Error = CellParseError;

                #[instrument(skip_all)]
                fn try_from(c: char) -> std::result::Result<Self, Self::Error> {
                        match c {
                                '.' => Ok(Cell::Empty),
                                '#' => Ok(Cell::Obstacle),
                                '\n' => Err(CellParseError::Newline {
                                        source_string: c.to_string(),
                                }),
                                _ => Err(CellParseError::OtherParse {
                                        source_string: c.to_string(),
                                }),
                        }
                }
        }
        impl FromStr for Cell {
                type Err = CellParseError;

                #[instrument]
                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                        match s.to_lowercase().as_str() {
                                "." => Ok(Cell::Empty),
                                "#" => Ok(Cell::Obstacle),
                                "\n" => Err(CellParseError::Newline {
                                        source_string: s.to_string(),
                                }),
                                _ => Err(CellParseError::OtherParse {
                                        source_string: s.to_string(),
                                }),
                        }
                }
        }
        /// Direction parsing.
        #[derive(Debug, derive_more::Display, derive_more::Error)]
        pub enum CellParseError {
                #[display("Newline character. {}", source_string)]
                Newline { source_string: String },
                #[display("Unparsable direction: {}", source_string)]
                OtherParse { source_string: String },
        }

        /// Direction of Facing.
        ///
        /// `up` : `^`
        /// `down` : `v`
        /// `left` : `<`
        /// `right` : `>`
        #[derive(Copy, Clone, PartialEq, Eq, Debug, derive_more::Display)]
        pub enum Direction {
                #[display("^")]
                Up,
                #[display("v")]
                Down,
                #[display("<")]
                Left,
                #[display(">")]
                Right,
        }
        impl TryFrom<char> for Direction {
                type Error = DirectionParseError;

                #[instrument(skip_all)]
                fn try_from(c: char) -> std::result::Result<Self, Self::Error> {
                        match c {
                                '^' => Ok(Direction::Up),
                                'v' => Ok(Direction::Down),
                                '<' => Ok(Direction::Left),
                                '>' => Ok(Direction::Right),
                                '\n' => Err(DirectionParseError::Newline {
                                        source_string: c.to_string(),
                                }),
                                _ => Err(DirectionParseError::OtherParse {
                                        source_string: c.to_string(),
                                }),
                        }
                }
        }
        impl FromStr for Direction {
                type Err = DirectionParseError;

                #[instrument]
                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                        match s.to_lowercase().as_str() {
                                "up" | "^" => Ok(Direction::Up),
                                "down" | "v" => Ok(Direction::Down),
                                "left" | "<" => Ok(Direction::Left),
                                "right" | ">" => Ok(Direction::Right),
                                "\n" => Err(DirectionParseError::Newline {
                                        source_string: s.to_string(),
                                }),
                                _ => Err(DirectionParseError::OtherParse {
                                        source_string: s.to_string(),
                                }),
                        }
                }
        }
        /// Direction parsing.
        #[derive(Debug, derive_more::Display, derive_more::Error)]
        pub enum DirectionParseError {
                #[display("Newline character. {}", source_string)]
                Newline { source_string: String },
                #[display("Unparsable direction: {}", source_string)]
                OtherParse { source_string: String },
        }
}

/// Parse txt input ...
#[instrument(skip_all, ret(level = Level::TRACE))]
pub fn parse_input(raw_input: &str) -> Result<()> {
        println!("\n{}\n", raw_input);
        dirty_pause()?;
        todo!()
}

/// Quick and dirty pause button so I can watch as program runs.
#[instrument]
fn dirty_pause() -> Result<()> {
        println!("Press Enter to continue...");
        let mut _input = String::new();
        let read_in = io::stdin().read_line(&mut _input)?;
        debug!(?read_in);
        Ok(())
}

#[cfg(test)]
mod tests {
        use indoc::indoc;
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
        fn example_test() -> Result<()> {
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
                let expected_ys = 10;
                let expected_xs = 10;
                let vv: Vec<&str> = input.lines().collect();
                let found_ys = vv.len();
                let found_xs = vv[0].len();
                assert_eq!(found_ys, expected_ys);
                assert_eq!(found_xs, expected_xs);
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
