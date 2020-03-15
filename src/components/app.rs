use wasm_bindgen::JsValue;
use web_sys::{console, HtmlElement, KeyboardEvent};
use yew::{
    format::{Json, Nothing},
    html,
    services::{
        fetch::{FetchService, FetchTask, Request, Response},
        keyboard::{KeyListenerHandle, KeyboardService},
    },
    Bridge, Bridged, Component, ComponentLink, Html, InputData, NodeRef, ShouldRender,
};
use yew_router::{
    agent::{RouteAgent, RouteRequest},
    route::Route,
};

use crate::{
    components::{About, ExtLinks, FeaturePage, Header, Index, VersionPage},
    data2::FeatureToml,
    icons::fa_circle_notch,
    util::document,
    AppRoute, VERSIONS,
};

pub struct App {
    link: ComponentLink<Self>,
    input_ref: NodeRef,
    router: Box<dyn Bridge<RouteAgent>>,
    data: Option<FeatureToml>,
    search_query: String,

    _key_listener_handle: KeyListenerHandle,
    _fetch_task: FetchTask,
}

pub enum Msg {
    SetData(FeatureToml),
    Update,
    FocusInput,
    Search(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router = RouteAgent::bridge(link.callback(|_| Msg::Update));

        let link_clone = link.clone();
        let _key_listener_handle = KeyboardService::register_key_press(
            &document(),
            (move |e: KeyboardEvent| {
                if e.key().as_str() == "s" {
                    link_clone.send_message(Msg::FocusInput);
                }
            })
            .into(),
        );

        let link_clone = link.clone();
        let _fetch_task = FetchService::new()
            .fetch(
                Request::get("features.toml").body(Nothing).unwrap(),
                (move |response: Response<Json<anyhow::Result<FeatureToml>>>| match response
                    .into_body()
                    .0
                {
                    Ok(data) => link_clone.send_message(Msg::SetData(data)),
                    Err(error) => {}, console::error_2(
                        &JsValue::from_str("Invalid data file!"),
                        &JsValue::from_str(&error.to_string()),
                    ),
                })
                .into(),
            )
            .unwrap();

        Self {
            link,
            input_ref: NodeRef::default(),
            router,
            search_query: String::new(),
            data: None,
            _key_listener_handle,
            _fetch_task,
        }
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        match msg {
            Msg::SetData(data) => {
                self.data = Some(data);
                true
            }
            Msg::Update => true,
            Msg::FocusInput => {
                self.input_ref.cast::<HtmlElement>().unwrap().focus().unwrap();
                false
            }
            Msg::Search(query) => {
                self.search_query = query;
                self.router.send(RouteRequest::ChangeRoute(Route::new_no_state("/")));

                // Re-render after routing, through Msg::Update
                false
            }
        }
    }

    fn view(&self) -> Html {
        type Router = yew_router::router::Router<AppRoute>;
        let search_query = self.search_query.clone();
        let data = self.data.clone();
        let render_route = Router::render(move |route| match route {
            AppRoute::Index => with_data(&data, |data| {
                html! {
                    <Index data=data.clone() search_query=search_query.clone() />
                }
            }),
            AppRoute::About => html! { <About /> },
            AppRoute::Feature(slug) => {
                with_data(&data, |data| match data.features().find(|f| f.1.slug() == slug) {
                    Some((v, f)) => html! { <FeaturePage feature=f.clone() version=v.clone() /> },
                    None => html! { "error: feature not found!" },
                })
            }
            AppRoute::Version(number) => match VERSIONS.iter().find(|v| v.number == number) {
                Some(&data) => html! { <VersionPage data=data /> },
                None => html! { "error: version not found!" },
            },
        });

        html! {
            <>
                <Header input_ref=self.input_ref.clone()
                    oninput=self.link.callback(|e: InputData| Msg::Search(e.value)) />
                <ExtLinks />
                <div class="page">
                    <Router render=render_route />
                </div>
            </>
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.link.send_message(Msg::FocusInput);
        false
    }
}

fn with_data(data: &Option<FeatureToml>, render: impl FnOnce(&FeatureToml) -> Html) -> Html {
    match &data {
        Some(data) => render(data),
        None => html! {
            <div class="loading-indicator">
                {fa_circle_notch()}<span>{"Loading…"}</span>
            </div>
        },
    }
}
