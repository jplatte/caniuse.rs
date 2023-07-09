use xilem_html::{
    elements::{a, div},
    OneOf2, View, ViewExt, ViewMarker,
};

use crate::{
    data::{Channel, FeatureData},
    router::AppRoute,
    util::view_text,
    AppState,
};

pub(crate) fn feature_entry(data: &FeatureData) -> impl View<AppState> + ViewMarker {
    let version = match data.version {
        None => OneOf2::A(div("Unstable").attr("class", "version none")),
        Some(version) => {
            let classes = match version.channel {
                Channel::Nightly => "version nightly",
                Channel::Beta => "version beta",
                Channel::Stable => "version stable",
            };

            OneOf2::B(
                div(a(format!("Rust {}", version.number)).attr("href", "#")).attr("class", classes),
            )
        }
    };

    div((
        div(a(view_text(data.title))
            .attr("class", "title")
            .on_click(|state: &mut AppState, evt| {
                state.route = AppRoute::Feature { slug: data.slug.into() };
                evt.prevent_default();
            })
            .passive(false))
        .attr("class", "box"),
        version,
    ))
    .attr("class", "feature-entry")
}
