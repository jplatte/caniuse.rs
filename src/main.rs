use std::mem;

use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};

mod features;

use features::FEATURES;

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
            let mut features =
                FEATURES.iter().filter(|f| f.matches(&self.current_search_term)).map(|f| {
                    // TODO: Show feature flag name
                    html! {
                        <li>
                            <span class="desc">{f.desc_short}</span>
                            <span class="since">{"Rust "}{f.stable_since}</span>
                        </li>
                    }
                });
            html! { <ul>{ for features }</ul> }
        };

        html! {
            <>
                <input type="search" oninput=self.link.callback(|e: InputData| Msg::Search(e.value)) />
                {content}
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
