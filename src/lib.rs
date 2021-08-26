#![recursion_limit = "512"]

use wasm_bindgen::prelude::wasm_bindgen;
use yew_router::Switch;

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
use util::document;

#[derive(Clone, Debug, Switch)]
enum AppRoute {
    #[to = "/features/{}"]
    Feature(String),
    #[to = "/versions/{}"]
    Version(String),
    #[to = "/about"]
    About,
    #[to = "/recent"]
    RecentlyStabilized,
    #[to = "/unstable"]
    Unstable,
    #[to = "/"]
    Index,
}

type RouterAnchor = yew_router::components::RouterAnchor<AppRoute>;
type RouterButton = yew_router::components::RouterButton<AppRoute>;

#[wasm_bindgen]
pub fn run() {
    yew::initialize();
    let page = document().query_selector("main").unwrap().unwrap();
    yew::App::<components::App>::new().mount(page);
    yew::run_loop();
}
