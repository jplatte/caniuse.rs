use std::fmt::Display;

use xilem_html::{
    elements::{a, code, li, Code},
    interfaces::Element as _,
    OneOf2, ViewSequence,
};

use crate::AppState;

pub fn view_text<T>(mut text: &'static str) -> Vec<OneOf2<&str, Code<T, (), &str>>> {
    let mut res = Vec::new();

    while let Some(backtick_pos) = text.find('`') {
        if backtick_pos != 0 {
            res.push(OneOf2::A(&text[..backtick_pos]));
        }
        text = &text[backtick_pos + 1..];

        let next_backtick_pos = match text.find('`') {
            Some(pos) => pos,
            None => {
                // This should never happen, backticks should be balanced
                break;
            }
        };

        res.push(OneOf2::B(code(&text[..next_backtick_pos])));
        text = &text[next_backtick_pos + 1..];
    }

    // Use the rest of the text verbatim
    res.push(OneOf2::A(text));
    res
}

pub(crate) fn list_link<T: Display>(
    text: &'static str,
    link_base: &str,
    rest: T,
) -> impl ViewSequence<AppState> {
    li(a(text).attr("href", format!("{link_base}{rest}")))
}

pub(crate) fn maybe_list_link<T: Display>(
    text: &'static str,
    link_base: &str,
    opt_rest: Option<T>,
) -> impl ViewSequence<AppState> {
    opt_rest.map(|rest| list_link(text, link_base, rest))
}

/* pub fn home_button() -> Html {
    let classes: Classes = "button".into();
    html! {
        <RouterLink to={AppRoute::Index} classes={classes}>{fa_home()}</RouterLink>
    }
}*/
