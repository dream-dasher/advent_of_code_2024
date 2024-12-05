//! Library code for Day02 of Advent of Code 2024.

mod part1_lib;
mod part2_lib;
mod support;

pub use part1_lib::process_part1;
pub use part2_lib::process_part2;
pub use support::{Error, Result, generate_tracing_subscriber};

pub const FINAL_INPUT: &str = include_str!("../data/final_input.txt");
pub const EXAMPLE_INPUT: &str = include_str!("../data/example_input.txt");
pub const CUSTOM_INPUT: &str = include_str!("../data/custom_input.txt");

mod parse {
        use derive_more::derive::{Constructor, Deref, DerefMut, From, Into};
        use tracing::{self as tea, Level, instrument};

        use crate::Result;

        /// Report (one line) of Rudolf Reactor readings.
        #[derive(Debug, Clone, Deref, DerefMut, From, Into)]
        pub struct LineReport(Vec<i64>);

        /// Difference between two values.  (Discrete First Derivative)
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Deref, DerefMut, Constructor)]
        pub struct Difference(i64);

        /// Safe: all levels same sign and (1..=3).contains()
        #[derive(Debug, PartialEq, Eq)]
        pub enum ReportStatus {
                Safe,
                Unsafe,
        }

        /// Parse txt input of spaced positive integers into line-wise reports (vecs)
        #[instrument(skip_all, ret(level = Level::DEBUG))]
        pub fn parse_input(raw_input: &str) -> Result<Vec<LineReport>> {
                let mut out = Vec::new();
                for line in raw_input.lines() {
                        let x: Result<Vec<_>> = line
                                .split_whitespace()
                                .map(|x| x.parse::<i64>().map_err(|e| e.into()))
                                .collect();
                        out.push(x?.into());
                }
                tea::info!(?out);
                Ok(out)
        }

        // /// Example use of regex crate capture for parsing.
        // #[instrument]
        // pub fn example_parse() -> Result<Vec<[String; 3]>> {
        //         const EXAMPLE_PATH_SPLIT_REGEX: &str = r"^(?m)^([^:]+):([0-9]+):(.+)$";
        //         let re = Regex::new(EXAMPLE_PATH_SPLIT_REGEX).unwrap();
        //         tea::info!(?re);

        //         let hay = indoc!("\
        //         path/to/foo:54:Blue Harvest
        //         path/to/bar:90:Something, Something, Something, Dark Side
        //         path/to/baz:3:It's a Trap!
        //         path/topos/babos:36:ZZzzaZZZaaaZalooong!
        //         ");
        //         tea::info!(?hay);

        //         let mut out = Vec::new();
        //         {
        //                 let _enter = tea::info_span!("Parsing").entered();
        //                 for (i, line) in hay.lines().enumerate() {
        //                         let (raw, [path, lineno, line]) = re.captures(line).unwrap().extract();
        //                         tea::info!(path, lineno, line, raw, i);
        //                         out.push([path.to_string(), lineno.to_string(), line.to_string()]);
        //                 }
        //         }
        //         tea::info!(?out);
        //         Ok(out)
        // }
}
