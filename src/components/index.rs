use std::rc::Rc;

use gloo::{events::EventListener, timers::callback::Timeout};
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::{
    components::FeatureEntry,
    search::{extract_search_terms, run_search},
    util::{document_body, window},
    FeatureData, FEATURES,
};

pub struct Index {
    link: ComponentLink<Self>,
    current_search_terms: Vec<String>,
    current_search_results: Vec<FeatureData>,
    items_visible: usize,
    search_scores: Vec<(u16, f64)>,

    _scroll_listener: EventListener,
    _resize_listener: EventListener,
    _timeout: Timeout,
}

pub enum Msg {
    Update,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub search_query: Rc<String>,
}

const BATCH_SIZE: usize = 20;

impl Component for Index {
    type Message = Msg;
    type Properties = Props;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let _scroll_listener = EventListener::new(&window(), "scroll", {
            let link = link.clone();
            move |_| link.send_message(Msg::Update)
        });
        let _resize_listener = EventListener::new(&window(), "resize", {
            let link = link.clone();
            move |_| link.send_message(Msg::Update)
        });
        let _timeout = create_timeout(link.clone());

        Self {
            link,
            current_search_terms: Vec::new(),
            current_search_results: Vec::new(),
            items_visible: BATCH_SIZE,
            search_scores: vec![(0, 0.0); FEATURES.len()],

            _scroll_listener,
            _resize_listener,
            _timeout,
        }
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        match msg {
            Msg::Update => {
                let inner_height = window().inner_height().unwrap().as_f64().unwrap();
                let scroll_y = window().scroll_y().unwrap();
                let distance_to_bottom =
                    document_body().scroll_height() as f64 - scroll_y - inner_height;

                if distance_to_bottom < inner_height {
                    self.items_visible += BATCH_SIZE;
                    self._timeout = create_timeout(self.link.clone());

                    true
                } else {
                    false
                }
            }
        }
    }

    fn change(&mut self, props: Props) -> ShouldRender {
        let search_terms = extract_search_terms(&props.search_query).unwrap_or_default();

        self.current_search_results = run_search(&search_terms, &mut self.search_scores);
        self.current_search_terms = search_terms;

        self.items_visible = BATCH_SIZE;
        self._timeout = create_timeout(self.link.clone());

        true
    }

    fn view(&self) -> Html {
        if self.current_search_terms.is_empty() {
            let list = FEATURES.iter().map(|&f| html! { <FeatureEntry key=f.slug data=f /> });
            html! { <div class="feature-list">{ for list.take(self.items_visible) }</div> }
        } else if self.current_search_results.is_empty() {
            html! { <div class="box muted">{"Nothing found, sorry."}</div> }
        } else {
            let list = self.current_search_results.iter().map(|&f| {
                html! { <FeatureEntry key=f.slug data=f /> }
            });

            html! { <div class="feature-list">{ for list.take(self.items_visible) }</div> }
        }
    }
}

// Creates a timeout that lets the browser render the page before calling `fn update()`.
fn create_timeout(link: ComponentLink<Index>) -> Timeout {
    Timeout::new(0, move || link.send_message(Msg::Update))
}
