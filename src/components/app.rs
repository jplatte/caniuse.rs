use std::mem;

use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};

use crate::{
    components::{Feature, MatchedFeature},
    FEATURES,
};

pub struct App {
    link: ComponentLink<Self>,
    current_search_term: String,
}

pub enum Msg {
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
            let mut list = FEATURES.iter().map(|f| html! { <Feature data=Some(*f) /> });
            html! { { for list } }
        } else {
            let mut list = FEATURES
                .iter()
                .filter_map(|f| f.matches(&self.current_search_term).map(|m| (f, m)))
                .map(|(f, m)| html! { <MatchedFeature data=Some(*f) match_={m} /> });
            html! { { for list } }
        };

        html! {
            <>
                {"Can I use "}
                <input type="search" oninput=self.link.callback(|e: InputData| Msg::Search(e.value)) />
                {" ?"}
                <ul class="feature-list">{ features }</ul>
            </>
        }
    }
}
