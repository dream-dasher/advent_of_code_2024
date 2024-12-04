use derive_more::derive::{Deref, DerefMut, From, Into};
use tracing::{instrument, warn};

#[expect(unused)]
use crate::{EXAMPLE_INPUT_1, FINAL_INPUT_1, support::Result};

/// Report (one line) of Rudolf Reactor readings.
#[derive(Debug, Clone, Deref, DerefMut, From, Into)]
pub struct LineReport(Vec<i64>);

/// Parse txt input of spaced postive integers into line-wise reports (vecs)
#[instrument(skip(hay))]
pub fn parse_input1(hay: &str) -> Result<Vec<LineReport>> {
    let mut out = Vec::new();
    for line in hay.lines() {
        let x: Result<Vec<_>> = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().map_err(|e| e.into()))
            .collect();
        out.push(x?.into());
    }
    tracing::info!(?out);
    Ok(out)
}

// /// Example use of regex crate capture for parsing.
// #[instrument]
// pub fn example_parse() -> Result<Vec<[String; 3]>> {
//         const EXAMPLE_PATH_SPLIT_REGEX: &str = r"^(?m)^([^:]+):([0-9]+):(.+)$";
//         let re = Regex::new(EXAMPLE_PATH_SPLIT_REGEX).unwrap();
//         tracing::info!(?re);

//         let hay = indoc!("\
//         path/to/foo:54:Blue Harvest
//         path/to/bar:90:Something, Something, Something, Dark Side
//         path/to/baz:3:It's a Trap!
//         path/topos/babos:36:ZZzzaZZZaaaZalooong!
//         ");
//         info!(?hay);

//         let mut out = Vec::new();
//         {
//                 let _enter = tracing::info_span!("Parsing").entered();
//                 for (i, line) in hay.lines().enumerate() {
//                         let (raw, [path, lineno, line]) = re.captures(line).unwrap().extract();
//                         tracing::info!(path, lineno, line, raw, i);
//                         out.push([path.to_string(), lineno.to_string(), line.to_string()]);
//                 }
//         }
//         tracing::info!(?out);
//         Ok(out)
// }
