use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::{
    components::{FullFeature, Index},
    features::FEATURES,
    util::Void,
    AppRoute,
};

struct App;

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
            AppRoute::Feature(name) => {
                match FEATURES.iter().find(|f| f.flag.map(|flag| flag == name).unwrap_or(false)) {
                    Some(&data) => html! { <FullFeature data=data /> },
                    None => html! { {"error: feature not found!"} },
                }
            }
        });

        html! {
            <Router render=render_route />
        }
    }
}
