//! Error & Result type for Day02 of Advent of Code 2024.

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
