use std::fmt::Display;

use yew::{
    html,
    virtual_dom::{VList, VNode, VTag, VText},
    Classes, Html,
};

use crate::{icons::fa_home, AppRoute, RouterLink};

pub enum Void {}

pub fn view_text(mut text: &str) -> Html {
    fn list_to_node(list: VList) -> VNode {
        if list.len() == 1 {
            list[0].clone()
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
    let classes: Classes = "button".into();
    html! {
        <RouterLink to={AppRoute::Index} classes={classes}>{fa_home()}</RouterLink>
    }
}

pub fn maybe_link<T: Display>(text: &str, link_base: &str, opt_rest: Option<T>) -> Html {
    match opt_rest {
        Some(id) => html! { <li><a href={format!("{}{}", link_base, id)}>{text}</a></li> },
        None => html! {},
    }
}
