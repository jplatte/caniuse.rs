use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct FeatureToml {
    pub versions: Vec<FeatureList>,
    pub unstable: FeatureList,
}

impl FeatureToml {
    pub fn features<'a>(
        &'a self,
    ) -> impl Iterator<Item = (&'a Option<VersionData>, &'a FeatureData)> {
        self.versions.iter().flat_map(|v| v.features.iter().map(move |f| (&v.version, f)))
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct VersionData {
    /// Rust version number, e.g. "1.0.0"
    pub number: String,
    /// The channel (stable / beta / nightly)
    #[serde(default)]
    pub channel: Channel,
    /// Blog post path (https://blog.rust-lang.org/{path})
    pub blog_post_path: Option<String>,
    /// GitHub milestone id (https://github.com/rust-lang/rust/milestone/{id})
    pub gh_milestone_id: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FeatureList {
    #[serde(flatten)]
    pub version: Option<VersionData>,
    /// List of features (to be) stabilized in this release
    #[serde(default)]
    pub features: Vec<FeatureData>,
}

/// A "feature", as tracked by this app. Can be a nightly Rust feature, a
/// stabilized API, or anything else that one version of Rust (deliberately)
/// supports while a previous one didn't support it.
#[derive(Clone, Debug, Deserialize)]
pub struct FeatureData {
    /// Short description to identify the feature
    pub title: String,
    /// Feature flag name, for things that were previously or are still Rust
    /// nightly features with such a thing (`#![feature(...)]`)
    pub flag: Option<String>,
    /// Feature slug, used for the permalink. If a feature flag exists, this
    /// can be omitted, then the flag is used for the permalink.
    pub slug: Option<String>,
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
    pub doc_path: Option<String>,
    /// Edition guide path (https://doc.rust-lang.org/edition-guide/{path})
    pub edition_guide_path: Option<String>,
    /// Unstable book path (https://doc.rust-lang.org/unstable-book/{path})
    pub unstable_book_path: Option<String>,
    /// Language items (functions, structs, modules) that are part of this
    /// feature (unless this feature is exactly one item and that item is
    /// already used as the title)
    #[serde(default)]
    pub items: Vec<String>,
}

impl FeatureData {
    pub fn slug(&self) -> &str {
        self.flag.as_ref().or(self.slug.as_ref()).unwrap()
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Channel {
    Stable,
    Beta,
    Nightly,
}

/// Not specifying the channel in features.toml is equivalent to specifying
/// "stable"
impl Default for Channel {
    fn default() -> Self {
        Self::Stable
    }
}
