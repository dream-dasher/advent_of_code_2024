//! Support code for Day05 of Advent of Code 2024.

mod error;
mod subscriber;

pub use error::{ErrKindDay05, ErrWrapperDay05};
pub use subscriber::active_global_default_tracing_subscriber;

pub type Result<T> = std::result::Result<T, ErrWrapperDay05>;
pub type Error = ErrWrapperDay05;
