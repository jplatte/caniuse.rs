use xilem_html::{
    elements::{self as html, a, button, div, input, label, li, nav, ul},
     View, ViewExt, ViewMarker,
};

use crate::{
    router::{route_link, AppRoute, ListRoute},
    AppState,
};

pub(crate) fn header(state: &mut AppState) -> impl View<AppState> + ViewMarker {
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
                        state.route = AppRoute::List(ListRoute::SearchResults {
                            input: evt.target().value(),
                        });
                    },
                ),
            ))
            .attr("class", "caniuse"),
            nav((
                menu_button,
                ul((li(a("<light / dark>")), li(route_link(AppRoute::About, "About"))))
                    .attr("class", if state.is_menu_open { "menu active" } else { "menu" }),
            ))
            .attr("aria-label", "Site navigation"),
        ))
        .attr("class", "inner"),
    )
}
