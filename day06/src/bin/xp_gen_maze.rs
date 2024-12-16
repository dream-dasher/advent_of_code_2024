//* Checking out maze generation.

use std::str::FromStr;

use clap::{Parser, Subcommand, ValueEnum};
use day06::{Result, active_global_default_tracing_subscriber};
use derive_more::derive::{Add, AddAssign, Constructor, Display, From, FromStr, Into, Sub, SubAssign};
use rand::Rng;
use tracing::{info, instrument, trace};

/// Choose to run Part 1 or 2 of Day06 of Advent of Code 2024.
#[derive(Parser, Debug)]
#[command(version, about, long_about, disable_help_subcommand = true)]
struct Args {
        /// sigh
        #[command(subcommand)]
        action: SubCom,
}

#[derive(Debug, Clone, Subcommand)]
enum SubCom {
        Default,
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
struct Xpos(usize);
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
struct Ypos(usize);

fn main() -> Result<()> {
        let args = Args::parse();
        let _writer_guard = active_global_default_tracing_subscriber()?;

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
struct Position {
        x: Xpos,
        y: Ypos,
}
impl FromStr for Position {
        type Err = PositionError;

        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                let coords: Vec<&str> = s.trim_matches(['(', ')']).split(',').collect();
                if coords.len() != 2 {
                        return Err(PositionError::Format {
                                source_string: s.to_string(),
                        });
                }
                Ok(Self {
                        x: coords[0].parse().map_err(|_| PositionError::Parse {
                                source_string: coords[0].to_string(),
                        })?,
                        y: coords[1].parse().map_err(|_| PositionError::Parse {
                                source_string: coords[1].to_string(),
                        })?,
                })
        }
}
#[derive(Debug, Display, derive_more::Error)]
pub enum PositionError {
        #[display("Invalid position format: {}", source_string)]
        Format { source_string: String },
        #[display("Parse error: {}", source_string)]
        Parse { source_string: String },
}

#[derive(Debug, PartialEq, Clone, FromStr, ValueEnum, Display)]
enum Direction {
        #[display("^")]
        Up,
        #[display("v")]
        Down,
        #[display("<")]
        Left,
        #[display(">")]
        Right,
}
