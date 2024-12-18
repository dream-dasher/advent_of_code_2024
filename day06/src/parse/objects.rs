//! Objects for Parsing into

use derive_more::derive::{From, Index, Into};
use itertools::Itertools as _;
use tracing::{Level, event, instrument};

use crate::{Result,
            support::{ErrWrapperDay06, error::ErrKindDay06}};

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
impl Maze {
        #[instrument(skip_all, err, ret(level = Level::TRACE))]
        fn new(positions: Vec<PositionState>, max_dims: Point2D) -> Result<Self> {
                if positions.len() != max_dims.x * max_dims.y {
                        Err("Maze dimensions do not match the positions vector length.".to_string())?
                }
                Ok(Self { positions, max_dims })
        }

        #[instrument(skip_all)]
        pub fn from_input_string(input: &str) -> Result<Self> {
                let positions: Result<Vec<PositionState>> =
                        input.chars().filter(|c| *c != '\n').map(|c| c.try_into()).collect();
                let max_dims = Point2D {
                        x: input.lines().next().unwrap().len(),
                        y: input.lines().count(),
                };
                Self::new(positions?, max_dims)
        }

        #[instrument(skip(self),ret(level = Level::DEBUG))]
        pub fn get(&self, point: Point2D) -> Option<PositionState> {
                self.pt_to_ln_index(point).map(|index| self.positions[index])
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
                        '\n' => Err(ErrKindDay06::ParseNewline { source_char: c })?,
                        '^' | 'v' | '<' | '>' => Err(ErrKindDay06::ParseUnexpectedDirection { source_char: c })?,
                        _ => Err(ErrKindDay06::ParseOther { source_char: c })?,
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
                        '\n' => Err(ErrKindDay06::ParseNewline { source_char: c })?,
                        '#' | '.' => Err(ErrKindDay06::ParseUnexpectedPositionState { source_char: c })?,
                        _ => Err(ErrKindDay06::ParseOther { source_char: c })?,
                }
        }
}
