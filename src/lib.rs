use data::{FEATURES, VERSIONS};
use gloo_utils::document;
use wasm_bindgen::prelude::wasm_bindgen;
use xilem_html::{
    elements::{div, main},
    interfaces::Element as _,
    App, Memoize, OneSeqOf5, View,
};

mod data;
mod router;
mod search;
mod util;

mod components {
    mod about;
    mod feature_entry;
    mod feature_page;
    mod header;
    mod index;
    mod version_page;

    pub(crate) use self::{
        about::about, feature_entry::feature_entry, feature_page::feature_page, header::header,
        index::index, version_page::version_page,
    };
}

use self::router::AppRoute;

//enum Theme {
//    Light,
//    Dark,
//}

struct AppState {
    route: AppRoute,
    //theme: Theme,
    is_menu_open: bool,
    search_scores: Vec<(u16, f64)>,
}

impl AppState {
    fn new() -> Self {
        Self {
            route: AppRoute::new(),
            // TODO: Use body data attr
            //theme: Theme::Dark,
            is_menu_open: false,
            search_scores: vec![(0, 0.0); FEATURES.len()],
        }
    }
}

fn app(state: &mut AppState) -> impl View<AppState> {
    let page_content = match &mut state.route {
        AppRoute::List(list_route) => {
            OneSeqOf5::A(components::index(list_route, &mut state.search_scores))
        }
        AppRoute::Feature { slug } => match FEATURES.iter().find(|f| f.slug == slug) {
            Some(data) => OneSeqOf5::B(components::feature_page(data)),
            None => OneSeqOf5::E("error: feature not found!"),
        },
        AppRoute::Version { number } => match VERSIONS.iter().find(|f| f.number == number) {
            Some(data) => OneSeqOf5::C(components::version_page(data)),
            None => OneSeqOf5::E("error: version not found!"),
        },
        AppRoute::About => OneSeqOf5::D(components::about()),
    };
    main((
        Memoize::new(state.is_menu_open, |&is_menu_open| components::header(is_menu_open)),
        div(page_content).attr("class", "page"),
    ))
}

#[wasm_bindgen]
pub fn run() {
    #[cfg(debug_assertions)]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let page = document().body().unwrap();
    App::new(AppState::new(), app).run(&page);
}
