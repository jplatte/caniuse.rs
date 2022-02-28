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
    /// Feature slug, used for the permalink.
    pub slug: &'static str,
    pub version: Option<&'static VersionData>,
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
    /// Documentation path (https://doc.rust-lang.org/{path})
    pub doc_path: Option<&'static str>,
    /// Edition guide path (https://doc.rust-lang.org/edition-guide/{path})
    pub edition_guide_path: Option<&'static str>,
    /// Unstable book path (https://doc.rust-lang.org/unstable-book/{path})
    pub unstable_book_path: Option<&'static str>,
    /// Language items (functions, structs, modules) that are part of this
    /// feature (unless this feature is exactly one item and that item is
    /// already used as the title)
    pub items: &'static [&'static str],
}

impl FeatureData {
    pub fn is_on_channel(&self, chan: Channel) -> bool {
        self.version.map(|v| v.channel == chan).unwrap_or(false)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct VersionData {
    /// The version number, without the patch component (e.g. "1.31")
    pub number: &'static str,
    /// The channel (stable / beta / nightly)
    pub channel: Channel,
    /// Release date, in format "yyyy-mm-dd"
    pub release_date: Option<&'static str>,
    /// Release notes (https://github.com/rust-lang/rust/blob/master/RELEASES.md#{anchor})
    pub release_notes: Option<&'static str>,
    /// GitHub milestone id (https://github.com/rust-lang/rust/milestone/{id})
    pub gh_milestone_id: Option<u64>,
    /// Blog post path (https://blog.rust-lang.org/{path})
    pub blog_post_path: Option<&'static str>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Channel {
    Nightly,
    Beta,
    Stable,
}

include!(concat!(env!("OUT_DIR"), "/features.rs"));
