use crate::search::{get_text_matches, Span};

/// A "feature", as tracked by this app. Can be a nightly Rust feature, a
/// stabilized API, or anything else that one version of Rust (deliberately)
/// supports while a previous one didn't support it.
#[derive(Copy, Clone, Debug)]
pub struct FeatureData {
    /// Short description to identify the feature
    pub title: &'static str,
    /// Feature flag name, for things that were previously or are still Rust
    /// nightly features with such a thing (`#![feature(...)]`)
    pub flag: Option<&'static str>,
    /// Feature slug, used for the permalink. If a feature flag exists, this
    /// can be omitted, then the flag is used for the permalink.
    pub slug: &'static str,
    /// The channel (stable / beta / nightly)
    pub channel: Channel,
    /// The Rust version that stabilized this feature (None for unstable
    /// nightly-only features)
    pub version: Option<&'static str>,
    /// RFC id (https://github.com/rust-lang/rfcs/pull/{id})
    pub rfc_id: Option<u64>,
    /// Implementation PR id (https://github.com/rust-lang/rust/pull/{id})
    ///
    /// Only for small features that were implemented in one PR.
    pub impl_pr_id: Option<u64>,
    /// Tracking issue id (https://github.com/rust-lang/rust/issues/{id})
    pub tracking_issue_id: Option<u64>,
    /// Stabilization PR id (https://github.com/rust-lang/rust/pull/{id})
    pub stabilization_pr_id: Option<u64>,
    /// Edition guide path (https://doc.rust-lang.org/edition-guide/{path})
    edition_guide_path: Option<&'static str>,
    /// Language items (functions, structs, modules) that are part of this
    /// feature (unless this feature is exactly one item and that item is
    /// already used as the title)
    pub items: &'static [&'static str],
}

#[derive(Copy, Clone, Debug)]
pub enum Channel {
    Nightly,
    Beta,
    Stable,
}

impl FeatureData {
    pub fn does_match(&self, search_terms: &[impl AsRef<str>]) -> bool {
        for term in search_terms {
            let term = term.as_ref();
            if !self.title.contains(term)
                && !self.flag.map(|f| f.contains(term)).unwrap_or(false)
                && !self.items.iter().any(|i| i.contains(term))
            {
                return false;
            }
        }

        true
    }

    pub fn get_matches(&self, search_terms: &[impl AsRef<str>]) -> Match {
        Match {
            title_spans: get_text_matches(self.title, &search_terms),
            flag_spans: self.flag.map(|f| get_text_matches(f, &search_terms)).unwrap_or_default(),
            item_spans: self.items.iter().map(|i| get_text_matches(i, &search_terms)).collect(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Match {
    pub title_spans: Vec<Span>,
    pub flag_spans: Vec<Span>,
    pub item_spans: Vec<Vec<Span>>,
}

include!(concat!(env!("OUT_DIR"), "/features.rs"));
