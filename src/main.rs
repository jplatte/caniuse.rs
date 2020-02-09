use stdweb::web::{document, IParentNode};
use yew_router::Switch;

mod features;
mod search;
mod util;
mod components {
    mod app;
    mod feature_entry;
    mod feature_page;
    mod header;
    mod index;

    pub use self::{
        app::App, feature_entry::FeatureEntry, feature_page::FeaturePage, header::Header,
        index::Index,
    };
}
mod services {
    pub mod resize;
    pub mod scroll;
}

pub use features::{Channel, FeatureData, FEATURES};

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    #[to = "/features/{}"]
    Feature(String),
    #[to = "/versions/{}"]
    Version(String),
    #[to = "/"]
    Index,
}

fn main() {
    yew::initialize();
    let page = document().query_selector("main").unwrap().unwrap();
    yew::App::<components::App>::new().mount(page);
    yew::run_loop();
}
