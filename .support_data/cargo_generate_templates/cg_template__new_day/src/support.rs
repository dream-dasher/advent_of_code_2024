//! Support code for {{ project-name | title_case }} of Advent of Code 2024.

mod error;
mod subscriber;

pub use error::{ErrKind{{ project-name | title_case }}, ErrWrapper{{ project-name | title_case }}};
pub use subscriber::generate_tracing_subscriber;

pub type Result<T> = std::result::Result<T, ErrWrapper{{ project-name | title_case }}>;
pub type Error = ErrWrapper{{ project-name | title_case }};
