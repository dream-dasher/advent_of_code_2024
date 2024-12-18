//! Objects for Parsing into

use bon::bon;
use derive_more::derive::{From, Index, Into};
use itertools::Itertools as _;
use tracing::{Level, event, instrument};

use crate::support::{ErrWrapperDay06, error::ErrKindDay06};

#[derive(Index, Debug, Clone, From, Into)]
pub struct Maze {
        #[index]
        positions: Vec<PositionState>,
        max_dims:  Point2D,
}
impl std::fmt::Display for Maze {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                for (r, c) in (0..self.max_dims.y).cartesian_product(0..self.max_dims.x) {
                        if c == 0 {
                                writeln!(f)?
                        }
                        write!(f, "{} ", self.positions[r * self.max_dims.x + c])?;
                }
                Ok(())
        }
}
#[bon]
impl Maze {
        #[builder]
        #[instrument(skip_all, err, ret(level = Level::TRACE))]
        fn new(positions: Vec<PositionState>, max_dims: Point2D) -> Result<Self, String> {
                if positions.len() != max_dims.x * max_dims.y {
                        Err("Maze dimensions do not match the positions vector length."
                                .to_string())?
                }
                Ok(Self {
                        positions,
                        max_dims,
                })
        }

        #[instrument(skip(self),ret(level = Level::DEBUG))]
        fn get(&self, p: Point2D) -> Option<PositionState> {
                match self.pt_to_ln_index(p) {
                        Some(index) => Some(self.positions[index]),
                        None => None,
                }
        }

        #[instrument]
        fn pt_to_ln_index(&self, point: Point2D) -> Option<usize> {
                if point.x >= self.max_dims.x || point.y >= self.max_dims.y {
                        event![Level::DEBUG, %point, ?self.max_dims, "point out of bounds"];
                        None
                } else {
                        Some(point.y * self.max_dims.x + point.x)
                }
        }
}

/// State of Position
/// ('guard is not part state')
#[derive(Clone, Copy, PartialEq, Eq, Debug, derive_more::Display)]
pub enum PositionState {
        #[display("#")]
        Obstacle,
        #[display(".")]
        Empty,
}

#[derive(Clone, Copy, Debug, derive_more::Display, PartialEq, Eq)]
#[display("G_'{}'@{}", dir, pos)]
pub struct Guard {
        pos: Point2D,
        dir: Direction,
}

#[derive(Clone, Copy, From, Into, PartialEq, Eq, PartialOrd, Debug, derive_more::Display)]
#[display("({},{})", x, y)]
#[from(forward)]
pub struct Point2D {
        x: usize,
        y: usize,
}

/// Direction of Facing.
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

// impls
impl TryFrom<char> for PositionState {
        type Error = ErrWrapperDay06;

        #[instrument(skip_all)]
        fn try_from(c: char) -> std::result::Result<Self, Self::Error> {
                match c {
                        '#' => Ok(PositionState::Obstacle),
                        '.' => Ok(PositionState::Empty),
                        '\n' => Err(ErrKindDay06::ParseNewline {
                                source_string: c.to_string(),
                        })?,
                        _ => Err(ErrKindDay06::ParseOther {
                                source_string: c.to_string(),
                        })?,
                }
        }
}
impl TryFrom<char> for Direction {
        type Error = ErrWrapperDay06;

        #[instrument(skip_all)]
        fn try_from(c: char) -> std::result::Result<Self, Self::Error> {
                match c {
                        '^' => Ok(Direction::Up),
                        'v' => Ok(Direction::Down),
                        '<' => Ok(Direction::Left),
                        '>' => Ok(Direction::Right),
                        '\n' => Err(ErrKindDay06::ParseNewline {
                                source_string: c.to_string(),
                        })?,
                        _ => Err(ErrKindDay06::ParseOther {
                                source_string: c.to_string(),
                        })?,
                }
        }
}
