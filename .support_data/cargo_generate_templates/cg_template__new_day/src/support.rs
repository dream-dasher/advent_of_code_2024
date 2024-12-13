//! Support code for {{ project-name | upper_camel_case }} of Advent of Code 2024.

mod error;
mod subscriber;

pub use error::ErrWrapper{{ project-name | upper_camel_case }};
pub use subscriber::generate_tracing_subscriber;

pub type Result<T> = std::result::Result<T, ErrWrapper{{ project-name | upper_camel_case }}>;
pub type Error = ErrWrapper{{ project-name | upper_camel_case }};
