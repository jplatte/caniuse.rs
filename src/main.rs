use std::mem;

use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};

mod features;

use features::{Feature, Match, Span, FEATURES};

struct App {
    link: ComponentLink<Self>,
    current_search_term: String,
}

enum Msg {
    Search(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App { link, current_search_term: String::new() }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Search(mut term) => {
                mem::swap(&mut self.current_search_term, &mut term);
                self.current_search_term != term
            }
        }
    }

    fn view(&self) -> Html {
        let content = if self.current_search_term.is_empty() {
            html! { <a href="list.html">{"Full list"}</a> }
        } else {
            let mut features = FEATURES
                .iter()
                .filter_map(|f| f.matches(&self.current_search_term).map(|m| (f, m)))
                .map(|(f, m)| view_matched_feature(f, m));
            html! { <ul class="feature-list">{ for features }</ul> }
        };

        html! {
            <main class="page">
                <input type="search" oninput=self.link.callback(|e: InputData| Msg::Search(e.value)) />
                {content}
            </main>
        }
    }
}

fn view_matched_feature(f: &Feature, m: Match) -> Html {
    let desc = match m.desc_span {
        Some(s) => view_text_with_match(f.desc_short, s),
        None => html! { {f.desc_short} },
    };

    let maybe_flag = match f.flag {
        Some(f) => {
            let val = match m.flag_span {
                Some(s) => view_text_with_match(f, s),
                None => html! { {f} },
            };

            html! {
                <div class="flag">{"Feature flag: "}{val}</div>
            }
        }
        None => {
            assert!(m.flag_span.is_none());
            html! {}
        }
    };

    let items = if f.items.is_empty() {
        html! {}
    } else {
        html! { <ul>{view_items(f.items, &m.item_spans)}</ul> }
    };

    html! {
        <li class="feature-box">
            <div class="feature">
                <h3 class="desc">{desc}</h3>
                {maybe_flag}
                <span class="since">{"Rust "}{f.stable_since}</span>
                {items}
            </div>
        </li>
    }
}

fn view_items(items: &[&str], item_spans: &[Option<Span>]) -> Html {
    let mut res = items.iter().zip(item_spans).filter_map(|(item, span)| {
        span.as_ref().map(|s| {
            html! {
                <li>{view_text_with_match(item, s.clone())}</li>
            }
        })
    });

    let more_items_indicator = if item_spans.iter().any(|s| s.is_none()) {
        html! { <li>{"…"}</li> }
    } else {
        html! {}
    };

    html! { <>{ for res }{more_items_indicator}</> }
}

fn view_text_with_match(text: &str, s: Span) -> Html {
    html! {
        <>
            {&text[..s.start]}
            <span class="match">{&text[s.clone()]}</span>
            {&text[s.end..]}
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
