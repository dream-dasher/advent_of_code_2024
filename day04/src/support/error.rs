//! Error & Result type for Day04 of Advent of Code 2024.

// // #![feature(error_generic_member_access)]
// use std::backtrace;

use std::io;

use derive_more::{Display, Error as DMError, From};
use tracing::subscriber::SetGlobalDefaultError; // !

pub type Result<T> = std::result::Result<T, ErrKindDay04>;
pub type Error = ErrWrapper;

// use derive_more::{Display, Error, derive::From};
#[derive(Debug, Display, DMError, From)]
pub enum ErrKindDay04 {
        #[display("error parsing char: {}", uninterpretable_char)]
        #[from(ignore)]
        CWCharParseError { uninterpretable_char: char },
        // #[display("parse error: {}", source)]
        // ParseError { source: num::ParseIntError },
        // #[display("env variable error: {}", source)]
        // EnvError { source: env::VarError },
        #[display("Error setting tracing subscriber default: {}", source)]
        TracingSubscriberError { source: SetGlobalDefaultError },
        #[display("io error: {}", source)]
        IoError { source: io::Error },
        #[display("Uncategorized error: {}", source)]
        #[from(ignore)]
        OtherError {
                source: Box<dyn std::error::Error + Send + Sync>,
        },
}
impl ErrKindDay04 {
        pub fn make_other_error<E>(error: E) -> Self
        where
                E: Into<Box<dyn std::error::Error + Send + Sync>>,
        {
                Self::OtherError { source: error.into() }
        }
}

#[derive(Debug, Display, DMError, From)]
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
                ErrKindDay04::OtherError { source: self.into() }.into()
        }
}
