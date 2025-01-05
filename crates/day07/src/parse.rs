//! Raw-input parsing code for Day07 of Advent of Code 2024.
//!
//! ## Data Observations:
//!
//! ```zsh
//! bat data/final_input.txt | choose 0 | sd : '' | sort -n
//! ```
//! largest solution in `final_input.tx`:          `84_474_639_541_600`
//!                             u32::MAX:               `4_294_967_295`
//!        u128::MAX.isqrt() == u64::MAX:  `18_446_744_073_709_551_615`
//!
//! So about 20k x *larger* than `u32` and about 200k x *smaller* than `u64`
//! (it is also significantly greater than the isqrt() of `u64`)

use derive_more::derive::{Display, Index};
// use derive_more::derive::{Constructor, Deref, DerefMut, From, Into};
use tracing::{Level, instrument};

use crate::{Result, support::error::ErrKindDay07};

/// Parse txt input ...
#[instrument(skip_all)]
pub fn parse_input(raw_input: &str) -> Result<Vec<EquationUncertain>> {
        let mut eqs = Vec::new();
        for line in raw_input.lines() {
                // <solution>:
                let mut colon_split_itr = line.split(':');
                let solution: u128 = colon_split_itr
                        .next()
                        .ok_or_else(|| ErrKindDay07::InputNoColon { source_input: line.to_string() })?
                        .trim()
                        .parse()?;
                let components: Vec<u128> = colon_split_itr
                        .next()
                        .ok_or_else(|| ErrKindDay07::InputNoColon { source_input: line.to_string() })?
                        .split_whitespace()
                        .map(|s| s.parse::<u128>())
                        .collect::<std::result::Result<_, _>>()?;

                let eq = EquationUncertain::new(solution, components);
                eqs.push(eq);
        }
        Ok(eqs)
}

/// Represents the putative solution and the numeric components of an equation.
#[derive(Debug, Index, Clone)]
pub struct EquationUncertain {
        #[index]
        pub solution:   u128,
        pub components: Vec<u128>,
        pub operators:  Vec<Operator>,
}
impl EquationUncertain {
        pub fn new(solution: u128, components: Vec<u128>) -> Self {
                let operators = vec![Operator::Unknown; components.len().saturating_sub(1)];

                Self { solution, components, operators }
        }
}
impl std::fmt::Display for EquationUncertain {
        /// Write something like:
        /// or '7290 = 6 _ 8 * 6 + 15'
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} = ", self.solution)?;

                if let Some(first) = self.components.first() {
                        write!(f, "{}", first)?;
                }

                for (i, component) in self.components.iter().skip(1).enumerate() {
                        if let Some(operator) = self.operators.get(i) {
                                write!(f, " {} {}", operator, component)?;
                        }
                }

                Ok(())
        }
}

/// Operators that may be between compoentns
#[derive(Debug, Default, Display, Copy, Clone)]
pub enum Operator {
        #[display("+")]
        Add,
        #[display("*")]
        Multiply,
        #[default]
        #[display("_")]
        Unknown,
}
