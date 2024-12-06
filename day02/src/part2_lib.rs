//! Library code for Part 2 of Day02 of Advent of Code 2024.
//! `bin > part2_bin.rs` will run this code along with content of `input2.txt`

use tracing::{self as tea, Level, instrument};

use crate::{Result,
            parse::{Difference, LineReport, ReportStatus, parse_input}};

#[instrument(skip_all, ret(level = Level::INFO))]
pub fn process_part2(raw_input: &str) -> Result<u64> {
        tea::trace!(%raw_input);
        let line_reports = parse_input(raw_input)?;
        process_parsed_2(line_reports)
}
fn process_parsed_2(line_reports: Vec<LineReport>) -> Result<u64> {
        let safe_line_reports = line_reports
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
                .map(safety_status_2)
                .filter(|x| *x == ReportStatus::Safe)
                .count();
        Ok(safe_line_reports.try_into()?)
}

/// Takes Vectors of Differences and returns a ReactorStatus
#[instrument(skip_all, ret(level = Level::DEBUG))]
fn safety_status_2(diffs: Vec<Difference>) -> ReportStatus {
        tea::trace!(?diffs);
        let mut has_skipped = false;
        let needed_sign = match diffs.len() {
                1 | 0 => return ReportStatus::Safe,
                2 => {
                        tea::warn!("just two");
                        let [a, b] = diffs.as_slice() else {
                                unreachable!("slice size known")
                        };
                        // sequence safe without change
                        if (1..=3).contains(&a.abs()) && (1..=3).contains(&b.abs()) && a.signum() == b.signum() {
                                return ReportStatus::Safe;
                        }
                        // sequence safe with deletion
                        if (1..=3).contains(&(*a + *b).abs()) {
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
                tea::info!("no needed sign; not resolvable as safe");
                return ReportStatus::Unsafe;
        };
        tea::info!(?needed_sign);

        'diff_loop: for (i, diff) in diffs.iter().enumerate() {
                let _entered = tea::trace_span!("For difference", ?diff, i).entered();
                match (is_safe_value(diff, needed_sign), has_skipped) {
                        (true, _) => continue,
                        (false, true) => return ReportStatus::Unsafe,
                        (false, false) => {
                                let _enter = tea::warn_span!("Unacceptable Value", ?diff, i).entered();
                                let pre_val = i.checked_sub(1).and_then(|pre_index| diffs.get(pre_index));
                                let post_val = i.checked_add(1).and_then(|post_index| diffs.get(post_index));
                                let is_pre_sum_safe = pre_val.is_none_or(|p| is_safe_value(&(*p + *diff), needed_sign));
                                let is_post_sum_safe =
                                        post_val.is_none_or(|p| is_safe_value(&(*p + *diff), needed_sign));
                                tea::debug!(
                                        ?i,
                                        ?pre_val,
                                        ?diff,
                                        ?post_val,
                                        is_pre_sum_safe,
                                        is_post_sum_safe,
                                        ?has_skipped,
                                        "unacceptable value"
                                );
                                tea::warn!(
                                        ?pre_val,
                                        ?diff,
                                        ?post_val,
                                        ?is_pre_sum_safe,
                                        ?is_post_sum_safe,
                                        "unacceptable value"
                                );

                                if is_pre_sum_safe || is_post_sum_safe {
                                        tea::trace!("skipping");
                                        has_skipped = true;
                                        continue 'diff_loop;
                                }
                                tea::warn!(?diffs, "UNSAFE!!!");
                                return ReportStatus::Unsafe;
                        }
                }
        }
        ReportStatus::Safe
}

/// Safety for an individual value
#[instrument(ret(level = Level::DEBUG))]
fn is_safe_value(diff: &Difference, needed_sign: NeededSign) -> bool {
        let is_out_of_magnitude = !(1..=3).contains(&diff.abs());
        let is_sign_change = diff.signum() != needed_sign.signum();
        !(is_out_of_magnitude || is_sign_change)
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

#[cfg(test)]
mod tests {
        use indoc::indoc;
        use test_log::test;

        use super::*;

        #[test]
        fn test_example_input_2() -> Result<()> {
                let input = indoc!("
                        7 6 4 2 1
                        1 2 7 8 9
                        9 7 6 2 1
                        1 3 2 4 5
                        8 6 4 4 1
                        1 3 6 7 9");
                let expected = 4;
                assert_eq!(process_part2(input)?, expected);
                Ok(())
        }
        #[test]
        fn test_simple_increasing_2() -> Result<()> {
                let input = indoc!("1 2 3 4");
                let expected = 1;
                assert_eq!(process_part2(input)?, expected);
                Ok(())
        }
        #[test]
        fn test_single_outlier() -> Result<()> {
                let input = indoc!("1 2 3 0");
                let expected = 1;
                assert_eq!(process_part2(input)?, expected);
                Ok(())
        }
        #[test]
        fn test_double_outlier() -> Result<()> {
                let input = indoc!("2 2 3 0");
                let expected = 0;
                assert_eq!(process_part2(input)?, expected);
                Ok(())
        }
        #[test]
        fn test_simple_decreasing_2() -> Result<()> {
                let input = indoc!("4 3 2 1");
                let expected = 1;
                assert_eq!(process_part2(input)?, expected);
                Ok(())
        }

        #[test]
        fn test_increasing_too_fast() -> Result<()> {
                let input = indoc!("1 6 7 10 16 17");
                let expected = 0;
                assert_eq!(process_part2(input)?, expected);
                Ok(())
        }

        #[test]
        fn test_empty_diffs() -> Result<()> {
                let input = indoc!("1");
                let expected = 1;
                assert_eq!(process_part2(input)?, expected);
                Ok(())
        }
        #[test]
        fn test_pair() -> Result<()> {
                let input = indoc!("1 100");
                let expected = 1;
                assert_eq!(process_part2(input)?, expected);
                Ok(())
        }
        #[test]
        fn test_triple_flat() -> Result<()> {
                let input = indoc!("1 1 1");
                let expected = 0;
                assert_eq!(process_part2(input)?, expected);
                Ok(())
        }

        use quickcheck_macros::quickcheck;
        use rand::Rng;

        fn generate_good_sequence(len: usize) -> Vec<LineReport> {
                let mut rng = rand::thread_rng();
                let mut out = Vec::with_capacity(len);
                let mut x = rng.gen_range(1..=55);
                for _ in 0..len {
                        out.push(x);
                        x += rng.gen_range(1..=3);
                }
                vec![out.into()]
        }

        #[quickcheck]
        fn test_good_increasing_sequence(len: u8) -> bool {
                let len = len as usize;
                let good_seq = generate_good_sequence(len);
                tea::warn!(?good_seq);
                let val = process_parsed_2(good_seq.clone());
                match val {
                        Ok(1) => {
                                tea::info!(?val, ?good_seq);
                                true
                        }
                        Ok(_) => {
                                tea::info!(?val, ?good_seq);
                                false
                        }
                        Err(_) => {
                                tea::error!(?val, ?good_seq);
                                false
                        }
                }
        }
}
