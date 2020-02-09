use yew::{html, Bridge, Bridged, Component, ComponentLink, Html, InputData, ShouldRender};
use yew_router::{
    agent::{RouteAgent, RouteRequest},
    route::Route,
};

use crate::{
    components::{FeaturePage, Header, Index},
    features::FEATURES,
    AppRoute,
};

pub struct App {
    link: ComponentLink<Self>,
    router: Box<dyn Bridge<RouteAgent>>,
    search_query: String,
}

pub enum Msg {
    Update,
    Search(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router = RouteAgent::bridge(link.callback(|_| Msg::Update));
        Self { link, router, search_query: String::new() }
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        match msg {
            Msg::Update => true,
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
        let render_route = Router::render(move |route| match route {
            AppRoute::Index => html! { <Index search_query=search_query.clone() /> },
            AppRoute::About => html! { {"Hello world!"} },
            AppRoute::Feature(slug) => match FEATURES.iter().find(|f| f.slug == slug) {
                Some(&data) => html! { <FeaturePage data=data /> },
                None => html! { {"error: feature not found!"} },
            },
            AppRoute::Version(_number) => html! { {"error: not implemented yet"} },
        });

        html! {
            <>
                <Header oninput=self.link.callback(|e: InputData| Msg::Search(e.value)) />
                <Router render=render_route />
            </>
        }
    }
}
