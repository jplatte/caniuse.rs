use std::fmt::Display;

use web_sys::{Document, Element, HtmlElement, Window};
use yew::{
    html,
    virtual_dom::{VList, VNode, VTag, VText},
    Html,
};

use crate::{AppRoute, RouterButton};

pub enum Void {}

pub fn window() -> Window {
    web_sys::window().unwrap()
}

pub fn document() -> Document {
    window().document().unwrap()
}

pub fn document_body() -> HtmlElement {
    document().body().unwrap()
}

pub fn document_element() -> Element {
    document().document_element().unwrap()
}

pub fn view_text(mut text: &str) -> Html {
    fn list_to_node(list: VList) -> VNode {
        if list.len() == 1 {
            list.children.into_iter().next().unwrap()
        } else {
            VNode::VList(list)
        }
    }

    let mut res = VList::new();

    while let Some(backtick_pos) = text.find('`') {
        if backtick_pos != 0 {
            res.add_child(text[..backtick_pos].into());
        }
        text = &text[backtick_pos + 1..];

        let next_backtick_pos = match text.find('`') {
            Some(pos) => pos,
            None => {
                // This should never happen, backticks should be balanced
                break;
            }
        };

        let mut code = VTag::new("code");
        code.add_child(text[..next_backtick_pos].into());
        text = &text[next_backtick_pos + 1..];

        res.add_child(VNode::VTag(Box::new(code)));
    }

    // Use the rest of the text verbatim
    res.add_child(VNode::VText(VText::new(text.into())));

    list_to_node(res)
}

// TODO: Go back in browser history if the previous page was part of the app
pub fn back_button() -> Html {
    // SVG data obtained from Fonticons Inc. under CC BY 4.0, via https://fontawesome.com/
    html! {
        <RouterButton route=AppRoute::Index>
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512" aria-label="back">
                <path d="M257.5 445.1l-22.2 22.2c-9.4 9.4-24.6 9.4-33.9 0L7 273c-9.4-9.4-9.4-24.6 0-33.9L201.4 44.7c9.4-9.4 24.6-9.4 33.9 0l22.2 22.2c9.5 9.5 9.3 25-.4 34.3L136.6 216H424c13.3 0 24 10.7 24 24v32c0 13.3-10.7 24-24 24H136.6l120.5 114.8c9.8 9.3 10 24.8.4 34.3z"/>
            </svg>
        </RouterButton>
    }
}

pub fn maybe_link<T: Display>(text: &str, link_base: &str, opt_rest: Option<T>) -> Html {
    match opt_rest {
        Some(id) => html! { <li><a href=format!("{}{}", link_base, id)>{text}</a></li> },
        None => html! {},
    }
}
