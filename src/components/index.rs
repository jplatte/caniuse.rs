use std::rc::Rc;

use gloo::{events::EventListener, timers::callback::Timeout};
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::{
    components::FeatureEntry,
    search::{extract_search_terms, run_search, InvalidSearchQuery},
    util::{document_body, window},
    AppRoute, Channel, FeatureData, RouterAnchor, VersionData, FEATURES,
};

pub struct Index {
    link: ComponentLink<Self>,
    show: ContentsToRender,
    current_search_terms: Vec<String>,
    current_search_results: Vec<FeatureData>,
    items_visible: usize,
    search_scores: Vec<(u16, f64)>,

    _scroll_listener: EventListener,
    _resize_listener: EventListener,
    _timeout: Timeout,
}

enum ContentsToRender {
    Explore(Explore),
    SearchResults,
    EmptySearchResults,
    InvalidSearchResults,
}

pub enum Msg {
    Update,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub show: IndexContents,
}

#[derive(Clone)]
pub enum IndexContents {
    Explore(Explore),
    SearchResults { search_query: Rc<String> },
}

#[derive(Clone, Copy, PartialEq)]
pub enum Explore {
    Stable,
    RecentlyStabilized,
    Unstable,
}

const BATCH_SIZE: usize = 20;

impl Component for Index {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let _scroll_listener = EventListener::new(&window(), "scroll", {
            let link = link.clone();
            move |_| link.send_message(Msg::Update)
        });
        let _resize_listener = EventListener::new(&window(), "resize", {
            let link = link.clone();
            move |_| link.send_message(Msg::Update)
        });
        let _timeout = create_timeout(link.clone());

        let mut current_search_terms = Vec::new();
        let mut current_search_results = Vec::new();
        let mut search_scores = vec![(0, 0.0); FEATURES.len()];
        let show =
            show(props, &mut current_search_terms, &mut current_search_results, &mut search_scores);

        Self {
            link,
            show,
            current_search_terms,
            current_search_results,
            items_visible: BATCH_SIZE,
            search_scores,

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
        self.show = show(
            props,
            &mut self.current_search_terms,
            &mut self.current_search_results,
            &mut self.search_scores,
        );

        self.items_visible = BATCH_SIZE;
        self._timeout = create_timeout(self.link.clone());

        true
    }

    fn view(&self) -> Html {
        match &self.show {
            ContentsToRender::Explore(ex) => {
                let list = FEATURES
                    .iter()
                    .filter(|f| {
                        matches!(
                            (ex, f.version),
                            (Explore::Stable, Some(VersionData { channel: Channel::Stable, .. }))
                                | (
                                    Explore::RecentlyStabilized,
                                    Some(VersionData {
                                        channel: Channel::Beta | Channel::Nightly,
                                        ..
                                    }),
                                )
                                | (Explore::Unstable, None)
                        )
                    })
                    .map(|&f| html! { <FeatureEntry key=f.slug data=f /> });

                let index_link_class = active_if(*ex == Explore::Stable);
                let recent_link_class = active_if(*ex == Explore::RecentlyStabilized);
                let unstable_link_class = active_if(*ex == Explore::Unstable);

                html! {
                    <>
                        <nav class="explore">
                            <div class="inner">
                                <RouterAnchor route=AppRoute::Index classes=index_link_class>
                                    {"Stable"}
                                </RouterAnchor>
                                <RouterAnchor route=AppRoute::RecentlyStabilized
                                    classes=recent_link_class>
                                    {"Recently Stabilized"}
                                </RouterAnchor>
                                <RouterAnchor route=AppRoute::Unstable classes=unstable_link_class>
                                    {"Unstable"}
                                </RouterAnchor>
                            </div>
                        </nav>
                        <div class="feature-list">{ for list.take(self.items_visible) }</div>
                    </>
                }
            }
            ContentsToRender::SearchResults => {
                let list = self.current_search_results.iter().map(|&f| {
                    html! { <FeatureEntry key=f.slug data=f /> }
                });

                html! { <div class="feature-list">{ for list.take(self.items_visible) }</div> }
            }
            ContentsToRender::EmptySearchResults => {
                html! { <div class="box muted">{"Nothing found, sorry."}</div> }
            }
            ContentsToRender::InvalidSearchResults => {
                html! { <div class="box muted">{"Invalid search terms."}</div> }
            }
        }
    }
}

fn show(
    props: Props,
    current_search_terms: &mut Vec<String>,
    current_search_results: &mut Vec<FeatureData>,
    search_scores: &mut Vec<(u16, f64)>,
) -> ContentsToRender {
    match props.show {
        IndexContents::Explore(ex) => ContentsToRender::Explore(ex),
        IndexContents::SearchResults { search_query } => {
            match extract_search_terms(&search_query) {
                Ok(search_terms) => {
                    *current_search_results = run_search(&search_terms, search_scores);
                    *current_search_terms = search_terms;

                    if current_search_results.is_empty() {
                        ContentsToRender::EmptySearchResults
                    } else {
                        ContentsToRender::SearchResults
                    }
                }
                Err(InvalidSearchQuery) => ContentsToRender::InvalidSearchResults,
            }
        }
    }
}

fn active_if(cond: bool) -> String {
    if cond {
        "active".to_owned()
    } else {
        String::new()
    }
}

// Creates a timeout that lets the browser render the page before calling `fn update()`.
fn create_timeout(link: ComponentLink<Index>) -> Timeout {
    Timeout::new(0, move || link.send_message(Msg::Update))
}
