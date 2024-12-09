//! Support code for Day04 of Advent of Code 2024.

mod error;
mod subscriber;

pub use error::{ErrKindDay04, ErrWrapper};
pub use subscriber::generate_tracing_subscriber;

pub type Result<T> = std::result::Result<T, ErrKindDay04>;
pub type Error = ErrWrapper;
