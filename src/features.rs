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
    /// The Rust version that stabilized this feature (or "nightly" if it's
    /// not stabilized and only available on the nightly channel
    pub version: &'static str,
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
    pub fn matches(&self, search_term: &str) -> Option<Match> {
        // TODO: fuzzy matching
        let len = search_term.len();

        let mut res = Match::default();
        res.flag_span = self.flag.and_then(|f| f.find(search_term).map(|start| span(start, len)));
        res.desc_span = self.desc_short.find(search_term).map(|start| span(start, len));
        res.item_spans =
            self.items.iter().map(|i| i.find(search_term).map(|start| span(start, len))).collect();

        if res.flag_span.is_some()
            || res.desc_span.is_some()
            || res.item_spans.iter().any(|s| s.is_some())
        {
            Some(res)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Match {
    pub flag_span: Option<Span>,
    pub desc_span: Option<Span>,
    pub item_spans: Vec<Option<Span>>,
}

pub type Span = std::ops::Range<usize>;

fn span(start: usize, len: usize) -> Span {
    Span { start, end: start + len }
}

include!(concat!(env!("OUT_DIR"), "/features.rs"));
