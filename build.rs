use std::{
    collections::{BTreeMap, BTreeSet},
    convert::TryInto,
    default::Default,
    env,
    error::Error,
    fmt::Debug,
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
};

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
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
    /// The channel (stable / beta / nightly)
    #[serde(default)]
    channel: Channel,
    /// Blog post path (https://blog.rust-lang.org/{path})
    blog_post_path: Option<String>,
    /// List of features (to be) stabilized in this release
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
    /// RFC id (https://github.com/rust-lang/rfcs/pull/{id})
    rfc_id: Option<u64>,
    /// Implementation PR id (https://github.com/rust-lang/rust/pull/{id})
    ///
    /// Only for small features that were implemented in one PR.
    impl_pr_id: Option<u64>,
    /// Tracking issue id (https://github.com/rust-lang/rust/issues/{id})
    tracking_issue_id: Option<u64>,
    /// Stabilization PR id (https://github.com/rust-lang/rust/pull/{id})
    stabilization_pr_id: Option<u64>,
    /// Documentation path (https://doc.rust-lang.org/{path})
    doc_path: Option<String>,
    /// Edition guide path (https://doc.rust-lang.org/edition-guide/{path})
    edition_guide_path: Option<String>,
    /// Unstable book path (https://doc.rust-lang.org/unstable-book/{path})
    unstable_book_path: Option<String>,
    /// Language items (functions, structs, modules) that are part of this
    /// feature (unless this feature is exactly one item and that item is
    /// already used as the title)
    #[serde(default)]
    items: Vec<String>,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum Channel {
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
    fs::write("public/index.html", tera.render("index.html", &ctx)?)?;
    fs::write("public/nightly.html", tera.render("nightly.html", &ctx)?)?;

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("features.rs");
    let mut out = BufWriter::new(File::create(out_path)?);

    write!(out, "{}", generate_versions(&feature_list.versions))?;

    let all_features = feature_list
        .versions
        .iter()
        .flat_map(|version| {
            let number: Option<&str> = Some(&version.number);
            version.features.iter().map(move |f| (version.channel, number, f))
        })
        .chain(feature_list.unstable.features.iter().map(|f| (Channel::Nightly, None, f)));
    write!(out, "{}", generate_features(all_features))?;

    Ok(())
}

fn generate_versions(versions: &[VersionedFeatureList]) -> TokenStream {
    let versions = versions.iter().map(|version| {
        let number = &version.number;
        let channel = Ident::new(&format!("{:?}", version.channel), Span::call_site());
        let blog_post_path = option_literal(&version.blog_post_path);

        quote! {
            VersionData {
                number: #number,
                channel: Channel::#channel,
                blog_post_path: #blog_post_path,
            }
        }
    });

    quote! {
        pub const VERSIONS: &[VersionData] = &[#(#versions),*];
    }
}

fn generate_features<'a>(
    features: impl Iterator<Item = (Channel, Option<&'a str>, &'a Feature)>,
) -> TokenStream {
    let mut monogram_index = BTreeMap::new();
    let mut bigram_index = BTreeMap::new();
    let mut trigram_index = BTreeMap::new();

    let features = features.enumerate().map(|(idx, (channel, version, feature))| {
        assert!(
            !feature.items.iter().any(|i| i.contains('`')),
            "items are always wrapped in code blocks and should not contain any '`'.",
        );

        let idx = idx.try_into().expect("At most 65536 features");

        add_feature_ngrams(1, &mut monogram_index, feature, idx);
        add_feature_ngrams(2, &mut bigram_index, feature, idx);
        add_feature_ngrams(3, &mut trigram_index, feature, idx);

        let title = &feature.title;
        let flag = option_literal(&feature.flag);
        let slug = feature
            .slug
            .as_ref()
            .or_else(|| feature.flag.as_ref())
            .unwrap_or_else(|| panic!("feature '{}' needs a feature flag or slug", title));
        let channel = Ident::new(&format!("{:?}", channel), Span::call_site());
        let version = option_literal(&version);
        let rfc_id = option_literal(&feature.rfc_id);
        let impl_pr_id = option_literal(&feature.impl_pr_id);
        let tracking_issue_id = option_literal(&feature.tracking_issue_id);
        let stabilization_pr_id = option_literal(&feature.stabilization_pr_id);
        let doc_path = option_literal(&feature.doc_path);
        let edition_guide_path = option_literal(&feature.edition_guide_path);
        let unstable_book_path = option_literal(&feature.unstable_book_path);
        let items = &feature.items;

        quote! {
            FeatureData {
                title: #title,
                flag: #flag,
                slug: #slug,
                channel: Channel::#channel,
                version: #version,
                rfc_id: #rfc_id,
                impl_pr_id: #impl_pr_id,
                tracking_issue_id: #tracking_issue_id,
                stabilization_pr_id: #stabilization_pr_id,
                doc_path: #doc_path,
                edition_guide_path: #edition_guide_path,
                unstable_book_path: #unstable_book_path,
                items: &[#(#items),*],
            }
        }
    });

    let features = quote! {
        #[allow(clippy::unreadable_literal)]
        pub const FEATURES: &[FeatureData] = &[#(#features),*];
    };

    let monogram_index_insert_stmts = monogram_index.into_iter().map(|(k, v)| {
        let byte = k[0];
        quote! {
            index.insert(#byte, &[#(#v),*] as &[u16]);
        }
    });

    let monogram_feature_index = quote! {
        pub const FEATURE_MONOGRAM_INDEX: once_cell::sync::Lazy<std::collections::HashMap<u8, &[u16]>> =
            once_cell::sync::Lazy::new(|| {
                let mut index = std::collections::HashMap::new();
                #(#monogram_index_insert_stmts)*
                index
            });
    };

    let bigram_index_insert_stmts = bigram_index.into_iter().map(|(k, v)| {
        let [b1, b2] = match &k[..] {
            &[b1, b2] => [b1, b2],
            _ => unreachable!(),
        };

        quote! {
            index.insert([#b1, #b2], &[#(#v),*] as &[u16]);
        }
    });

    let bigram_feature_index = quote! {
        pub const FEATURE_BIGRAM_INDEX: once_cell::sync::Lazy<std::collections::HashMap<[u8; 2], &[u16]>> =
            once_cell::sync::Lazy::new(|| {
                let mut index = std::collections::HashMap::new();
                #(#bigram_index_insert_stmts)*
                index
            });
    };

    let trigram_index_insert_stmts = trigram_index.into_iter().map(|(k, v)| {
        let [b1, b2, b3] = match &k[..] {
            &[b1, b2, b3] => [b1, b2, b3],
            _ => unreachable!(),
        };

        quote! {
            index.insert([#b1, #b2, #b3], &[#(#v),*] as &[u16]);
        }
    });

    let trigram_feature_index = quote! {
        pub const FEATURE_TRIGRAM_INDEX: once_cell::sync::Lazy<std::collections::HashMap<[u8; 3], &[u16]>> =
            once_cell::sync::Lazy::new(|| {
                let mut index = std::collections::HashMap::new();
                #(#trigram_index_insert_stmts)*
                index
            });
    };

    quote! {
        #features
        #monogram_feature_index
        #bigram_feature_index
        #trigram_feature_index
    }
}

fn option_literal<T: ToTokens>(opt: &Option<T>) -> TokenStream {
    match opt {
        Some(lit) => quote! { Some(#lit) },
        None => quote! { None },
    }
}

fn add_feature_ngrams(
    n: usize,
    index: &mut BTreeMap<Vec<u8>, BTreeSet<u16>>,
    feature: &Feature,
    idx: u16,
) {
    let mut strings = vec![&feature.title];
    if let Some(f) = &feature.flag {
        strings.push(f);
    }
    strings.extend(feature.items.iter());

    for string in strings {
        for trigram in string.as_bytes().windows(n) {
            if trigram.iter().all(|&byte| byte.is_ascii_graphic() && byte != b'`') {
                index.entry(trigram.to_owned()).or_default().insert(idx);
            }
        }
    }
}
