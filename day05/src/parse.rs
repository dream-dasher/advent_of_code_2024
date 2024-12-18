//! Raw-input parsing code for Day05 of Advent of Code 2024.
//!
//! ## Input patterns
//! - Two sections
//!   - Relations
//!     - <less_than>|<greater_than>
//!   - <blankline>
//!   - Sequences
//!     - n,n,n,n,n,n...   <--should be an odd number of values

use std::{collections::{HashMap, HashSet},
          sync::OnceLock};

use derive_more::derive::{Constructor, Deref, DerefMut, Display, From, FromStr, Into, IntoIterator};
use tracing::{self as tea, Level, instrument};

use crate::{Result, support::ErrKindDay05};

pub static PAGE_RELATIONS: OnceLock<PageRelations> = OnceLock::new();

/// A single page number.
#[derive(
        Debug,
        Clone,
        Constructor,
        PartialEq,
        Eq,
        From,
        Into,
        Deref,
        DerefMut,
        Copy,
        FromStr,
        Hash,
        Display,
)]
#[display("p_{}", _0)]
pub struct Page(u32);
impl PartialOrd for Page {
        /// This assumes that all elements encountered were represented in the rules.
        #[instrument]
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for Page {
        /// This assumes that all elements encountered were represented in the rules.
        #[instrument]
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                tea::debug!(%self, %other);
                tea::debug!(?PAGE_RELATIONS);
                tea::debug!(get_got = ?PAGE_RELATIONS.get());

                match PAGE_RELATIONS
                        .get()
                        .expect("static should have been set")
                        .say_pair_are_ordered((*self, *other))
                {
                        Some(true) => std::cmp::Ordering::Less,
                        Some(false) => std::cmp::Ordering::Greater,
                        None => std::cmp::Ordering::Equal,
                }
        }
}
/// A relationship between two page numbers.
#[derive(Debug, Clone, Constructor, From, PartialEq, Eq, Into, Display)]
#[display("{} < {}", less, more)]
pub struct PageRelation {
        less: Page,
        more: Page,
}
/// A sequence of page numbers, which may or may not be in the correct order.
#[derive(Debug, Clone, Constructor, From, PartialEq, Eq, Into, IntoIterator, Deref, DerefMut)]
pub struct PageSequence {
        sequence: Vec<Page>,
}
/// Set of Page numbers and their lesser and greater pages.
#[derive(Debug, Clone, Constructor, From, Into, IntoIterator, Deref, DerefMut)]
pub struct PageRelations {
        page_relations: HashMap<Page, RelatedPages>,
}
impl PageRelations {
        /// Checks that both elements are represented in the rules and if so that their order matches the rules.
        /// Note: this assumes that the rules are self-consistent
        #[instrument]
        pub fn say_pair_are_ordered(&self, (less, more): (Page, Page)) -> Option<bool> {
                let _ = self.get(&more)?;
                self.get(&less).map(|rp| rp.greater_pages.contains(&more))
        }

        #[instrument()]
        pub fn verify_total_ordering_shape(&self) -> Result<()> {
                tea::trace!(?self);
                for (page, rels) in self.iter() {
                        tea::trace!(%page, ?rels);
                        if rels.total_size() != self.len() - 1 {
                                tea::warn!(%page, ?rels, total_size=rels.total_size(), len_minus_one=self.len()-1, "FALSE; not the shape of a total ordering");
                                Err(ErrKindDay05::NonTotalOrderingShape)?
                        }
                }
                tea::trace!(num_elements = self.len(), "Total Order consistent chape found");
                Ok(())
        }
}
/// Set of pages less than and more than a number.
#[derive(Debug, Clone, Constructor, From, Into, Default)]
pub struct RelatedPages {
        lesser_pages:  HashSet<Page>,
        greater_pages: HashSet<Page>,
}
impl RelatedPages {
        #[instrument]
        pub fn lessers_size(&self) -> usize { self.lesser_pages.len() }

        #[instrument]
        pub fn greaters_size(&self) -> usize { self.greater_pages.len() }

        #[instrument]
        pub fn total_size(&self) -> usize { self.lesser_pages.len() + self.greater_pages.len() }
}

/// Parse txt input ...
#[instrument(skip_all, ret(level = Level::TRACE))]
pub fn parse_input(raw_input: &str) -> Result<(PageRelations, Vec<PageSequence>)> {
        let mut to_check: Vec<PageSequence> = Vec::new();
        let mut page_rels = PageRelations::new(HashMap::new());

        let (order_relations, sequences) = raw_input
                .split_once("\n\n")
                .expect("blank line to split input on");
        for line in order_relations.lines() {
                let (less, more) =
                        line.split_once('|')
                                .ok_or_else(|| ErrKindDay05::OrderPatternError {
                                        source_input: line.to_string(),
                                })?;
                let rule = PageRelation::new(less.parse::<Page>()?, more.parse::<Page>()?);
                tea::trace!(%rule);
                {
                        let _tea = tea::debug_span!("Inserting Local Rules", %rule).entered();
                        tea::debug!(
                                "CAVEAT: this assumes all local relations are described; this would need to a single loop over each HashSet it inserts to in order to describe transitive relations."
                        );
                        page_rels
                                .entry(rule.less)
                                .or_default()
                                .greater_pages
                                .insert(rule.more);
                        page_rels
                                .entry(rule.more)
                                .or_default()
                                .lesser_pages
                                .insert(rule.less);
                }
        }
        // for (key, val) in page_rels.iter() {
        //         tea::info!(%key, lt=?val.lesser_pages, rt=?val.greater_pages);
        // }
        for line in sequences.lines() {
                let sequence: PageSequence = line
                        .split_terminator(',')
                        .map(|n| n.parse::<Page>())
                        .collect::<std::result::Result<Vec<_>, _>>()?
                        .into();
                tea::trace!(?sequence);
                to_check.push(sequence);
        }

        tea::warn!(is_maybe_total_ordering = page_rels.verify_total_ordering_shape().is_ok());
        Ok((page_rels, to_check))
}

#[cfg(test)]
mod tests {
        use indoc::indoc;
        use test_log::test;
        use tracing::{self as tea, instrument};

        use super::*;

        #[test]
        #[instrument]
        fn spot_test() -> Result<()> {
                let input = indoc!("
                        1|2
                        2|3
                        1|4

                        1,2,3
                        3,2,1");
                // let expected = 2;
                let (relations, sequences) = parse_input(input)?;
                tea::trace!(?relations, ?sequences);
                let mut relations = relations.keys().map(|k| (*k).into()).collect::<Vec<u32>>();
                relations.sort_unstable();
                assert_eq!(vec!(1, 2, 3, 4), relations);
                Ok(())
        }
}
