use crate::util::{text_matches, Span};

/// A "feature", as tracked by this app. Can be a nightly Rust feature, a
/// stabilized API, or anything else that one version of Rust (deliberately)
/// supports while a previous one didn't support it.
#[derive(Copy, Clone, Debug)]
pub struct FeatureData {
    /// Short description to identify the feature
    pub title: &'static str,
    /// Description of the feature, if the title is not descriptive enough
    pub description: Option<&'static str>,
    /// Feature flag name, for things that were previously or are still Rust
    /// nightly features with such a thing (`#![feature(...)]`)
    pub flag: Option<&'static str>,
    /// What kind of feature this is (language or standard library)
    pub kind: FeatureKind,
    /// The Rust version that stabilized this feature (or "nightly" if it's
    /// not stabilized and only available on the nightly channel
    pub version: &'static str,
    /// Implementation PR id (https://github.com/rust-lang/rust/pull/{id})
    ///
    /// Only for small features that were implemented in one PR.
    pub impl_pr_id: Option<u64>,
    /// Stabilization PR id (https://github.com/rust-lang/rust/pull/{id})
    pub stabilization_pr_id: Option<u64>,
    /// Language items (functions, structs, modules) that are part of this
    /// feature (unless this feature is exactly one item and that item is
    /// already used as the title)
    pub items: &'static [&'static str],
}

#[derive(Copy, Clone, Debug)]
pub enum FeatureKind {
    /// A language feature
    Lang,
    /// A standard library (`core` / `std` / ...) feature
    StdLib,
}

impl FeatureData {
    pub fn matches(&self, search_query: &str) -> Option<Match> {
        // TODO: Split on non-alphanumeric characters instead
        let search_terms: Vec<_> = search_query.split_whitespace().collect();

        let mut res = Match::default();
        res.title_spans = text_matches(self.title, &search_terms);
        res.flag_spans = self.flag.map(|f| text_matches(f, &search_terms)).unwrap_or_default();
        res.desc_spans = self.description.map(|d| text_matches(d, &search_terms)).unwrap_or_default();
        res.item_spans = self.items.iter().map(|i| text_matches(i, &search_terms)).collect();

        if !res.title_spans.is_empty()
            || !res.flag_spans.is_empty()
            || !res.desc_spans.is_empty()
            || res.item_spans.iter().any(|s| !s.is_empty())
        {
            Some(res)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Match {
    pub title_spans: Vec<Span>,
    pub flag_spans: Vec<Span>,
    pub desc_spans: Vec<Span>,
    pub item_spans: Vec<Vec<Span>>,
}

include!(concat!(env!("OUT_DIR"), "/features.rs"));
