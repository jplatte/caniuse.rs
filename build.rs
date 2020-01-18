use std::{
    env,
    error::Error,
    fmt::{self, Debug, Display},
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
};

use quote::{format_ident, quote};
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct FeatureList {
    features: Vec<Feature>,
}

/// A "feature", as tracked by this app. Can be a nightly Rust feature, a
/// stabilized API, or anything else that one version of Rust (deliberately)
/// supports while a previous one didn't support it.
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Feature {
    /// Feature flag name, for things that were previously or are still Rust
    /// nightly features with such a thing (`#![feature(...)]`)
    flag: Option<String>,
    /// What kind of feature this is (language or standard library)
    kind: FeatureKind,
    /// The Rust version that stabilized this feature (or "nightly" if it's
    /// not stabilized and only available on the nightly channel)
    version: String,
    /// Short description to identify the feature
    desc_short: String,
    /// Language items (functions, structs, modules) that are part of this
    /// feature (unless this feature is exactly one item and that item is
    /// already used as desc_short)
    #[serde(default)]
    items: Vec<String>,
    // TODO: Long description (pbbly with markdown)
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

    let tera = Tera::new("templates/*")?;
    let ctx = Context::from_serialize(&feature_list)?;
    fs::write("static/index.html", tera.render("index.html", &ctx)?)?;
    fs::write("static/nightly.html", tera.render("nightly.html", &ctx)?)?;

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("features.rs");
    let mut out = BufWriter::new(File::create(out_path)?);
    write!(out, "{}", generate_features_array(&feature_list.features))?;

    Ok(())
}

fn generate_features_array(features: &[Feature]) -> impl Display {
    let features = features.iter().map(|feature| {
        let flag = match &feature.flag {
            Some(f) => quote! { Some(#f) },
            None => quote! { None },
        };
        let kind = format_ident!("{}", &feature.kind);
        let version = &feature.version;
        let desc_short = &feature.desc_short;
        let items = &feature.items;

        quote! {
            Feature {
                flag: #flag,
                kind: FeatureKind::#kind,
                version: #version,
                desc_short: #desc_short,
                items: &[#(#items),*],
            }
        }
    });

    quote! {
        pub const FEATURES: &[Feature] = &[#(#features),*];
    }
}
