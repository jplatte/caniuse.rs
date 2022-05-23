use std::rc::Rc;

use gloo::{events::EventListener, timers::callback::Timeout, utils::window};
use yew::{html, html::Scope, Classes, Component, Context, Html, Properties};

use crate::{
    components::FeatureEntry,
    search::{extract_search_terms, run_search, InvalidSearchQuery},
    AppRoute, Channel, FeatureData, RouterLink, FEATURES,
};

pub struct Index {
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

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub show: IndexContents,
}

#[derive(Clone, PartialEq)]
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

    fn create(ctx: &Context<Self>) -> Self {
        let _scroll_listener = EventListener::new(&window(), "scroll", {
            let link = ctx.link().clone();
            move |_| link.send_message(Msg::Update)
        });
        let _resize_listener = EventListener::new(&window(), "resize", {
            let link = ctx.link().clone();
            move |_| link.send_message(Msg::Update)
        });
        let _timeout = create_timeout(ctx.link().clone());

        let mut current_search_terms = Vec::new();
        let mut current_search_results = Vec::new();
        let mut search_scores = vec![(0, 0.0); FEATURES.len()];
        let show = show(
            ctx.props(),
            &mut current_search_terms,
            &mut current_search_results,
            &mut search_scores,
        );

        Self {
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

    fn update(&mut self, ctx: &Context<Self>, msg: Msg) -> bool {
        match msg {
            Msg::Update => {
                if self.items_visible < self.current_search_results.len() {
                    self.items_visible += BATCH_SIZE;
                    self._timeout = create_timeout(ctx.link().clone());

                    true
                } else {
                    false
                }
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.show = show(
            ctx.props(),
            &mut self.current_search_terms,
            &mut self.current_search_results,
            &mut self.search_scores,
        );

        self.items_visible = BATCH_SIZE;
        self._timeout = create_timeout(ctx.link().clone());

        true
    }

    fn view(&self, _: &Context<Self>) -> Html {
        match &self.show {
            ContentsToRender::Explore(ex) => {
                // Stack slots, to be able to dynamically dispatch to one of
                // the iterators without an extra allocation
                let (mut s1, mut s2, mut s3);

                // Features are sorted by version, with unstable ones being at
                // the very end, which allows using skip_while + take_while
                // instead of filter to simplify iteration
                let features: &mut dyn Iterator<Item = &FeatureData> = match ex {
                    Explore::Stable => {
                        s1 = FEATURES
                            .iter()
                            .skip_while(|f| !f.is_on_channel(Channel::Stable))
                            .take_while(|f| f.is_on_channel(Channel::Stable));

                        &mut s1
                    }
                    Explore::RecentlyStabilized => {
                        s2 = FEATURES
                            .iter()
                            .skip_while(|f| f.is_on_channel(Channel::Nightly))
                            .take_while(|f| f.is_on_channel(Channel::Beta))
                            .chain(
                                FEATURES.iter().take_while(|f| f.is_on_channel(Channel::Nightly)),
                            );

                        &mut s2
                    }
                    Explore::Unstable => {
                        s3 = FEATURES.iter().skip_while(|f| f.version.is_some());

                        &mut s3
                    }
                };

                let index_link_class = active_if(*ex == Explore::Stable);
                let recent_link_class = active_if(*ex == Explore::RecentlyStabilized);
                let unstable_link_class = active_if(*ex == Explore::Unstable);

                let list = features
                    .take(self.items_visible)
                    .map(|&f| html! { <FeatureEntry key={f.slug} data={f} /> });

                html! {
                    <>
                        <nav class="explore">
                            <div class="inner">
                                <RouterLink to={AppRoute::Index} classes={index_link_class}>
                                    {"Stable"}
                                </RouterLink>
                                <RouterLink to={AppRoute::RecentlyStabilized}
                                    classes={recent_link_class}>
                                    {"Recently Stabilized"}
                                </RouterLink>
                                <RouterLink to={AppRoute::Unstable} classes={unstable_link_class}>
                                    {"Unstable"}
                                </RouterLink>
                            </div>
                        </nav>
                        <div class="feature-list">{ for list }</div>
                    </>
                }
            }
            ContentsToRender::SearchResults => {
                let list = self.current_search_results.iter().take(self.items_visible).map(|&f| {
                    html! { <FeatureEntry key={f.slug} data={f} /> }
                });

                html! { <div class="feature-list">{ for list }</div> }
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
    props: &Props,
    current_search_terms: &mut Vec<String>,
    current_search_results: &mut Vec<FeatureData>,
    search_scores: &mut Vec<(u16, f64)>,
) -> ContentsToRender {
    match &props.show {
        IndexContents::Explore(ex) => ContentsToRender::Explore(*ex),
        IndexContents::SearchResults { search_query } => match extract_search_terms(search_query) {
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
        },
    }
}

fn active_if(cond: bool) -> Classes {
    if cond {
        "active".into()
    } else {
        Classes::new()
    }
}

// Creates a timeout that lets the browser render the page before calling `fn update()`.
fn create_timeout(scope: Scope<Index>) -> Timeout {
    Timeout::new(0, move || scope.send_message(Msg::Update))
}
