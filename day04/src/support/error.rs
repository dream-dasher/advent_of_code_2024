//! Error & Result type for Day04 of Advent of Code 2024.

// // #![feature(error_generic_member_access)]
// use std::backtrace;

use std::io;

use derive_more::{Display, Error, From};
use tracing::subscriber::SetGlobalDefaultError; // !

// use derive_more::{Display, Error, derive::From};
#[derive(Debug, Display, From, Error)]
pub enum ErrKindDay04 {
        #[from(ignore)]
        #[display("error parsing char: {}", uninterpretable_char)]
        CWCharParse { uninterpretable_char: char },
        #[display("Error extracting lines from input: {}", source_input)]
        NoInputLines { source_input: String },
        // #[display("parse error: {}", source)]
        // Parse { source: num::ParseIntError },
        // #[display("env variable error: {}", source)]
        // Env { source: env::VarError },
        #[display("Error setting tracing subscriber default: {}", source)]
        TracingSubscriber { source: SetGlobalDefaultError },
        #[display("io error: {}", source)]
        Io { source: io::Error },
        #[from(ignore)]
        #[display("Unlabelled error (dyn error object): {}", source)]
        OtherDynError {
                source: Box<dyn std::error::Error + Send + Sync>,
        },
}
impl ErrKindDay04 {
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
pub struct ErrWrapper {
        source:    ErrKindDay04,
        spantrace: tracing_error::SpanTrace,
        // backtrace: backtrace::Backtrace,
}
impl<T> From<T> for ErrWrapper
where
        T: Into<ErrKindDay04>,
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
        fn to_other(self) -> ErrWrapper;
}
impl<E> ToOther for E
where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
        fn to_other(self) -> ErrWrapper {
                ErrKindDay04::OtherDynError { source: self.into() }.into()
        }
}
