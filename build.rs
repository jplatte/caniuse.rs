#![feature(array_windows)]

use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet},
    default::Default,
    env,
    fmt::Debug,
    io::{BufWriter, Write},
    iter,
    path::Path,
};

use anyhow::Context as _;
use fs_err::{self as fs, DirEntry, File};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tera::{Context, Tera};

#[derive(Serialize)]
struct Data {
    versions: Vec<FeatureList>,
    unstable: FeatureList,
}

#[derive(Deserialize, Serialize)]
struct VersionData {
    /// Rust version number, e.g. "1.0.0"
    number: String,
    /// The channel (stable / beta / nightly)
    #[serde(default)]
    channel: Channel,
    /// Release date, in format "yyyy-mm-dd"
    #[serde(skip_serializing_if = "Option::is_none")]
    release_date: Option<String>,
    /// Release notes (https://github.com/rust-lang/rust/blob/master/RELEASES.md#{anchor})
    #[serde(skip_serializing_if = "Option::is_none")]
    release_notes: Option<String>,
    /// Blog post path (https://blog.rust-lang.org/{path})
    #[serde(skip_serializing_if = "Option::is_none")]
    blog_post_path: Option<String>,
    /// GitHub milestone id (https://github.com/rust-lang/rust/milestone/{id})
    #[serde(skip_serializing_if = "Option::is_none")]
    gh_milestone_id: Option<u64>,
}

#[derive(Serialize)]
struct FeatureList {
    version: Option<VersionData>,
    /// List of features (to be) stabilized in this release
    features: Vec<FeatureData>,
}

/// A "feature", as tracked by this app. Can be a nightly Rust feature, a
/// stabilized API, or anything else that one version of Rust (deliberately)
/// supports while a previous one didn't support it.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct FeatureData {
    /// Short description to identify the feature
    title: String,
    /// Feature flag name, for things that were previously or are still Rust
    /// nightly features with such a thing (`#![feature(...)]`)
    #[serde(skip_serializing_if = "Option::is_none")]
    flag: Option<String>,
    /// Feature slug, used for the permalink. Filled from filename.
    #[serde(skip_deserializing)]
    slug: String,
    /// RFC id (https://github.com/rust-lang/rfcs/pull/{id})
    #[serde(skip_serializing_if = "Option::is_none")]
    rfc_id: Option<u64>,
    /// Implementation PR id (https://github.com/rust-lang/rust/pull/{id})
    ///
    /// Only for small features that were implemented in one PR.
    #[serde(skip_serializing_if = "Option::is_none")]
    impl_pr_id: Option<u64>,
    /// Tracking issue id (https://github.com/rust-lang/rust/issues/{id})
    #[serde(skip_serializing_if = "Option::is_none")]
    tracking_issue_id: Option<u64>,
    /// Stabilization PR id (https://github.com/rust-lang/rust/pull/{id})
    #[serde(skip_serializing_if = "Option::is_none")]
    stabilization_pr_id: Option<u64>,
    /// Documentation path (https://doc.rust-lang.org/{path})
    #[serde(skip_serializing_if = "Option::is_none")]
    doc_path: Option<String>,
    /// Edition guide path (https://doc.rust-lang.org/edition-guide/{path})
    #[serde(skip_serializing_if = "Option::is_none")]
    edition_guide_path: Option<String>,
    /// Unstable book path (https://doc.rust-lang.org/unstable-book/{path})
    #[serde(skip_serializing_if = "Option::is_none")]
    unstable_book_path: Option<String>,
    /// Language items (functions, structs, modules) that are part of this
    /// feature (unless this feature is exactly one item and that item is
    /// already used as the title)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=data");
    println!("cargo:rerun-if-changed=templates");

    let data = collect_data()?;

    // TODO: Add a filter that replaces `` by <code></code>
    let tera = Tera::new("templates/*").context("loading templates")?;
    fs::create_dir_all("public")?;

    let ctx = Context::from_serialize(&data)?;
    let index_html = BufWriter::new(File::create("public/index.html")?);
    tera.render_to("index.html", &ctx, index_html).context("rendering index.html")?;

    let nightly_html = BufWriter::new(File::create("public/nightly.html")?);
    tera.render_to("nightly.html", &ctx, nightly_html).context("rendering nightly.html")?;

    let (code, json) = generate_output(data);

    let mut features_rs =
        BufWriter::new(File::create(Path::new(&env::var("OUT_DIR").unwrap()).join("features.rs"))?);
    write!(features_rs, "{}", code).context("writing features.rs")?;

    let features_json = BufWriter::new(File::create("public/features.json")?);
    serde_json::to_writer_pretty(features_json, &json)?;

    Ok(())
}

fn collect_data() -> anyhow::Result<Data> {
    let mut data = Data {
        versions: Vec::new(),
        unstable: FeatureList { version: None, features: Vec::new() },
    };

    for entry in fs::read_dir("data")? {
        let dir = entry?;
        assert!(dir.file_type()?.is_dir(), "expected only directories in data/");

        let dir_name = dir.file_name().into_string().unwrap();

        let features = match dir_name.as_str() {
            "unstable" => &mut data.unstable.features,
            _ => {
                let version_data_raw = fs::read(dir.path().join("version.toml"))?;
                let version_data = toml::from_slice(&version_data_raw)?;
                data.versions
                    .push(FeatureList { version: Some(version_data), features: Vec::new() });
                &mut data.versions.last_mut().unwrap().features
            }
        };

        collect_features(dir, &dir_name, features)?;
    }

    data.versions.sort_unstable_by_key(|v| {
        let num_str = &v.version.as_ref().unwrap().number;
        assert!(&num_str[..2] == "1.");
        let num: u16 = num_str[2..].parse().unwrap();
        Reverse(num)
    });

    Ok(data)
}

fn collect_features(
    dir: DirEntry,
    dir_name: &str,
    features: &mut Vec<FeatureData>,
) -> anyhow::Result<()> {
    for entry in fs::read_dir(dir.path())? {
        let file = entry?;
        let file_name = file.file_name().into_string().unwrap();

        if file_name == "version.toml" {
            continue;
        }

        let slug = match file_name.strip_suffix(".toml") {
            Some(basename) => basename.to_owned(),
            None => {
                panic!(
                    "expected only .toml files in data/*, found `{}`",
                    file_name,
                )
            }
        };

        let feature = toml::from_str(&fs::read_to_string(file.path())?)
            .with_context(|| format!("deserializing of data/{}/{}", dir_name, file_name))?;

        features.push(FeatureData { slug, ..feature });
    }

    Ok(())
}

fn generate_output(data: Data) -> (TokenStream, serde_json::Value) {
    let mut json = json!({ "versions": {}, "features": {} });

    let mut monogram_index = BTreeMap::new();
    let mut bigram_index = BTreeMap::new();
    let mut trigram_index = BTreeMap::new();

    let mut versions = Vec::new();
    let mut features = Vec::new();

    let mut feat_idx = 0;

    for v in data.versions.into_iter().chain(iter::once(data.unstable)) {
        let v_idx = v.version.as_ref().map(|d| {
            json["versions"][&d.number] = serde_json::to_value(d).unwrap();

            let number = &d.number;
            let channel = Ident::new(&format!("{:?}", d.channel), Span::call_site());
            let release_date = option_literal(&d.release_date);
            let release_notes = option_literal(&d.release_notes);
            let blog_post_path = option_literal(&d.blog_post_path);
            let gh_milestone_id = option_literal(&d.gh_milestone_id);

            versions.push(quote! {
                VersionData {
                    number: #number,
                    channel: Channel::#channel,
                    release_date: #release_date,
                    release_notes: #release_notes,
                    blog_post_path: #blog_post_path,
                    gh_milestone_id: #gh_milestone_id,
                }
            });

            versions.len() - 1
        });

        for f in v.features {
            assert!(
                !f.items.iter().any(|i| i.contains('`')),
                "items are always wrapped in code blocks and should not contain any '`'.",
            );

            add_feature_ngrams::<1>(&mut monogram_index, &f, feat_idx);
            add_feature_ngrams::<2>(&mut bigram_index, &f, feat_idx);
            add_feature_ngrams::<3>(&mut trigram_index, &f, feat_idx);

            json["features"][&f.slug] = {
                let mut feat_json = serde_json::to_value(&f).unwrap();
                feat_json["version"] =
                    serde_json::to_value(&v.version.as_ref().map(|d| &d.number)).unwrap();
                feat_json.as_object_mut().unwrap().remove("slug");
                feat_json
            };

            let title = &f.title;
            let flag = option_literal(&f.flag);
            let slug = f.slug;
            let rfc_id = option_literal(&f.rfc_id);
            let impl_pr_id = option_literal(&f.impl_pr_id);
            let tracking_issue_id = option_literal(&f.tracking_issue_id);
            let stabilization_pr_id = option_literal(&f.stabilization_pr_id);
            let doc_path = option_literal(&f.doc_path);
            let edition_guide_path = option_literal(&f.edition_guide_path);
            let unstable_book_path = option_literal(&f.unstable_book_path);
            let items = &f.items;

            let version = match v_idx {
                Some(idx) => quote!(Some(&VERSIONS[#idx])),
                None => quote!(None),
            };

            features.push(quote! {
                FeatureData {
                    title: #title,
                    flag: #flag,
                    slug: #slug,
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
            });

            feat_idx += 1;
        }
    }

    let versions = quote! {
        pub static VERSIONS: &[VersionData] = &[#(#versions),*];
    };

    let features = quote! {
        #[allow(clippy::unreadable_literal)]
        pub static FEATURES: &[FeatureData] = &[#(#features),*];
    };

    let monogram_index_insert_stmts = monogram_index.into_iter().map(|([b], v)| {
        quote! {
            index.insert(#b, &[#(#v),*] as &[u16]);
        }
    });

    let monogram_feature_index = quote! {
        pub static FEATURE_MONOGRAM_INDEX: once_cell::sync::Lazy<std::collections::HashMap<u8, &[u16]>> =
            once_cell::sync::Lazy::new(|| {
                let mut index = std::collections::HashMap::new();
                #(#monogram_index_insert_stmts)*
                index
            });
    };

    let bigram_index_insert_stmts = bigram_index.into_iter().map(|([b1, b2], v)| {
        quote! {
            index.insert([#b1, #b2], &[#(#v),*] as &[u16]);
        }
    });

    let bigram_feature_index = quote! {
        pub static FEATURE_BIGRAM_INDEX: once_cell::sync::Lazy<std::collections::HashMap<[u8; 2], &[u16]>> =
            once_cell::sync::Lazy::new(|| {
                let mut index = std::collections::HashMap::new();
                #(#bigram_index_insert_stmts)*
                index
            });
    };

    let trigram_index_insert_stmts = trigram_index.into_iter().map(|([b1, b2, b3], v)| {
        quote! {
            index.insert([#b1, #b2, #b3], &[#(#v),*] as &[u16]);
        }
    });

    let trigram_feature_index = quote! {
        pub static FEATURE_TRIGRAM_INDEX: once_cell::sync::Lazy<std::collections::HashMap<[u8; 3], &[u16]>> =
            once_cell::sync::Lazy::new(|| {
                let mut index = std::collections::HashMap::new();
                #(#trigram_index_insert_stmts)*
                index
            });
    };

    let stream = quote! {
        #versions
        #features
        #monogram_feature_index
        #bigram_feature_index
        #trigram_feature_index
    };

    (stream, json)
}

fn option_literal<T: ToTokens>(opt: &Option<T>) -> TokenStream {
    match opt {
        Some(lit) => quote! { Some(#lit) },
        None => quote! { None },
    }
}

fn add_feature_ngrams<const N: usize>(
    index: &mut BTreeMap<[u8; N], BTreeSet<u16>>,
    feature: &FeatureData,
    idx: u16,
) {
    let mut strings = vec![&feature.title];
    if let Some(f) = &feature.flag {
        strings.push(f);
    }
    strings.extend(feature.items.iter());

    for string in strings {
        for ngram in string.as_bytes().array_windows() {
            if ngram.iter().all(|&byte| byte.is_ascii_graphic() && byte != b'`') {
                index.entry(*ngram).or_default().insert(idx);
            }
        }
    }
}
