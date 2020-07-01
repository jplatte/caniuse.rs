use std::{rc::Rc, time::Duration};

use yew::{
    html,
    services::{
        resize::{ResizeService, ResizeTask},
        timeout::{TimeoutService, TimeoutTask},
    },
    Component, ComponentLink, Html, Properties, ShouldRender,
};

use crate::{
    components::FeatureEntry,
    search::{extract_search_terms, run_search},
    services::scroll::{ScrollService, ScrollTask},
    util::{document_body, window},
    FeatureData, FEATURES,
};

pub struct Index {
    link: ComponentLink<Self>,
    current_search_terms: Vec<String>,
    current_search_results: Vec<FeatureData>,
    items_visible: usize,
    search_scores: Vec<(u16, f64)>,

    _scroll_task: ScrollTask,
    _resize_task: ResizeTask,
    _timeout_task: TimeoutTask,
}

pub enum Msg {
    Update,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub search_query: Rc<String>,
}

const BATCH_SIZE: usize = 12;

impl Component for Index {
    type Message = Msg;
    type Properties = Props;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let _scroll_task = ScrollService::new().register(link.callback(|_| Msg::Update));
        let _resize_task = ResizeService::new().register(link.callback(|_| Msg::Update));
        let _timeout_task =
            TimeoutService::spawn(Duration::from_secs(0), link.callback(|_| Msg::Update));

        Self {
            link,
            current_search_terms: Vec::new(),
            current_search_results: Vec::new(),
            items_visible: BATCH_SIZE,
            search_scores: vec![(0, 0.0); FEATURES.len()],

            _scroll_task,
            _resize_task,
            _timeout_task,
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
                    self._timeout_task = TimeoutService::spawn(
                        Duration::from_secs(0),
                        self.link.callback(|_| Msg::Update),
                    );

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
        self._timeout_task =
            TimeoutService::spawn(Duration::from_secs(0), self.link.callback(|_| Msg::Update));

        true
    }

    fn view(&self) -> Html {
        if self.current_search_terms.is_empty() {
            let list = FEATURES.iter().map(|&f| html! { <FeatureEntry data=f /> });
            html! { <div class="feature-list">{ for list.take(self.items_visible) }</div> }
        } else if self.current_search_results.is_empty() {
            html! { <div class="box muted">{"Nothing found, sorry."}</div> }
        } else {
            let list = self.current_search_results.iter().map(|&f| {
                html! { <FeatureEntry data=f /> }
            });

            html! { <div class="feature-list">{ for list.take(self.items_visible) }</div> }
        }
    }
}
