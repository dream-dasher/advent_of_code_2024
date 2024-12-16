//! Checking out maze generation.
//!
//! ## Utility
//!
/*!
clear; RUST_LOG=trace cargo run -qp day06 --bin xp_gen_maze
clear; RUST_LOG=trace cargo run -qp day06 --bin xp_gen_maze -- --help
clear; RUST_LOG=trace cargo run -qp day06 --bin xp_gen_maze manual 10 5,5 up
clear; RUST_LOG=trace cargo run -qp day06 --bin xp_gen_maze manual 10 '(5,5)' '>'
   */

use std::str::FromStr;

use clap::{Parser, Subcommand, ValueEnum};
use cli_input::*;
use day06::{Result, active_global_default_tracing_subscriber};
use derive_more::derive::{Add, AddAssign, Constructor, Display, From, FromStr, Into, Sub, SubAssign};
use rand::Rng;
use tracing::{info, instrument, trace};

fn main() -> Result<()> {
        let _writer_guard = active_global_default_tracing_subscriber()?;
        let args = Args::try_parse()?;

        let maze_str = match args.action {
                SubCom::Default => input_maze_generator(None),
                SubCom::Manual {
                        sq_side_len,
                        start_pos,
                        direction,
                } => {
                        info!(%start_pos, %direction);
                        input_maze_generator(Some(sq_side_len))
                }
        };
        let side_len = maze_str
                .find('\n')
                .ok_or_else(|| "no newline in string maze - possibly zero sized".to_string())?;
        println!("\n{}\n", maze_str);
        println!("size:: {}x{}", side_len, side_len);
        trace!("finishing main()");
        Ok(())
}

/// Generates a square maze as ch row
#[instrument]
fn input_maze_generator(sq_side_len: Option<usize>) -> String {
        let mut rng = rand::thread_rng();
        let side_len = sq_side_len.unwrap_or(rng.gen_range(1..=300));
        trace!(side_len);
        let chance_of_obstacle: f64 = rng.gen_range(0.01..=0.3);
        let is_obstacle_gen = std::iter::from_fn(move || Some(rng.gen_bool(chance_of_obstacle)));
        let row_iter = is_obstacle_gen
                .take(side_len)
                .map(|is_obstacle| if is_obstacle { '#' } else { '.' })
                .chain(std::iter::once('\n'));
        let maze_string: String = row_iter.cycle().take(side_len * (side_len + 1)).collect();
        maze_string
}

/// Argument related information.  Held in a mod for tidiness's sake.
mod cli_input {
        use super::*;

        /// Make a maze.  On auto-pilot or with manual control.
        #[derive(Parser, Debug)]
        #[command(version, about, long_about, disable_help_subcommand = true)]
        pub struct Args {
                /// sigh
                #[command(subcommand)]
                pub action: SubCom,
        }
        /// Default options or Manual controls
        #[derive(Debug, Clone, Subcommand)]
        pub enum SubCom {
                /// Default Options
                Default,
                /// Set side length of the maze square, and direction & start position of 'character'
                Manual {
                        sq_side_len: usize,
                        start_pos:   Position,
                        direction:   Direction,
                },
        }
        #[derive(
                Constructor,
                Clone,
                Copy,
                From,
                FromStr,
                Into,
                PartialEq,
                Eq,
                PartialOrd,
                Ord,
                Debug,
                Add,
                Sub,
                SubAssign,
                AddAssign,
                Display,
        )]
        pub struct Xpos(usize);
        #[derive(
                Constructor,
                Clone,
                Copy,
                From,
                FromStr,
                Into,
                PartialEq,
                Eq,
                PartialOrd,
                Ord,
                Debug,
                Add,
                Sub,
                SubAssign,
                AddAssign,
                Display,
        )]
        pub struct Ypos(usize);

        #[derive(
                Clone,
                Copy,
                Constructor,
                From,
                Into,
                PartialEq,
                PartialOrd,
                Parser,
                Debug,
                Add,
                Sub,
                SubAssign,
                AddAssign,
                Display,
        )]
        #[display("({},{})", x, y)]
        pub struct Position {
                x: Xpos,
                y: Ypos,
        }
        impl FromStr for Position {
                type Err = PositionParseError;

                /// Takes `(\d+,\d+)` or `\d+,\d+` and creates a Position.
                ///
                /// ## Caveat
                /// CLI input like `(...)` may not be parsed correctly by the shell unless quoted (e.g. `"(1,2)"` vs `(1,2)` )
                #[instrument]
                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                        let coords: Vec<&str> = s.trim_matches(['(', ')']).split(',').collect();
                        if coords.len() != 2 {
                                return Err(PositionParseError::Format {
                                        source_string: s.to_string(),
                                });
                        }
                        Ok(Self {
                                x: coords[0].parse().map_err(|_| PositionParseError::Parse {
                                        source_string: coords[0].to_string(),
                                })?,
                                y: coords[1].parse().map_err(|_| PositionParseError::Parse {
                                        source_string: coords[1].to_string(),
                                })?,
                        })
                }
        }

        /// Error for Position parsing.
        #[derive(Debug, Display, derive_more::Error)]
        pub enum PositionParseError {
                #[display("Invalid position format: {}", source_string)]
                Format { source_string: String },
                #[display("Parse error: {}", source_string)]
                Parse { source_string: String },
        }

        /// Direction of Facing.
        ///
        /// `up` : `^`
        /// `down` : `v`
        /// `left` : `<`
        /// `right` : `>`
        #[derive(Debug, PartialEq, Clone, ValueEnum, Display)]
        pub enum Direction {
                #[display("^")]
                #[value(alias = "^")]
                Up,
                #[display("v")]
                #[value(alias = "v")]
                Down,
                #[display("<")]
                #[value(alias = "<")]
                Left,
                #[display(">")]
                #[value(alias = ">")]
                Right,
        }

        impl FromStr for Direction {
                type Err = String;

                #[instrument]
                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                        match s.to_lowercase().as_str() {
                                "up" | "^" => Ok(Direction::Up),
                                "down" | "v" => Ok(Direction::Down),
                                "left" | "<" => Ok(Direction::Left),
                                "right" | ">" => Ok(Direction::Right),
                                _ => Err(format!("Invalid direction: {}", s)),
                        }
                }
        }
}
