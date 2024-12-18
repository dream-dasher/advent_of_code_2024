//! Objects for Parsing into

// NOTE: common derives for 'objects'
//       common impls like `try_from`
use tracing::instrument;

use crate::support::{ErrWrapperDay06, error::ErrKindDay06};

#[derive(Clone, Copy, From, Into, PartialEq, Eq, PartialOrd, Debug)]
pub struct Position {
        row: usize,
        col: usize,
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
