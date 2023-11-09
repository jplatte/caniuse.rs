use xilem_html::{
    elements::{div, nav},
    interfaces::Element as _,
    OneOf3, ViewSequence,
};

use super::{feature_entry, feature_entry::ShowVersion};
use crate::{
    data::FEATURES,
    router::{route_link, AppRoute, ListRoute},
    search::{extract_search_terms, run_search, InvalidSearchQuery},
    AppState,
};

pub(crate) fn index(
    route: &ListRoute,
    search_scores: &mut [(u16, f64)],
) -> impl ViewSequence<AppState> {
    let mut stable_link_class = None;
    let mut recently_stabilized_link_class = None;
    let mut unstable_link_class = None;

    let contents = match route {
        ListRoute::Stable => {
            stable_link_class = Some("active");
            OneOf3::A(
                div(FEATURES
                    .iter()
                    .take_while(|f| f.is_stable())
                    .map(|f| feature_entry(f, ShowVersion::Yes))
                    .collect::<Vec<_>>())
                .attr("class", "feature-list"),
            )
        }
        ListRoute::RecentlyStabilized => {
            recently_stabilized_link_class = Some("active");
            OneOf3::A(
                div(FEATURES
                    .iter()
                    .skip_while(|f| !f.is_recently_stabilized())
                    .take_while(|f| f.is_recently_stabilized())
                    .map(|f| feature_entry(f, ShowVersion::Yes))
                    .collect())
                .attr("class", "feature-list"),
            )
        }
        ListRoute::Unstable => {
            unstable_link_class = Some("active");
            OneOf3::A(
                div(FEATURES
                    .iter()
                    .skip_while(|f| !f.is_unstable())
                    .map(|f| feature_entry(f, ShowVersion::Yes))
                    .collect())
                .attr("class", "feature-list"),
            )
        }
        ListRoute::SearchResults { input } => match extract_search_terms(input) {
            Ok(search_terms) => {
                let results = run_search(&search_terms, search_scores);
                if results.is_empty() {
                    OneOf3::C(div("Nothing found, sorry.").attr("class", "box muted"))
                } else {
                    OneOf3::B(
                        div(results
                            .iter()
                            .map(|f| feature_entry(f, ShowVersion::Yes))
                            .collect::<Vec<_>>())
                        .attr("class", "feature-list"),
                    )
                }
            }
            Err(InvalidSearchQuery) => {
                OneOf3::C(div("Invalid search terms.").attr("class", "box muted"))
            }
        },
    };

    (
        nav(div((
            route_link(AppRoute::List(ListRoute::Stable), "Stable")
                .attr("class", stable_link_class)
                .on_click(|state: &mut AppState, evt| {
                    state.route = AppRoute::List(ListRoute::Stable);
                    evt.prevent_default();
                })
                .passive(false),
            route_link(AppRoute::List(ListRoute::RecentlyStabilized), "Recently Stabilized")
                .attr("class", recently_stabilized_link_class)
                .on_click(|state: &mut AppState, evt| {
                    state.route = AppRoute::List(ListRoute::RecentlyStabilized);
                    evt.prevent_default();
                })
                .passive(false),
            route_link(AppRoute::List(ListRoute::Unstable), "Unstable")
                .attr("class", unstable_link_class)
                .on_click(|state: &mut AppState, evt| {
                    state.route = AppRoute::List(ListRoute::Unstable);
                    evt.prevent_default();
                })
                .passive(false),
        ))
        .attr("class", "inner"))
        .attr("class", "explore"),
        contents,
    )
}
