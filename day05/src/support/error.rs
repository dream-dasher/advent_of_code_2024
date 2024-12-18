//! Error & Result type for Day05 of Advent of Code 2024.
//!
//!
//! ## Utility reference
//! For adding backtrace to errors:
//! `#![feature(error_generic_member_access)]`
//! `use std::backtrace;`

use std::{io, num};

use derive_more::{Display, Error, From};
use tracing::subscriber::SetGlobalDefaultError; // !

// use derive_more::{Display, Error, derive::From};
#[derive(Debug, Display, From, Error)]
pub enum ErrKindDay05 {
        #[display("io error: {}", source)]
        Io { source: io::Error },
        #[display("parse error: {}", source)]
        ParseInt { source: num::ParseIntError },
        #[display("Error setting tracing subscriber default: {}", source)]
        TracingSubscriber { source: SetGlobalDefaultError },
        #[from(ignore)]
        #[display("Unlabelled error (dyn error object): {}", source)]
        OtherDynError {
                source: Box<dyn std::error::Error + Send + Sync>,
        },
        #[display("Error extracting lines from input: {}", source_input)]
        OrderPatternError { source_input: String },
        #[display("Ordering Shapes does not Allow for rules to be a total ordering")]
        NonTotalOrderingShape,
        #[display("Unable to set the Static Page Relations OnceLock")]
        StaticPageRelationsSetFailure,
        // #[from(ignore)]
        // #[display("error parsing char: {}", uninterpretable_char)]
        // CharParse { uninterpretable_char: char },
        // #[display("env variable error: {}", source)]
        // Env { source: env::VarError },
}
impl ErrKindDay05 {
        pub fn make_other_error<E>(error: E) -> Self
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
pub struct ErrWrapperDay05 {
        pub source:    ErrKindDay05,
        pub spantrace: tracing_error::SpanTrace,
        // backtrace: backtrace::Backtrace,
}
impl<T> From<T> for ErrWrapperDay05
where
        T: Into<ErrKindDay05>,
{
        fn from(error: T) -> Self {
                Self {
                        source:    error.into(),
                        spantrace: tracing_error::SpanTrace::capture(),
                        // backtrace: backtrace::Backtrace::capture(),
                }
        }
}
// Using custom display as debug so we can get SpanTrace auto printed.
impl std::fmt::Debug for ErrWrapperDay05 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self)
        }
}

#[expect(dead_code)]
trait ToOther {
        fn to_other(self) -> ErrWrapperDay05;
}
impl<E> ToOther for E
where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
        fn to_other(self) -> ErrWrapperDay05 {
                ErrKindDay05::OtherDynError { source: self.into() }.into()
        }
}
