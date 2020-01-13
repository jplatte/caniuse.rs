use std::{
    env,
    fmt::{self, Debug, Display},
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
};

use quote::{format_ident, quote};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
struct FeatureFile {
    feature: Vec<Feature>,
}

/// A "feature", as tracked by this app. Can be a nightly Rust feature, a
/// stabilized API, or anything else that one version of Rust (deliberately)
/// supports while a previous one didn't support it.
#[derive(Clone, Debug, Deserialize)]
struct Feature {
    /// Feature flag name, for things that were previously or are still Rust
    /// nightly features with such a thing (`#![feature(...)]`)
    flag: Option<String>,
    /// What kind of feature this is (language or standard library)
    kind: FeatureKind,
    /// The Rust version that stabilized this feature
    stable_since: String,
    /// Short description to identify the feature
    desc_short: String,
    /// Language items (functions, structs, modules) that are part of this
    /// feature (unless this feature is exactly one item and that item is
    /// already used as desc_short)
    #[serde(default)]
    items: Vec<String>,
    // TODO: Long description (pbbly with markdown)
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum FeatureKind {
    /// A language feature
    Lang,
    /// A standard library (`core` / `std` / ...) feature
    StdLib,
}

impl quote::IdentFragment for FeatureKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

fn main() {
    println!("cargo:rerun-if-changed=features.toml");

    let features_raw = fs::read("features.toml").unwrap();
    let feature_file: FeatureFile = toml::from_slice(&features_raw).unwrap();

    // TODO: Also generate static/list.html

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("features.rs");
    let mut out = BufWriter::new(File::create(out_path).unwrap());
    write!(out, "{}", generate_features_array(&feature_file.feature)).unwrap();
}

fn generate_features_array(features: &[Feature]) -> impl Display {
    let features = features.iter().map(|feature| {
        let flag = match &feature.flag {
            Some(f) => quote! { Some(#f) },
            None => quote! { None },
        };
        let kind = format_ident!("{}", &feature.kind);
        let stable_since = &feature.stable_since;
        let desc_short = &feature.desc_short;
        let items = &feature.items;

        quote! {
            Feature {
                flag: #flag,
                kind: FeatureKind::#kind,
                stable_since: #stable_since,
                desc_short: #desc_short,
                items: &[#(#items),*],
            }
        }
    });

    quote! {
        pub const FEATURES: &[Feature] = &[#(#features),*];
    }
}
