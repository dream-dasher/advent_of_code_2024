//! Support code for Day04 of Advent of Code 2024.

mod error;
mod subscriber;

pub use error::{ErrKindDay04, Error, Result};
pub use subscriber::generate_tracing_subscriber;
