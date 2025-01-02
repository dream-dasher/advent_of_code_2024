/*!
0   | * * * *  |
1a  | +        |

1b  |   +      |
2b  | + +      |

1c  |     +    |
2c. | +   +    |
2c: |   + +    |

1d  |       +  |
2d. | +     +  |
2d: |   +   +  |
2d; |     + +  |
**/
use std::fmt::Debug;

use clap::Parser;
use day07::Result;
use dirty_terminal::{clear_screen_ansi, dirty_pause};
use tracing::{Level, event, instrument};

const LEN: usize = 5;
fn main() -> Result<()> {
        let args = Args::parse();
        tracing_subscriber::fmt::init();
        let b_arr = [Symbol::A; LEN];
        let mut all = Vec::with_capacity(2_usize.pow(LEN as u32));
        all.extend(vec![b_arr]);
        all.extend(boop_array(b_arr, 0, args.manual_mode));
        let solution_len = all.len();
        for i in all {
                println!("{:?}", i);
        }
        println!("all length: {:?}", solution_len);
        println!("2^{} {}", LEN, 2_usize.pow(LEN as u32));
        Ok(())
}

#[instrument(ret(level = Level::TRACE))]
fn boop_array<const N: usize>(arr: [Symbol; N], idx: usize, manual_mode: bool) -> Vec<[Symbol; N]> {
        let mut out: Vec<[Symbol; N]> = Vec::new();
        // out.push(arr);
        let to_do = idx..N;
        for i in to_do {
                let mut arr_alt = arr;
                arr_alt[i] = Symbol::B;
                if manual_mode {
                        event![Level::INFO, ?arr_alt, idx, i, "update"];
                        dirty_pause().unwrap();
                        clear_screen_ansi();
                }
                out.push(arr_alt);
                if i + 1 < N {
                        out.extend(boop_array(arr_alt, i + 1, manual_mode));
                }
        }
        out
}

#[derive(Clone, Copy, derive_more::Display)]
enum Symbol {
        #[display("_")]
        A,
        #[display("X")]
        B,
}
impl Debug for Symbol {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self)
        }
}

/// Checking our combinatorial value generation.
#[derive(Parser, Debug)]
#[command(version, about, long_about)]
pub struct Args {
        /// Whether each step should be paused. (For review with Tracing @ INFO level.)
        #[arg(long, short, value_enum)]
        manual_mode: bool,
}

mod dirty_terminal {
        use std::{io, io::Write as _};

        use super::*;
        /// Clear terminal screen using ANSI escape code.
        ///
        /// Not the most robust, but decent in a pinch.
        pub fn clear_screen_ansi() {
                // There are ANSI escape codes that can be used to clear the screen!
                const ANSI_CLEAR_SCREEN: &str = "\x1B[2J\x1B[H";
                print!("{}", ANSI_CLEAR_SCREEN);
                std::io::stdout().flush().unwrap();
        }
        /// Quick and dirty pause button so I can watch as program runs.
        pub fn dirty_pause() -> Result<()> {
                println!("Press Enter to continue...");
                let mut _input = String::new();
                let read_in = io::stdin().read_line(&mut _input)?;
                event![Level::DEBUG, ?read_in];
                Ok(())
        }
}
