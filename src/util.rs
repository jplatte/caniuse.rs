use std::fmt::Display;

use web_sys::{Document, Element, HtmlElement, Window};
use yew::{
    html,
    virtual_dom::{VList, VNode, VTag, VText},
    Html,
};

use crate::{icons::fa_home, AppRoute, RouterButton};

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
    res.add_child(VNode::VText(VText::new(text.to_owned())));

    list_to_node(res)
}

pub fn home_button() -> Html {
    html! {
        <RouterButton route=AppRoute::Index>{fa_home()}</RouterButton>
    }
}

pub fn maybe_link<T: Display>(text: &str, link_base: &str, opt_rest: Option<T>) -> Html {
    match opt_rest {
        Some(id) => html! { <li><a href=format!("{}{}", link_base, id)>{text}</a></li> },
        None => html! {},
    }
}
