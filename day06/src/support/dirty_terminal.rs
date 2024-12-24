//! Dirty terminal code, for cursory exploration

use std::{io, io::Write as _};

use tracing::{Level, event, instrument};

use crate::Result;

/// Clear terminal screen using ANSI escape code.
///
/// Not the most robust, but decent in a pinch.
#[instrument]

pub fn clear_screen_ansi() {
        // There are ANSI escape codes that can be used to clear the screen!
        const ANSI_CLEAR_SCREEN: &str = "\x1B[2J\x1B[H";
        print!("{}", ANSI_CLEAR_SCREEN);
        std::io::stdout().flush().unwrap();
}

/// Quick and dirty pause button so I can watch as program runs.
#[instrument]
pub fn dirty_pause() -> Result<()> {
        println!("Press Enter to continue...");
        let mut _input = String::new();
        let read_in = io::stdin().read_line(&mut _input)?;
        event![Level::DEBUG, ?read_in];
        Ok(())
}
