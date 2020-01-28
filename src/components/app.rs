use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::{
    components::{FullFeature, Index},
    features::FEATURES,
    util::Void,
    AppRoute,
};

pub struct App;

impl Component for App {
    type Message = Void;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        type Router = yew_router::router::Router<AppRoute>;
        let render_route = Router::render(|route| match route {
            AppRoute::Index => html! { <Index /> },
            AppRoute::Feature(slug) => match FEATURES.iter().find(|f| f.slug == slug) {
                Some(&data) => html! { <FullFeature data=data /> },
                None => html! { {"error: feature not found!"} },
            },
            AppRoute::Version(_number) => html! { {"error: not implemented yet"} },
        });

        html! {
            <Router render=render_route />
        }
    }
}
