//! Error & Result type for Day01 of Advent of Code 2024.

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error + Send + Sync>;

// #[derive(Error, Diagnostic, Debug)]
// pub enum AocErrorDay00 {
//         #[error(transparent)]
//         #[diagnostic(code(aoc::io_error))]
//         IoError(#[from] std::io::Error),
// }
