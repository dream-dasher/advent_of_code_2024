//! Support code for Day03 of Advent of Code 2024.

mod error;
mod subscriber;

pub use error::{Error, Result};
pub use subscriber::generate_tracing_subscriber;
