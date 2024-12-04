use indoc::indoc;
use regex::Regex;
use tracing::instrument;

#[expect(unused)]
use crate::{EXAMPLE_INPUT_2, FINAL_INPUT_2, support::Result};

/// Copy of parse_input1
#[instrument(skip(hay))]
pub fn parse_input2(hay: &str) -> Result<(Vec<u64>, Vec<u64>)> {
        const SPLIT_REGEX: &str = r"^(\d+).*?(\d+)$";
        let re = Regex::new(SPLIT_REGEX).unwrap();
        tracing::debug!(?re);
        let mut left_vec: Vec<_> = Vec::new();
        let mut right_vec: Vec<_> = Vec::new();
        let _enter = tracing::info_span!("Parsing").entered();
        for (i, line) in hay.lines().enumerate() {
                let (raw, [left, right]) = re.captures(line).unwrap().extract();
                tracing::trace!(i, left, right);
                tracing::trace!(raw);
                left_vec.push(left.parse()?);
                right_vec.push(right.parse()?);
        }
        tracing::trace!(?left_vec, ?right_vec);
        tracing::debug!(
                hay_line_count = hay.lines().count(),
                left_len = left_vec.len(),
                right_len = right_vec.len()
        );
        Ok((left_vec, right_vec))
}

// #[instrument(skip(hay))]
// fn parse_input2(hay: &str) -> Result<Vec<_>> {
//         const _REGEX: &str = r"^$";
//         let re = Regex::new(_REGEX).unwrap();
//         info!(?re);
//         let mut out = Vec::new();
//         let _enter = tracing::info_span!("Parsing").entered();
//         for (i, line) in hay.lines().enumerate() {
//                 let (x, [a, b, c, d]) = re.captures(line).unwrap().extract();
//                 info!(?a,?b,?c,?d);
//                 out.push((a.to_string(),b.to_string(),c.to_string(),d.to_string()));
//         }
//         Ok(out)
// }

/// Example use of regex crate capture for parsing.
#[instrument]
pub fn example_parse() -> Result<Vec<[String; 3]>> {
        const EXAMPLE_PATH_SPLIT_REGEX: &str = r"^(?m)^([^:]+):([0-9]+):(.+)$";
        let re = Regex::new(EXAMPLE_PATH_SPLIT_REGEX).unwrap();
        tracing::info!(?re);

        let hay = indoc!("\
        path/to/foo:54:Blue Harvest
        path/to/bar:90:Something, Something, Something, Dark Side
        path/to/baz:3:It's a Trap!
        path/topos/babos:36:ZZzzaZZZaaaZalooong!
        ");
        tracing::info!(?hay);

        let mut out = Vec::new();
        {
                let _enter = tracing::info_span!("Parsing").entered();
                for (i, line) in hay.lines().enumerate() {
                        let (raw, [path, lineno, line]) = re.captures(line).unwrap().extract();
                        tracing::info!(path, lineno, line, raw, i);
                        out.push([path.to_string(), lineno.to_string(), line.to_string()]);
                }
        }
        tracing::info!(?out);
        Ok(out)
}
