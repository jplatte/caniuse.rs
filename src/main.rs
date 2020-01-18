use std::mem;

use stdweb::web::{document, IParentNode};
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
        let features = if self.current_search_term.is_empty() {
            let mut list = FEATURES.iter().map(|f| view_feature(f));
            html! { { for list } }
        } else {
            let mut list = FEATURES
                .iter()
                .filter_map(|f| f.matches(&self.current_search_term).map(|m| (f, m)))
                .map(|(f, m)| view_matched_feature(f, m));
            html! { { for list } }
        };

        html! {
            <main class="page">
                {"Can I use "}
                <input type="search" oninput=self.link.callback(|e: InputData| Msg::Search(e.value)) />
                {" ?"}
                <ul class="feature-list">{ features }</ul>
            </main>
        }
    }
}

fn view_feature(f: &Feature) -> Html {
    let maybe_flag = match f.flag {
        Some(f) => html! {
            <div class="flag">{"Feature flag: "}{f}</div>
        },
        None => html! {},
    };

    let items = if f.items.is_empty() {
        html! {}
    } else {
        html! { {view_items(f.items)} }
    };

    html! {
        <li class="feature-box">
            <div class="feature">
                <h3 class="desc">{f.desc_short}</h3>
                {maybe_flag}
                <span class="version stable">{"Rust "}{f.version}</span>
                {items}
            </div>
        </li>
    }
}

fn view_items(items: &[&str]) -> Html {
    let mut items = items.iter().map(|i| html! { <li>{i}</li> });
    html! {
        <details>
            <summary>{"Items"}</summary>
            <ul>
                { for items }
            </ul>
        </details>
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
        html! { <ul>{view_matched_items(f.items, &m.item_spans)}</ul> }
    };

    html! {
        <li class="feature-box">
            <div class="feature">
                <h3 class="desc">{desc}</h3>
                {maybe_flag}
                <span class="version stable">{"Rust "}{f.version}</span>
                {items}
            </div>
        </li>
    }
}

fn view_matched_items(items: &[&str], item_spans: &[Option<Span>]) -> Html {
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
    yew::initialize();
    let page = document().query_selector("main").unwrap().unwrap();
    yew::App::<App>::new().mount(page);
    yew::run_loop();
}
