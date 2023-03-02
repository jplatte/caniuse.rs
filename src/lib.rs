#![allow(clippy::derive_partial_eq_without_eq)]

use gloo_utils::document;
use wasm_bindgen::prelude::wasm_bindgen;
use yew_router::Routable;

mod data;
mod icons;
mod search;
mod util;
mod components {
    mod about;
    mod app;
    mod feature_entry;
    mod feature_page;
    mod header;
    mod index;
    mod version_page;

    pub use self::{
        about::About, app::App, feature_entry::FeatureEntry, feature_page::FeaturePage,
        header::Header, index::Index, version_page::VersionPage,
    };
}

use data::{Channel, FeatureData, VersionData, FEATURES, VERSIONS};

#[derive(Clone, Debug, PartialEq, Routable)]
enum AppRoute {
    #[at("/features/:name")]
    Feature { name: String },
    #[at("/versions/:number")]
    Version { number: String },
    #[at("/about")]
    About,
    #[at("/recent")]
    RecentlyStabilized,
    #[at("/unstable")]
    Unstable,
    #[at("/")]
    Index,
    #[at("/search/:query")]
    SearchIndex { query: String },
}

type RouterLink = yew_router::components::Link<AppRoute>;

#[wasm_bindgen]
pub fn run() {
    let page = document().query_selector("main").unwrap().unwrap();
    yew::start_app_in_element::<components::App>(page);
}
