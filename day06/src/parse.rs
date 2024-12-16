//! Raw-input parsing code for Day06 of Advent of Code 2024.

use std::io;

use tracing::{Level, debug, instrument};

use crate::Result;

mod brute_simulate {
        use std::str::FromStr;

        use derive_more::derive::{Add, AddAssign, Constructor, Deref, DerefMut, From, FromStr, Into, Sub, SubAssign};

        use super::*;
        use crate::support::error::ErrKindDay06;

        #[derive(Debug, PartialEq, Eq)]
        pub struct PopulatedMaze {
                maze:  Maze,
                guard: Guard,
        }
        impl PopulatedMaze {
                pub fn try_new(maze: Maze, guard: Guard) -> Result<PopulatedMaze> {
                        let (g_x, g_y) = (*guard.pos.x, *guard.pos.y);
                        let (m_x, m_y) = (maze.row_len, maze.col_len);
                        if m_x <= g_x || m_y <= g_y {
                                Err(ErrKindDay06::GuardOutOfBounds {
                                        guard_pos: (g_x, g_y),
                                        maze_max:  (m_x, m_y),
                                })?
                        }
                        Ok(Self { maze, guard })
                }

                pub fn update(&mut self) -> Result<()> {
                        let (g_x, g_y) = (*self.guard.pos.x, *self.guard.pos.y);
                        let mut is_path = false;
                        // look ahead
                        // while !is_path {}
                        let (d_x, d_y) = self.guard.dir.pos_delta();

                        todo!()
                }
        }
        #[derive(Debug, PartialEq, Eq)]
        pub struct Maze {
                cells:   Vec<Cell>,
                row_len: usize,
                col_len: usize,
        }
        impl Maze {
                pub fn get_pos(&self, pos: Position) -> Option<Cell> {
                        if *pos.x >= self.row_len || *pos.y >= self.col_len {
                                None
                        } else {
                                let idx = *pos.x + self.row_len * *pos.y;
                                Some(self.cells[idx])
                        }
                }

                pub fn get_adjacent(&self, pos: &Position) -> [Option<Cell>; 4] {
                        // let up = Position { x: pos.x, y: pos.y - 1 };
                        todo!()
                }
        }
        // impl FromStr for Maze {
        //         type Err = MazeParseError;

        //         // #[instrument]
        //         // fn from_str(s: &str) -> Result<Self> {
        //         //         let mut cells = Vec::new();
        //         //         let first_line = s.lines().next().ok_or(ErrKindDay06::NoInputLines {
        //         //                 source_input: s.to_string(),
        //         //         })?;
        //         //         let row_len = first_line.len();
        //         //         let mut col_len = 0;
        //         //         for
        //         //         first_line.chars().for_each(|c| {
        //         //                 cells.push(c.try_into().map_err(|_| MazeParseError::CellParse {
        //         //                         source_string: c.to_string(),
        //         //                 })?);
        //         //         });

        //         //         todo!();
        //         //         Ok(Self {
        //         //                 cells,
        //         //                 row_len,
        //         //                 col_len,
        //         //         })
        //         // }
        // }
        impl std::fmt::Display for Maze {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        let row_len = self.row_len;
                        for (i, cell) in self.cells.iter().enumerate() {
                                write!(f, "{}", cell)?;
                                if i % row_len == row_len - 1 {
                                        writeln!(f)?;
                                }
                        }
                        debug_assert_eq!(self.cells.len(), (row_len * self.col_len));
                        Ok(())
                }
        }
        #[derive(Debug, PartialEq, Eq)]
        pub struct Guard {
                pos: Position,
                dir: Direction,
        }

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
        impl Direction {
                pub fn pos_delta(&self) -> (isize, isize) {
                        match self {
                                Self::Up => (0, -1),
                                Self::Down => (0, 1),
                                Self::Left => (-1, 0),
                                Self::Right => (1, 0),
                        }
                }
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

        #[derive(
                Constructor,
                Clone,
                Copy,
                From,
                FromStr,
                Into,
                PartialEq,
                Eq,
                PartialOrd,
                Ord,
                Debug,
                Add,
                Sub,
                SubAssign,
                AddAssign,
                derive_more::derive::Display,
                Deref,
                DerefMut,
        )]
        pub struct Xpos(usize);
        #[derive(
                Constructor,
                Clone,
                Copy,
                From,
                FromStr,
                Into,
                PartialEq,
                Eq,
                PartialOrd,
                Ord,
                Debug,
                Add,
                Sub,
                SubAssign,
                AddAssign,
                derive_more::derive::Display,
                Deref,
                DerefMut,
        )]
        pub struct Ypos(usize);
        #[derive(
                Clone,
                Copy,
                Constructor,
                From,
                Into,
                PartialEq,
                PartialOrd,
                Debug,
                Add,
                Sub,
                SubAssign,
                AddAssign,
                derive_more::derive::Display,
                Eq,
        )]
        #[display("({},{})", x, y)]
        pub struct Position {
                x: Xpos,
                y: Ypos,
        }
        impl FromStr for Position {
                type Err = PositionParseError;

                /// Takes `(\d+,\d+)` or `\d+,\d+` and creates a Position.
                ///
                /// ## Caveat
                /// CLI input like `(...)` may not be parsed correctly by the shell unless quoted (e.g. `"(1,2)"` vs `(1,2)` )
                #[instrument]
                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                        let coords: Vec<&str> = s.trim_matches(['(', ')']).split(',').collect();
                        if coords.len() != 2 {
                                return Err(PositionParseError::Format {
                                        source_string: s.to_string(),
                                });
                        }
                        Ok(Self {
                                x: coords[0].parse().map_err(|_| PositionParseError::Parse {
                                        source_string: coords[0].to_string(),
                                })?,
                                y: coords[1].parse().map_err(|_| PositionParseError::Parse {
                                        source_string: coords[1].to_string(),
                                })?,
                        })
                }
        }
        /// Error for Position parsing.
        #[derive(Debug, derive_more::derive::Display, derive_more::Error)]
        pub enum PositionParseError {
                #[display("Invalid position format: {}", source_string)]
                Format { source_string: String },
                #[display("Parse error: {}", source_string)]
                Parse { source_string: String },
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
