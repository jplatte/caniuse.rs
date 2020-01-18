use stdweb::web::{document, IParentNode};

mod features;
mod components {
    mod app;
    mod feature;
    mod feature_skel;
    mod full_feature;
    mod matched_feature;

    pub use self::{
        app::App, feature::Feature, feature_skel::FeatureSkel, full_feature::FullFeature,
        matched_feature::MatchedFeature,
    };
}

pub use features::{FeatureData, FEATURES};

fn main() {
    yew::initialize();
    let page = document().query_selector("main").unwrap().unwrap();
    yew::App::<components::App>::new().mount(page);
    yew::run_loop();
}
