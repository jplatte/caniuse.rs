use xilem_html::{elements::div, OneOf2, SetAttr, View, ViewMarker};

use crate::{
    data::{Channel, FeatureData},
    router::{route_link, AppRoute},
    util::view_text,
    AppState,
};

pub(crate) fn feature_entry(
    data: &FeatureData,
    show_version: ShowVersion,
) -> impl View<AppState> + ViewMarker {
    let version = matches!(show_version, ShowVersion::Yes).then(|| match data.version {
        None => OneOf2::A(div("Unstable").attr("class", "version none")),
        Some(version) => {
            let classes = match version.channel {
                Channel::Nightly => "version nightly",
                Channel::Beta => "version beta",
                Channel::Stable => "version stable",
            };

            OneOf2::B(
                div(route_link(
                    AppRoute::Version { number: version.number.into() },
                    format!("Rust {}", version.number),
                ))
                .attr("class", classes),
            )
        }
    });

    div((
        div(route_link(AppRoute::Feature { slug: data.slug.into() }, view_text(data.title))
            .attr("class", "title"))
        .attr("class", "box"),
        version,
    ))
    .attr("class", "feature-entry")
}

pub(crate) enum ShowVersion {
    Yes,
    No,
}
