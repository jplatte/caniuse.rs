use wasm_bindgen::JsCast;
use xilem_html::{
    elements::{self as html, a, button, div, input, label, li, nav, ul},
    interfaces::Element as _,
    View, ViewMarker,
};

use crate::{
    router::{route_link, AppRoute, ListRoute},
    AppState,
};

pub(crate) fn header(is_menu_open: bool) -> impl View<AppState> + ViewMarker {
    let menu_button =
        button("\u{2630}").attr("type", "button").on_click(|state: &mut AppState, _| {
            state.is_menu_open ^= true;
        });

    html::header(
        div((
            div((
                label("Can I use").attr("for", "query"),
                input("?").attr("id", "query").attr("type", "search").on_input(
                    |state: &mut AppState, evt| {
                        if let Some(input) = evt
                            .target()
                            .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                        {
                            state.route =
                                AppRoute::List(ListRoute::SearchResults { input: input.value() });
                        }
                    },
                ),
            ))
            .attr("class", "caniuse"),
            nav((
                menu_button,
                ul((li(a("<light / dark>")), li(route_link(AppRoute::About, "About"))))
                    .attr("class", if is_menu_open { "menu active" } else { "menu" }),
            ))
            .attr("aria-label", "Site navigation"),
        ))
        .attr("class", "inner"),
    )
}
