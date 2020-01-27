use std::{
    env,
    error::Error,
    fmt::{self, Debug},
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
};

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct FeatureList {
    unstable: UnstableFeatureList,
    versions: Vec<VersionedFeatureList>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct UnstableFeatureList {
    #[serde(default)]
    features: Vec<Feature>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct VersionedFeatureList {
    /// Rust version number, e.g. "1.0.0"
    number: String,
    #[serde(default)]
    features: Vec<Feature>,
}

/// A "feature", as tracked by this app. Can be a nightly Rust feature, a
/// stabilized API, or anything else that one version of Rust (deliberately)
/// supports while a previous one didn't support it.
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Feature {
    /// Short description to identify the feature
    title: String,
    /// Feature flag name, for things that were previously or are still Rust
    /// nightly features with such a thing (`#![feature(...)]`)
    flag: Option<String>,
    /// Feature slug, used for the permalink. If a feature flag exists, this
    /// can be omitted, then the flag is used for the permalink.
    slug: Option<String>,
    /// What kind of feature this is (language or standard library)
    kind: FeatureKind,
    /// Implementation PR id (https://github.com/rust-lang/rust/pull/{id})
    ///
    /// Only for small features that were implemented in one PR.
    impl_pr_id: Option<u64>,
    /// Tracking issue id (https://github.com/rust-lang/rust/issues/{id})
    tracking_issue_id: Option<u64>,
    /// Stabilization PR id (https://github.com/rust-lang/rust/pull/{id})
    stabilization_pr_id: Option<u64>,
    /// Language items (functions, structs, modules) that are part of this
    /// feature (unless this feature is exactly one item and that item is
    /// already used as the title)
    #[serde(default)]
    items: Vec<String>,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
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

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=features.toml");
    println!("cargo:rerun-if-changed=templates/index.html");
    println!("cargo:rerun-if-changed=templates/nightly.html");
    println!("cargo:rerun-if-changed=templates/skel.html");

    let features_raw = fs::read("features.toml")?;
    let feature_list: FeatureList = toml::from_slice(&features_raw)?;

    // TODO: Add a filter that replaces `` by <code></code>
    let tera = Tera::new("templates/*")?;
    let ctx = Context::from_serialize(&feature_list)?;
    fs::write("static/index.html", tera.render("index.html", &ctx)?)?;
    fs::write("static/nightly.html", tera.render("nightly.html", &ctx)?)?;

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("features.rs");
    let mut out = BufWriter::new(File::create(out_path)?);

    let all_features = feature_list.unstable.features.iter().map(|f| ("nightly", f)).chain(
        feature_list.versions.iter().flat_map(|version| {
            let number: &str = &version.number;
            version.features.iter().map(move |f| (number, f))
        }),
    );
    write!(out, "{}", generate_features_array(all_features))?;

    Ok(())
}

fn generate_features_array<'a>(
    features: impl Iterator<Item = (&'a str, &'a Feature)>,
) -> TokenStream {
    let features = features.map(|(version, feature)| {
        assert!(
            !feature.items.iter().any(|i| i.contains('`')),
            "items are always wrapped in code blocks and should not contain any '`'.",
        );

        let title = &feature.title;
        let flag = option_literal(&feature.flag);
        let slug = option_literal(&feature.slug);
        let kind = format_ident!("{}", &feature.kind);
        let impl_pr_id = option_literal(&feature.impl_pr_id);
        let tracking_issue_id = option_literal(&feature.tracking_issue_id);
        let stabilization_pr_id = option_literal(&feature.stabilization_pr_id);
        let items = &feature.items;

        quote! {
            FeatureData {
                title: #title,
                flag: #flag,
                kind: FeatureKind::#kind,
                version: #version,
                impl_pr_id: #impl_pr_id,
                tracking_issue_id: #tracking_issue_id,
                stabilization_pr_id: #stabilization_pr_id,
                items: &[#(#items),*],
            }
        }
    });

    quote! {
        pub const FEATURES: &[FeatureData] = &[#(#features),*];
    }
}

fn option_literal<T: ToTokens>(opt: &Option<T>) -> TokenStream {
    match opt {
        Some(lit) => quote! { Some(#lit) },
        None => quote! { None },
    }
}
