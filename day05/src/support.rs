//! Support code for Day05 of Advent of Code 2024.

mod error;
mod subscriber;

pub use error::ErrWrapperDay05;
pub use subscriber::generate_tracing_subscriber;

pub type Result<T> = std::result::Result<T, ErrWrapperDay05>;
pub type Error = ErrWrapperDay05;