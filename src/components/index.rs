use xilem_html::{
    elements::{a, div, nav},
    OneOf3, ViewExt, ViewSequence,
};

use super::feature_entry;
use crate::{
    data::FEATURES,
    router::{AppRoute, ListRoute},
    search::{extract_search_terms, run_search, InvalidSearchQuery},
    AppState,
};

pub(crate) fn index(
    route: &ListRoute,
    search_scores: &mut [(u16, f64)],
) -> impl ViewSequence<AppState> {
    let mut stable_link = a("Stable").attr("href", "/");
    let mut recently_stabilized_link = a("Recently Stabilized").attr("href", "/recent");
    let mut unstable_link = a("Unstable").attr("href", "/unstable");

    let contents = match route {
        ListRoute::Stable => {
            stable_link = stable_link.attr("class", "active");
            OneOf3::A(
                div(FEATURES
                    .iter()
                    .take_while(|f| f.is_stable())
                    .map(feature_entry)
                    .collect::<Vec<_>>())
                .attr("class", "feature-list"),
            )
        }
        ListRoute::RecentlyStabilized => {
            recently_stabilized_link = recently_stabilized_link.attr("class", "active");
            OneOf3::A(
                div(FEATURES
                    .iter()
                    .skip_while(|f| !f.is_recently_stabilized())
                    .take_while(|f| f.is_recently_stabilized())
                    .map(feature_entry)
                    .collect())
                .attr("class", "feature-list"),
            )
        }
        ListRoute::Unstable => {
            unstable_link = unstable_link.attr("class", "active");
            OneOf3::A(
                div(FEATURES.iter().skip_while(|f| !f.is_unstable()).map(feature_entry).collect())
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
                        div(results.iter().map(feature_entry).collect::<Vec<_>>())
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
            stable_link
                .on_click(|state: &mut AppState, evt| {
                    state.route = AppRoute::List(ListRoute::Stable);
                    evt.prevent_default();
                })
                .passive(false),
            recently_stabilized_link
                .on_click(|state: &mut AppState, evt| {
                    state.route = AppRoute::List(ListRoute::RecentlyStabilized);
                    evt.prevent_default();
                })
                .passive(false),
            unstable_link
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
