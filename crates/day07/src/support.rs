//! Support code for Day07 of Advent of Code 2024.

pub mod error;
mod subscriber;

pub use error::ErrWrapperDay07;
pub use subscriber::activate_global_default_tracing_subscriber;

pub type Result<T> = std::result::Result<T, ErrWrapperDay07>;
pub type Error = ErrWrapperDay07;
