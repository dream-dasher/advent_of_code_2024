//! Raw-input parsing code for Day06 of Advent of Code 2024.

use std::{io, str::FromStr};

use derive_more::derive::{Add, AddAssign, Constructor, Deref, DerefMut, From, FromStr, Into, Sub, SubAssign};
use tracing::{Level, debug, instrument};

use super::*;
use crate::{Result, support::error::ErrKindDay06};

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
