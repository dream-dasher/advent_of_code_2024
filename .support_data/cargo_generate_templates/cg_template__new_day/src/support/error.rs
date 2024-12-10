//! Error & Result type for {{ project-name | title_case }} of Advent of Code 2024.
//!
//!
//! ## Utility reference
//! For adding backtrace to errors:
//! `#![feature(error_generic_member_access)]`
//! `use std::backtrace;`

use std::io;

use derive_more::{Display, Error, From};
use tracing::subscriber::SetGlobalDefaultError; // !

// use derive_more::{Display, Error, derive::From};
#[derive(Debug, Display, From, Error)]
pub enum ErrKind{{ project-name | title_case }} {
        #[display("io error: {}", source)]
        Io { source: io::Error },
        #[display("Error setting tracing subscriber default: {}", source)]
        TracingSubscriber { source: SetGlobalDefaultError },
        #[from(ignore)]
        #[display("Unlabelled error (dyn error object): {}", source)]
        OtherDynError {
                source: Box<dyn std::error::Error + Send + Sync>,
        },
        // #[display("Error extracting lines from input: {}", source_input)]
        // NoInputLines { source_input: String },
        // #[from(ignore)]
        // #[display("error parsing char: {}", uninterpretable_char)]
        // CharParse { uninterpretable_char: char },
        // #[display("parse error: {}", source)]
        // ParseInt { source: num::ParseIntError },
        // #[display("env variable error: {}", source)]
        // Env { source: env::VarError },
}
impl ErrKind{{ project-name | title_case }} {
        pub fn make_other_error<E>(error: E) -> Self
        where
                E: Into<Box<dyn std::error::Error + Send + Sync>>,
        {
                Self::OtherDynError { source: error.into() }
        }
}

#[derive(Debug, Display, Error, From)]
#[display(
        "error: {:#}\n\n\nspantrace capture: {:?}\n\n\nspantrace: {:#}",
        source,
        spantrace.status(),
        spantrace,
)]
pub struct ErrWrapper{{ project-name | title_case }} {
        source:    ErrKind{{ project-name | title_case }},
        spantrace: tracing_error::SpanTrace,
        // backtrace: backtrace::Backtrace,
}
impl<T> From<T> for ErrWrapper{{ project-name | title_case }}
where
        T: Into<ErrKind{{ project-name | title_case }}>,
{
        fn from(error: T) -> Self {
                Self {
                        source:    error.into(),
                        spantrace: tracing_error::SpanTrace::capture(),
                        // backtrace: backtrace::Backtrace::capture(),
                }
        }
}

#[expect(dead_code)]
trait ToOther {
        fn to_other(self) -> ErrWrapper{{ project-name | title_case }};
}
impl<E> ToOther for E
where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
        fn to_other(self) -> ErrWrapper{{ project-name | title_case }} {
                ErrKind{{ project-name | title_case }}::OtherDynError { source: self.into() }.into()
        }
}
