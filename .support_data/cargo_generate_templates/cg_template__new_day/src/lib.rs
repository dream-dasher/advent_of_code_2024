//! Library code for {{ project-name | title_case }} of Advent of Code 2024.

mod part1_lib;
mod part2_lib;
mod support;

pub use part1_lib::process_part1;
pub use part2_lib::process_part2;
pub use support::{Error, Result, generate_tracing_subscriber};

pub const FINAL_INPUT: &str = include_str!("../data/final_input.txt");
pub const EXAMPLE_INPUT: &str = include_str!("../data/example_input.txt");

mod parse {
        // use derive_more::derive::{Constructor, Deref, DerefMut, From, Into};
        use tracing::{self as tea, Level, instrument};

        use crate::Result;

        /// Parse txt input of spaced postive integers into line-wise reports (vecs)
        #[instrument(skip_all, ret(level = Level::DEBUG))]
        pub fn parse_input(raw_input: &str) -> Result<Vec<LineReport>> {
                todo!()
        }

        /// Example use of regex crate capture for parsing.
        #[instrument(skip_all, ret(level = Level::INFO))]
        pub fn example_parse() -> Result<Vec<[String; 3]>> {
                const EXAMPLE_PATH_SPLIT_REGEX: &str = r"^(?m)^([^:]+):([0-9]+):(.+)$";
                let re = Regex::new(EXAMPLE_PATH_SPLIT_REGEX).expect("string should be valid regex");
                tea::info!(?re);
                const HAY: &str = indoc!("\
                path/to/foo:54:Blue Harvest
                path/to/bar:90:Something, Something, Something, Dark Side
                path/to/baz:3:It's a Trap!
                path/topos/babos:36:ZZzzaZZZaaaZalooong!
                ");
                tea::info!(?HAY);

                let mut out = Vec::new();
                {
                        let _enter = tea::info_span!("Parsing").entered();
                        for (i, line) in HAY.lines().enumerate() {
                                let (raw, [path, lineno, line]) = re.captures(line).unwrap().extract();
                                tea::info!(path, lineno, line, raw, i);
                                out.push([path.to_string(), lineno.to_string(), line.to_string()]);
                        }
                }
                Ok(out)
        }
}
