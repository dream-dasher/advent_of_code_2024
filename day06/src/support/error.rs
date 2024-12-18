//! Error & Result type for Day06 of Advent of Code 2024.
//!
//!
//! ## Utility reference
//! For adding backtrace to errors:
//! `#![feature(error_generic_member_access)]`
//! `use std::backtrace;`

use std::io;

use derive_more::{Display, Error, From};
use tracing::{instrument, subscriber::SetGlobalDefaultError}; // !

// use derive_more::{Display, Error, derive::From};
#[derive(Debug, Display, From, Error)]
pub enum ErrKindDay06 {
        //
        // `custom` errors
        #[from(ignore)] // manually generate; would conflict with `OtherStringError` auto-derive
        #[display(
                "Requested Guard position is out of Maze bounds. guard_pos {:?} vs maze_max {:?}",
                guard_pos,
                maze_max
        )]
        GuardOutOfBounds {
                guard_pos: (usize, usize),
                maze_max:  (usize, usize),
        },
        #[from(ignore)]
        #[display("Newline character. {}", source_string)]
        ParseNewline { source_string: String },
        #[from(ignore)]
        #[display("Unparsable direction: {}", source_string)]
        ParseOther { source_string: String },
        #[from(ignore)]
        #[display("Error extracting lines from input: {}", source_input)]
        NoInputLines { source_input: String },
        //
        // `packed` errors
        #[display("CLI parsing library error: {}", source)]
        Clap { source: clap::Error },
        #[display("io error: {}", source)]
        Io { source: io::Error },
        #[display("Error setting tracing subscriber default: {}", source)]
        TracingSubscriber { source: SetGlobalDefaultError },
        //
        // `other` errors
        #[from(ignore)] // use `make_dyn_error` instead; would conflict with auto-derives
        #[display("Uncategorized Error (dyn error object): {}", source)]
        OtherDynError {
                source: Box<dyn std::error::Error + Send + Sync>,
        },
        #[display(r#"Uncategorized string err: "{}""#, source_string)]
        OtherStringError { source_string: String },
        //
        // // common error types
        // #[from(ignore)]
        // #[display("error parsing char: {}", uninterpretable_char)]
        // CharParse { uninterpretable_char: char },
        // #[display("parse error: {}", source)]
        // ParseInt { source: num::ParseIntError },
        // #[display("env variable error: {}", source)]
        // Env { source: env::VarError },
}
impl ErrKindDay06 {
        #[instrument(skip_all)]
        pub fn make_dyn_error<E>(error: E) -> Self
        where
                E: Into<Box<dyn std::error::Error + Send + Sync>>,
        {
                Self::OtherDynError { source: error.into() }
        }
}

#[derive(Display, Error, From)]
#[display(
        "error: {:#}\n\n\nspantrace capture: {:?}\n\n\nspantrace: {:#}",
        source,
        spantrace.status(),
        spantrace,
)]
pub struct ErrWrapperDay06 {
        source:    ErrKindDay06,
        spantrace: tracing_error::SpanTrace,
        // backtrace: backtrace::Backtrace,
}
// Using custom display as debug so we can get SpanTrace auto printed.
impl std::fmt::Debug for ErrWrapperDay06 {
        #[instrument(skip_all)]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self)
        }
}
impl<T> From<T> for ErrWrapperDay06
where
        T: Into<ErrKindDay06>,
{
        #[instrument(skip_all)]
        fn from(error: T) -> Self {
                Self {
                        source:    error.into(),
                        spantrace: tracing_error::SpanTrace::capture(),
                        // backtrace: backtrace::Backtrace::capture(),
                }
        }
}

pub trait ToOther {
        fn to_other(self) -> ErrWrapperDay06;
}
impl<E> ToOther for E
where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
        #[instrument(skip_all)]
        fn to_other(self) -> ErrWrapperDay06 {
                ErrKindDay06::OtherDynError { source: self.into() }.into()
        }
}
