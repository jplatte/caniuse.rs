use std::{
    collections::{BTreeMap, BTreeSet},
    default::Default,
    env,
    fmt::Debug,
    fs::{self, File},
    io::{self, BufRead, BufReader, BufWriter, Write},
    iter,
    path::Path,
};

use anyhow::{bail, Context as _};
use itertools::Itertools as _;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use serde::{Deserialize, Serialize};
use tera::{Context, Tera, Value};

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
    /// Blog post path (https://blog.rust-lang.org/{path})
    blog_post_path: Option<String>,
    /// GitHub milestone id (https://github.com/rust-lang/rust/milestone/{id})
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
struct FeatureData {
    /// Short description to identify the feature
    title: String,
    /// Feature flag name, for things that were previously or are still Rust
    /// nightly features with such a thing (`#![feature(...)]`)
    flag: Option<String>,
    /// Feature slug, used for the permalink. If a feature flag exists, this
    /// can be omitted, then the flag is used for the permalink.
    #[serde(skip_deserializing)] // filled from filename or flag
    slug: String,
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

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=templates/index.html");
    println!("cargo:rerun-if-changed=templates/nightly.html");
    println!("cargo:rerun-if-changed=templates/skel.html");

    let data = collect_data()?;

    // TODO: Add a filter that replaces `` by <code></code>
    let mut tera = Tera::new("templates/*").context("loading templates")?;
    tera.register_filter("code_backticks", |val: &Value, _: &_| -> tera::Result<Value> {
        let input: String = tera::from_value(val.clone())?;
        let output: String = input
            .split('`')
            .interleave_shortest(["<code>", "</code>"].iter().copied().cycle())
            .collect();
        Ok(tera::to_value(output)?)
    });

    // Try to create `public` directory
    fs::create_dir("public")
        .or_else(|e| if e.kind() == io::ErrorKind::AlreadyExists { Ok(()) } else { Err(e) })
        .context("creating dir public")?;

    let ctx = Context::from_serialize(&data)?;
    fs::write(
        "public/index.html",
        tera.render("index.html", &ctx).context("rendering index.html")?,
    )
    .context("writing public/index.html")?;
    fs::write(
        "public/nightly.html",
        tera.render("nightly.html", &ctx).context("rendering nightly.html")?,
    )
    .context("writing public/nightly.html")?;

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("features.rs");
    let mut out = BufWriter::new(File::create(out_path).context("creating $OUT_DIR/features.rs")?);

    write!(out, "{}", generate_output(data)).context("writing features.rs")?;

    Ok(())
}

fn collect_data() -> anyhow::Result<Data> {
    let mut data = Data {
        versions: Vec::new(),
        unstable: FeatureList { version: None, features: Vec::new() },
    };

    for entry in fs::read_dir("data").context("opening data/")? {
        let dir = entry?;
        assert!(dir.file_type()?.is_dir(), "expected only directories in data/");

        let dir_name = dir.file_name().into_string().unwrap();
        println!("cargo:rerun-if-changed=data/{}", dir_name);

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

        for entry in
            fs::read_dir(dir.path()).with_context(|| format!("opening data/{}", dir_name))?
        {
            let file = entry?;
            let file_name = file.file_name().into_string().unwrap();
            println!("cargo:rerun-if-changed=data/{}/{}", dir_name, file_name);

            if file_name == "version.toml" {
                continue;
            }

            assert!(
                file_name.ends_with(".md"),
                "expected only .md files and version.toml in data/*"
            );
            let feature_file = BufReader::new(
                File::open(file.path())
                    .with_context(|| format!("opening data/{}/{}", dir_name, file_name))?,
            );

            let mut feature_file_lines = feature_file.lines();
            let mut feature_file_frontmatter = String::new();
            assert_eq!(
                match feature_file_lines.next() {
                    Some(Ok(s)) => s,
                    _ => bail!("reading first line of data/{}/{} failed", dir_name, file_name),
                },
                "+++",
                "expected frontmatter at the beginning of data/{}/{}",
                dir_name,
                file_name
            );

            loop {
                match feature_file_lines.next() {
                    Some(Ok(s)) if s == "+++" => break,
                    Some(Ok(s)) => {
                        feature_file_frontmatter += s.as_str();
                        feature_file_frontmatter.push('\n');
                    }
                    _ => bail!("reading frontmatter of data/{}/{} failed", dir_name, file_name),
                }
            }

            // TODO: Read file contents after frontmatter

            let mut feature: FeatureData =
                toml::from_str(&feature_file_frontmatter).with_context(|| {
                    format!("deserializing frontmatter of data/{}/{}", dir_name, file_name)
                })?;

            //             [   file_name without '.md'    ]
            feature.slug = file_name[..file_name.len() - 3].to_owned();
            features.push(feature);
        }
    }

    Ok(data)
}

fn generate_output(feature_toml: Data) -> TokenStream {
    let mut monogram_index = BTreeMap::new();
    let mut bigram_index = BTreeMap::new();
    let mut trigram_index = BTreeMap::new();

    let mut versions = Vec::new();
    let mut features = Vec::new();

    let mut feat_idx = 0;

    for v in feature_toml.versions.into_iter().chain(iter::once(feature_toml.unstable)) {
        let v_idx = v.version.map(|d| {
            let number = &d.number;
            let channel = Ident::new(&format!("{:?}", d.channel), Span::call_site());
            let blog_post_path = option_literal(&d.blog_post_path);
            let gh_milestone_id = option_literal(&d.gh_milestone_id);

            versions.push(quote! {
                VersionData {
                    number: #number,
                    channel: Channel::#channel,
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

            add_feature_ngrams(1, &mut monogram_index, &f, feat_idx);
            add_feature_ngrams(2, &mut bigram_index, &f, feat_idx);
            add_feature_ngrams(3, &mut trigram_index, &f, feat_idx);

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
        pub const VERSIONS: &[VersionData] = &[#(#versions),*];
    };

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
        #versions
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
    feature: &FeatureData,
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
