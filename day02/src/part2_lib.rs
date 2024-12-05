//! Library code for Part 2 of Day02 of Advent of Code 2024.
//! `bin > part2_bin.rs` will run this code along with content of `input2.txt`

use tracing::{self as tea, Level, instrument};

use crate::{Result,
            parse::{Difference, ReportStatus, parse_input}};

#[instrument(skip_all, ret(level = Level::INFO))]
pub fn process_part2(input: &str) -> Result<u64> {
        tea::trace!(%input);
        let line_reports = parse_input(input)?;
        let safe_lines_count = line_reports
                .iter()
                .map(|line| {
                        let first_derivative: Vec<Difference> = line
                                .windows(2)
                                .map(|w| match w {
                                        &[a, b] => Difference::new(b - a),
                                        _ => unreachable!("windows(2)"),
                                })
                                .collect();
                        first_derivative
                })
                .map(|diff| safety_status_2(diff, None))
                .filter(|x| *x == ReportStatus::Safe)
                .count();
        Ok(safe_lines_count.try_into()?)
}

/// Takes Vectors of Differences and returns a ReactorStatus
#[instrument(skip_all, ret(level = Level::DEBUG))]
fn safety_status_2(diffs: Vec<Difference>, has_skipped: Option<bool>) -> ReportStatus {
        let mut has_skipped = has_skipped.unwrap_or(false);
        let needed_sign = match (diffs.len(), has_skipped) {
                (1, false) | (0, _) => return ReportStatus::Safe,
                (2, false) | (1, true) => {
                        if diffs.iter().any(|&v| (1..=3).contains(&v.abs())) {
                                return ReportStatus::Safe;
                        }
                        return ReportStatus::Unsafe;
                }
                (2, true) => {
                        if (diffs[0].signum() == diffs[1].signum()) && *diffs[0] != 0 {
                                return ReportStatus::Safe;
                        }
                        return ReportStatus::Unsafe;
                }
                _ => {
                        if let [a, b, c, ..] = diffs.as_slice() {
                                tea::trace!(?a, ?b, ?c, ?diffs, "destructured values");
                                common_sign([*a, *b, *c])
                        } else {
                                tea::error!(?diffs, "Guaranteed destructuring failed. Logic error.");
                                unreachable!("triple destructuring should always be valid");
                        }
                }
        };
        let Some(needed_sign) = needed_sign else {
                tea::info!("no needed sign");
                return ReportStatus::Unsafe;
        };
        tea::warn!(?needed_sign);

        for (i, diff) in diffs.iter().enumerate() {
                let is_out_of_magnitude = !(1..=3).contains(&diff.abs());
                let is_sign_change = diff.signum() != needed_sign.signum();
                tea::debug!(is_out_of_magnitude, is_sign_change);
                if is_out_of_magnitude || is_sign_change {
                        tea::warn!(?i, ?diff, "unsafe");
                        tea::warn!(iless = i-1, i, imore = i+1, before = ?diffs[i - 1], before = ?diffs[i + 1]);
                        return ReportStatus::Unsafe;
                }
        }
        ReportStatus::Safe
}

/// Sign required if sequence maye be valid
#[derive(Debug, Clone, Copy)]
enum NeededSign {
        Pos,
        Neg,
}
impl NeededSign {
        fn signum(&self) -> i64 {
                match self {
                        NeededSign::Pos => 1,
                        NeededSign::Neg => -1,
                }
        }
}
/// Looks at 3 values and returns the majority sign if any.
/// If there is not a majority sign then there is a set of numbers that can be fixed.
/// due to different signs and one zero or multiple zeroes.
///
/// If there are fewer than 3 values
#[instrument(ret(level = Level::DEBUG))]
fn common_sign(three_vals: [Difference; 3]) -> Option<NeededSign> {
        let pos_count = three_vals.iter().filter(|v| v.is_positive()).count();
        let neg_count = three_vals.iter().filter(|v| v.is_negative()).count();
        match (pos_count, neg_count) {
                ((2..=3), _) => Some(NeededSign::Pos),
                (_, (2..=3)) => Some(NeededSign::Neg),
                _ => None,
        }
}

/// Part 2: determine if report is safe by way of its derivative
#[instrument(skip_all, ret(level = Level::DEBUG))]
fn is_safe_2(diffs: Vec<i64>, has_skipped: Option<bool>) -> ReportStatus {
        // WARN: assuming no empty diffs
        let mut has_skipped = has_skipped.unwrap_or(false);

        let first_elem = diffs[0];
        'level: for diff in diffs.clone() {
                tracing::debug!(?first_elem, ?diff);
                let is_out_of_magnitude = !(1..=3).contains(&diff.abs());
                let is_sign_change = (first_elem.is_positive() && diff.is_negative())
                        || (first_elem.is_negative() && diff.is_positive());
                tracing::debug!(is_out_of_magnitude, is_sign_change);
                if is_out_of_magnitude || is_sign_change {
                        if !has_skipped {
                                tracing::warn!(has_skipped, "forking to create skip");
                                has_skipped = true;
                                let diffs_variant1 = diffs.clone();
                                let diffs_variant2 = diffs.clone();
                                continue 'level;
                        }
                        return ReportStatus::Unsafe;
                }
        }
        tea::trace!("safe");
        ReportStatus::Safe
}

// #[cfg(test)]
// mod tests {
//         use indoc::indoc;

//         use super::*;

//         #[test]
//         fn test_process_example() -> Result<()> {
//                 let input = EXAMPLE_INPUT_2
//                 let expected = todo!();
//                 assert_eq!(process_part2(input)?, expected);
//                 Ok(())
//         }

//         // /// Test's expected value to be populated after solution verification.
//         // /// NOTE: `#[ignore]` is set for this test by default.
//         // #[ignore]
//         // #[test]
//         // fn test_process_problem_input() -> Result<()> {
//         //         tracing_subscriber::fmt::init();
//         //         let file_input = include_str!("../input1.txt");
//         //         let expected = todo!();
//         //         assert_eq!(process(file_input)?, expected);
//         //         Ok(())
//         // }
// }
