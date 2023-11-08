use xilem_html::{
    elements::{div, h3, span, time, ul},
    interfaces::Element as _,
    ViewSequence,
};

use crate::{
    data::{VersionData, FEATURES},
    util::maybe_list_link,
    AppState,
};

use super::{feature_entry, feature_entry::ShowVersion};

pub(crate) fn version_page(data: &VersionData) -> impl ViewSequence<AppState> {
    let maybe_blog_link =
        maybe_list_link("Blog post", "https://blog.rust-lang.org/", data.blog_post_path);
    let maybe_release_notes = maybe_list_link(
        "Release notes",
        "https://github.com/rust-lang/rust/blob/master/RELEASES.md#",
        data.release_notes,
    );
    let maybe_gh_milestone_link = maybe_list_link(
        "GitHub milestone",
        "https://github.com/rust-lang/rust/milestone/",
        data.gh_milestone_id,
    );

    let maybe_release_date = data.release_date.map(|release_date| {
        (span("Release date:"), time(release_date).attr("datetime", release_date))
    });

    let features: Vec<_> = FEATURES
        .iter()
        .filter(|f| matches!(f.version, Some(v) if v.number == data.number))
        .map(|f| feature_entry(f, ShowVersion::No))
        .collect();

    (
        // TODO: Home button
        div((
            h3(format!("Rust {}", data.number)).attr("class", "title"),
            div(maybe_release_date).attr("class", "info"),
            ul((maybe_blog_link, maybe_release_notes, maybe_gh_milestone_link))
                .attr("class", "links"),
        ))
        .attr("class", "box"),
        div(features).attr("class", "feature-list"),
    )
}
