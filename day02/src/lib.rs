//! Library code for Day02 of Advent of Code 2024.

mod part1_lib;
mod part2_lib;
mod support;

pub use part1_lib::{process_part1_rayon, process_part1_serial};
pub use part2_lib::{process_part2_rayon, process_part2_serial};
pub use support::{Error, Result, generate_tracing_subscriber};

pub const FINAL_INPUT: &str = include_str!("../data/final_input.txt");
pub const EXAMPLE_INPUT: &str = include_str!("../data/example_input.txt");
pub const CUSTOM_INPUT: &str = include_str!("../data/custom_input.txt");

mod parse {
        use derive_more::derive::{Add, Constructor, Deref, DerefMut, From, Into, Sum};
        use rayon::prelude::*;
        use tracing::{self as tea, Level, instrument};

        use crate::Result;

        /// Report (one line) of Rudolf Reactor readings.
        #[derive(Debug, Clone, Deref, DerefMut, From, Into)]
        pub struct LineReport(pub Vec<i64>);

        /// Difference between two values.  (Discrete First Derivative)
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Deref, DerefMut, Constructor, Add, Sum)]
        pub struct Difference(i64);

        /// Safe: all levels same sign and (1..=3).contains()
        #[derive(Debug, PartialEq, Eq)]
        pub enum ReportStatus {
                Safe,
                Unsafe,
        }

        /// [Perf test] Original serial parse
        /// Parse txt input of spaced positive integers into line-wise reports (vecs)
        #[instrument(skip_all, ret(level = Level::TRACE))]
        pub fn parse_input_serial(raw_input: &str) -> Result<Vec<LineReport>> {
                tea::trace!(raw_input);
                let mut out = Vec::new();
                for line in raw_input.lines() {
                        let x: Result<Vec<_>> = line
                                .split_whitespace()
                                .map(|x| x.parse::<i64>().map_err(|e| e.into()))
                                .collect();
                        out.push(x?.into());
                }
                Ok(out)
        }
        /// [Perf test] Rayon parallel parse -- Note: overhead is higher than gain for default full text
        /// Parse txt input of spaced positive integers into line-wise reports (vecs)
        #[instrument(skip_all, ret(level = Level::TRACE))]
        pub fn parse_input_rayon(raw_input: &str) -> Result<Vec<LineReport>> {
                tea::trace!(raw_input);
                let mut out = Vec::new();
                for line in raw_input.lines() {
                        let x: Result<Vec<_>> = line
                                .par_split_whitespace()
                                .map(|x| x.parse::<i64>().map_err(|e| e.into()))
                                .collect();
                        out.push(x?.into());
                }
                Ok(out)
        }
}
