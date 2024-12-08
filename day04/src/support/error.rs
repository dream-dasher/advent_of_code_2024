//! Error & Result type for Day04 of Advent of Code 2024.

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error + Send + Sync>;
