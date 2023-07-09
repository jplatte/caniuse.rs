use xilem_html::{
    elements::{a, div, h3, li, p, ul},
    ViewSequence,
};

use crate::AppState;

pub(crate) fn about() -> impl ViewSequence<AppState> {
    // TODO: home button
    div((
        h3("About caniuse.rs"),
        p((
            "Created by Jonas Platte, in Rust, using ",
            a("xilem_html")
                .attr("href", "https://github.com/linebender/xilem/tree/main/crates/xilem_html"),
            ".",
        )),
        p((
            "You can find the code for this site on ",
            a("GitHub").attr("href", "https://github.com/jplatte/caniuse.rs"),
            ".",
        )),
        h3("About the creator"),
        p((
            "I'm Jonas and I work on free software in my spare time, usually on projects \
             written in Rust and / or for the Linux desktop. I am a maintainer of the ",
            a("Ruma").attr("href", "https://ruma.io/"),
            " project and have made minor contributions to many other open-source projects \
              over the course of the years.",
        )),
        p(("I've also created ", a("turbo.fish").attr("href", "https://turbo.fish/"), ".")),
        p((
            "You can find me on",
            ul((
                li(a("GitHub").attr("href", "https://github.com/jplatte")),
                li(a("sourcehut").attr("href", "https://git.sr.ht/~jplatte")),
                li(a("My blog").attr("href", "https://blog.turbo.fish/")),
            )),
        )),
        p((
            "If you want to support me financially, you can do so on ",
            ul(li(a("Liberapay").attr("href", "https://liberapay.com/jplatte"))),
        )),
    ))
    .attr("class", "about box")
}
