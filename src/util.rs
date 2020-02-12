use std::fmt::Display;

use yew::{
    html,
    virtual_dom::{VList, VNode, VTag, VText},
    Html,
};

use crate::search::Span;

pub enum Void {}

pub fn view_text(text: &str) -> Html {
    view_text_with_matches(text, &[])
}

// Does search match highlighting as well as replacing `` by <code></code>.
// Requires the spans to be sorted.
pub fn view_text_with_matches(mut text: &str, mut spans: &[Span]) -> Html {
    fn list_to_node(list: VList) -> VNode {
        if list.len() == 1 {
            list.children.into_iter().next().unwrap()
        } else {
            VNode::VList(list)
        }
    }

    enum Op {
        OpenCodeTag,
        CloseCodeTag,
        AddHighlight { len: usize },
    }

    use Op::*;

    let mut res = VList::new();
    let mut innermost = &mut res;
    let mut span_offset = 0;
    let mut codetag_open = false;

    loop {
        let backtick_pos = text.find('`');
        let next_highlight = spans.first().copied();

        let (op, idx) = match (backtick_pos, next_highlight) {
            (None, None) => {
                // No replacements to do anymore, use the rest of the text verbatim
                innermost.add_child(VNode::VText(VText::new(text.into())));
                break;
            }
            (Some(bt_idx), maybe_sp) => match maybe_sp {
                Some(Span { start, len }) if (start - span_offset) < bt_idx => {
                    (AddHighlight { len }, start - span_offset)
                }
                _ => {
                    if codetag_open {
                        (CloseCodeTag, bt_idx)
                    } else {
                        (OpenCodeTag, bt_idx)
                    }
                }
            },
            (_, Some(sp)) => (AddHighlight { len: sp.len }, sp.start - span_offset),
        };

        if idx != 0 {
            innermost.add_child(text[..idx].into());
            text = &text[idx..];
            span_offset += idx;
        }

        match op {
            OpenCodeTag => {
                innermost.add_child(VNode::VTag(Box::new(VTag::new("code"))));
                innermost = match innermost.children.last_mut() {
                    Some(VNode::VTag(tag)) => &mut tag.children,
                    _ => unreachable!(),
                };
                text = &text[1..];
                span_offset += 1;
                codetag_open = true;
            }
            CloseCodeTag => {
                innermost = &mut res;
                text = &text[1..];
                span_offset += 1;
                codetag_open = false;
            }
            AddHighlight { len } => {
                let mut tag = Box::new(VTag::new("span"));
                tag.add_class("match");
                tag.add_child(text[..len].into());
                innermost.add_child(VNode::VTag(tag));
                text = &text[len..];
                span_offset += len;
                spans = &spans[1..];
            }
        }
    }

    list_to_node(res)
}

pub fn maybe_link<T: Display>(text: &str, link_base: &str, opt_rest: Option<T>) -> Html {
    match opt_rest {
        Some(id) => html! { <li><a href=format!("{}{}", link_base, id)>{text}</a></li> },
        None => html! {},
    }
}
