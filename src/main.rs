use stdweb::web::{document, IParentNode};
use yew_router::Switch;

mod features;
mod search;
mod util;
mod components {
    mod app;
    mod feature;
    mod feature_skel;
    mod full_feature;
    mod index;
    mod matched_feature;
    mod support_indicator;

    pub use self::{
        app::App, feature::Feature, feature_skel::FeatureSkel, full_feature::FullFeature,
        index::Index, matched_feature::MatchedFeature, support_indicator::SupportIndicator,
    };
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
