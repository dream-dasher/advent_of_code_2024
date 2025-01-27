//! Error & Result type for Day06 of Advent of Code 2024.
//!
//
//! ## Common ErrorKinds
//! // //
//! // // `custom` errors
//! // #[from(ignore)]
//! // #[display("Error extracting lines from input: {}", source_input)]
//! // InputNoLines { source_input: String },
//! // #[from(ignore)]
//! // #[display("error parsing char: {}", uninterpretable_char)]
//! // ParseChar { uninterpretable_char: char },
//! // #[from(ignore)]
//! // #[display("parse error: {}", source)]
//! // ParseInt { source: num::ParseIntError },
//! // #[display("Unparsable character: {}", source_char)]
//! // ParseOther { source_char: char },
//! // //
//! // // `packed` errors
//! // #[display("CLI parsing library error: {}", source)]
//! // Clap { source: clap::Error },
//! // #[display("Error with tracing_subscriber::EnvFilter parsing env directive: {}", source)]
//! // EnvError { source: tracing_subscriber::filter::FromEnvError },
//! // #[display("eframe (egui) error: {}", source)]
//! // EFrame { source: eframe::Error },
//! // #[display("io error: {}", source)]
//! // Io { source: io::Error },
//! // #[display("Error setting tracing subscriber default: {}", source)]
//! // TracingSubscriber { source: SetGlobalDefaultError },
//! // //
//! // // `other` errors
//! // #[from(ignore)] // use `make_dyn_error` instead; would conflict with auto-derives
//! // #[display("Uncategorized Error (dyn error object): {}", source)]
//! // OtherDynError { source: Box<dyn std::error::Error + Send + Sync> },
//! // #[display(r#"Uncategorized string err: "{}""#, source_string)]
//! // OtherStringError { source_string: String },
//!
//! ## Utility reference
//! For adding backtrace to errors:
//! `#![feature(error_generic_member_access)]`
//! `use std::backtrace;`

use std::io;

use derive_more::{Display, Error, From};
use tracing::instrument;

use crate::parse::{Point2D, PositionState};

// use derive_more::{Display, Error, derive::From};
#[derive(Debug, Display, From, Error)]
pub enum ErrKindDay06 {
        //
        // `custom` errors
        #[from(ignore)]
        #[display("No guard found in maze input")]
        NoGuardFound { source_input: Option<String> },
        // #[from(ignore)] // manually generate; would conflict with `OtherStringError` auto-derive
        #[display(
                "Guard position overlaps with non-empty maze position state: guard_pos {} vs position_state {}",
                guard_pos,
                position_state
        )]
        GuardOnNonEmptySpace { guard_pos: Point2D, position_state: PositionState },
        // #[from(ignore)] // manually generate; would conflict with `OtherStringError` auto-derive
        #[display(
                // note: raw Point2D instead of (usize, usize)  -- best practices?
                "Requested point is outside of Maze bounds. guard_pos {:?} vs maze_max {:?}",
                point,
                maze_max
        )]
        PointOutOfBounds {
                // note: down cast of Point2D to (usize, usize)  -- best practices?
                point:    Point2D,
                maze_max: Point2D,
        },
        #[display(
                // note: raw Point2D instead of (usize, usize)  -- best practices?
                "Requested Guard position is out of Maze bounds. guard_pos {:?} vs maze_max {:?}",
                guard_pos,
                maze_max
        )]
        GuardOutOfBounds {
                // note: down cast of Point2D to (usize, usize)  -- best practices?
                guard_pos: (usize, usize),
                maze_max:  (usize, usize),
        },
        #[from(ignore)]
        #[display("Newline character. {}", source_char)]
        ParseNewline { source_char: char },
        #[from(ignore)]
        #[display("Unexpected PositionState char: {}", source_char)]
        ParseUnexpectedPositionState { source_char: char },
        #[from(ignore)]
        #[display("Unexpected Direction char: {}", source_char)]
        ParseUnexpectedDirection { source_char: char },
        #[from(ignore)]
        #[display("Unparsable character: {}", source_char)]
        ParseOther { source_char: char },
        #[from(ignore)]
        #[display("Error extracting lines from input: {}", source_input)]
        InputNoLines { source_input: String },
        //
        // `packed` errors
        #[display("CLI parsing library error: {}", source)]
        Clap { source: clap::Error },
        #[display("Error with tracing_subscriber::EnvFilter parsing env directive: {}", source)]
        EnvError { source: tracing_subscriber::filter::FromEnvError },
        #[display("io error: {}", source)]
        Io { source: io::Error },
        #[display("Error setting tracing subscriber default: {}", source)]
        TracingSubscriber { source: tracing::subscriber::SetGlobalDefaultError },
        //
        // `other` errors
        #[from(ignore)] // use `make_dyn_error` instead; would conflict with auto-derives
        #[display("Uncategorized Error (dyn error object): {}", source)]
        OtherErrorDyn { source: Box<dyn std::error::Error + Send + Sync> },
        #[display(r#"Uncategorized string err: "{}""#, source_string)]
        OtherErrorString { source_string: String },
}
impl ErrKindDay06 {
        #[instrument(skip_all)]
        pub fn make_dyn_error<E>(error: E) -> Self
        where
                E: Into<Box<dyn std::error::Error + Send + Sync>>,
        {
                Self::OtherErrorDyn { source: error.into() }
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
        pub source:    ErrKindDay06,
        pub spantrace: tracing_error::SpanTrace,
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
                ErrKindDay06::OtherErrorDyn { source: self.into() }.into()
        }
}
