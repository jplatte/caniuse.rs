/// A "feature", as tracked by this app. Can be a nightly Rust feature, a
/// stabilized API, or anything else that one version of Rust (deliberately)
/// supports while a previous one didn't support it.
#[derive(Clone, Debug)]
pub struct Feature {
    /// Feature flag name, for things that were previously or are still Rust
    /// nightly features with such a thing (`#![feature(...)]`)
    pub flag: Option<&'static str>,
    /// What kind of feature this is (language or standard library)
    pub kind: FeatureKind,
    /// The Rust version that stabilized this feature
    pub stable_since: &'static str,
    /// Short description to identify the feature
    pub desc_short: &'static str,
    /// Language items (functions, structs, modules) that are part of this
    /// feature (unless this feature is exactly one item and that item is
    /// already used as desc_short)
    pub items: &'static [&'static str],
    // TODO: Long description (pbbly with markdown)
}

#[derive(Copy, Clone, Debug)]
pub enum FeatureKind {
    /// A language feature
    Lang,
    /// A standard library (`core` / `std` / ...) feature
    StdLib,
}

impl Feature {
    pub fn matches(&self, search_term: &str) -> bool {
        // TODO(1): match on flag, items
        // TODO(2): return Option<Span>, fuzzy matching
        self.desc_short.find(search_term).is_some()
    }
}

include!(concat!(env!("OUT_DIR"), "/features.rs"));
