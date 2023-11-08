use std::ops::Not;

use xilem_html::{
    elements::{code, div, h3, li, pre, span, ul},
    interfaces::Element as _,
    OneOf2, ViewSequence,
};

use crate::{
    data::{Channel, FeatureData},
    router::{route_link, AppRoute},
    util::{list_link, maybe_list_link, view_text},
    AppState,
};

pub(crate) fn feature_page(data: &FeatureData) -> impl ViewSequence<AppState> {
    // TODO: Home button
    div((
        h3(view_text(data.title)).attr("class", "title"),
        div((
            span("Since version:"),
            span(match data.version {
                Some(v) => {
                    OneOf2::A(route_link(AppRoute::Version { number: v.number.into() }, v.number))
                }
                None => OneOf2::B("none (unstable)"),
            }),
            data.flag.map(|flag| (span("Feature flag:"), span(code(view_text(flag))))),
        ))
        .attr("class", "info"),
        ul((
            maybe_list_link("RFC", "https://github.com/rust-lang/rfcs/issues/", data.rfc_id),
            maybe_list_link(
                "Implementation PR",
                "https://github.com/rust-lang/rust/pull/",
                data.impl_pr_id,
            ),
            maybe_list_link(
                "Tracking issue",
                "https://github.com/rust-lang/rust/issues/",
                data.tracking_issue_id,
            ),
            maybe_list_link(
                "Stabilization PR",
                "https://github.com/rust-lang/rust/pull/",
                data.stabilization_pr_id,
            ),
            data.doc_path.map(|path| {
                let prefix = match data.version.map_or(Channel::Nightly, |v| v.channel) {
                    Channel::Nightly => "nightly/",
                    Channel::Beta => "beta/",
                    Channel::Stable => "",
                };
                list_link("Documentation", "https://doc.rust-lang.org/", format!("{prefix}{path}"))
            }),
            maybe_list_link(
                "Edition Guide",
                "https://doc.rust-lang.org/edition-guide/",
                data.edition_guide_path,
            ),
            maybe_list_link(
                "Unstable book",
                "https://doc.rust-lang.org/unstable-book/",
                data.unstable_book_path,
            ),
        ))
        .attr("class", "links"),
        data.items.is_empty().not().then(|| view_items(data.items)),
    ))
    .attr("class", "box")
}

fn view_items(items: &'static [&str]) -> impl ViewSequence<AppState> {
    let items: Vec<_> = items
        .iter()
        .map(|&item| {
            li(if item.contains('\n') { OneOf2::A(pre(item)) } else { OneOf2::B(code(item)) })
        })
        .collect();

    div(("Items", ul(items))).attr("class", "item")
}
