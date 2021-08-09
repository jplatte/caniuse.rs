use std::rc::Rc;

use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, KeyboardEvent};
use yew::{
    html, Bridge, Bridged, Component, ComponentLink, Html, InputData, NodeRef, ShouldRender,
};
use yew_router::{
    agent::{RouteAgent, RouteRequest},
    route::Route,
};

use crate::{
    components::{About, ExtLinks, FeaturePage, Header, Index, VersionPage},
    util::document,
    AppRoute, FEATURES, VERSIONS,
};

pub struct App {
    link: ComponentLink<Self>,
    input_ref: NodeRef,
    router: Box<dyn Bridge<RouteAgent>>,
    search_query: Rc<String>,

    _key_listener: EventListener,
}

pub enum Msg {
    Update,
    FocusInput,
    Search(Rc<String>),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router = RouteAgent::bridge(link.callback(|_| Msg::Update));

        let link2 = link.clone();
        let _key_listener = EventListener::new(&document(), "keypress", move |event| {
            let event = event.dyn_ref::<KeyboardEvent>().expect("wrong event type");
            if event.key().as_str() == "s" {
                link2.send_message(Msg::FocusInput);
            }
        });

        Self {
            link,
            input_ref: NodeRef::default(),
            router,
            search_query: Rc::new(String::new()),
            _key_listener,
        }
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        match msg {
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

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        type Router = yew_router::router::Router<AppRoute>;
        let search_query = self.search_query.clone();
        let render_route = Router::render(move |route| match route {
            AppRoute::Index => html! { <Index search_query=search_query.clone() /> },
            AppRoute::About => html! { <About /> },
            AppRoute::Feature(slug) => match FEATURES.iter().find(|f| f.slug == slug) {
                Some(&data) => html! { <FeaturePage data=data /> },
                None => html! { "error: feature not found!" },
            },
            AppRoute::Version(number) => match VERSIONS.iter().find(|v| v.number == number) {
                Some(&data) => html! { <VersionPage data=data /> },
                None => html! { "error: version not found!" },
            },
        });

        html! {
            <>
                <Header input_ref=self.input_ref.clone()
                    oninput=self.link.callback(|e: InputData| Msg::Search(Rc::new(e.value))) />
                <ExtLinks />
                <div class="page">
                    <Router render=render_route />
                </div>
            </>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::FocusInput);
        }
    }
}
