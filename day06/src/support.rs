//! Support code for Day06 of Advent of Code 2024.

mod error;
mod subscriber;

pub use error::ErrWrapperDay06;
pub use subscriber::active_global_default_tracing_subscriber;

pub type Result<T> = std::result::Result<T, ErrWrapperDay06>;
pub type Error = ErrWrapperDay06;
