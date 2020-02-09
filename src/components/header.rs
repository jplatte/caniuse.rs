use std::mem;

use yew::{
    html, Bridge, Bridged, Callback, Component, ComponentLink, Html, InputData, Properties,
    ShouldRender,
};
use yew_router::{agent::RouteAgent, route::Route};

use crate::{
    icons::{fa_bars, fa_heart, Style},
    AppRoute,
};

pub struct Header {
    oninput: Callback<InputData>,
    on_about_page: bool,

    _router: Box<dyn Bridge<RouteAgent>>,
}

pub enum Msg {
    OpenMenu,
    CloseMenu,
    UpdateAboutButton(Route),
}

#[derive(Clone, Properties)]
pub struct Props {
    #[props(required)]
    pub oninput: Callback<InputData>,
}

impl Component for Header {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Props, link: ComponentLink<Self>) -> Self {
        let _router = RouteAgent::<()>::bridge(link.callback(Msg::UpdateAboutButton));
        Self { oninput: props.oninput, on_about_page: false, _router }
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        match msg {
            Msg::OpenMenu => {
                // TODO
                false
            }
            Msg::CloseMenu => {
                // TODO
                false
            }
            Msg::UpdateAboutButton(active_route) => {
                let mut on_about_page = active_route.as_str() == "/about";
                mem::swap(&mut on_about_page, &mut self.on_about_page);
                self.on_about_page != on_about_page
            }
        }
    }

    fn change(&mut self, props: Props) -> ShouldRender {
        self.oninput = props.oninput;
        true
    }

    fn view(&self) -> Html {
        type RouterButton = yew_router::components::RouterButton<AppRoute>;
        let about_button = if self.on_about_page {
            html! {
                <RouterButton route=AppRoute::Index classes="active">
                    {fa_heart(Style::Solid)}
                </RouterButton>
            }
        } else {
            html! {
                <RouterButton route=AppRoute::About>
                    {fa_heart(Style::Regular)}
                </RouterButton>
            }
        };

        html! {
            <header>
                <div class="inner">
                    <div class="caniuse">
                        <label for="query">{"Can I use"}</label>
                        <input id="query" type="search" oninput=self.oninput.clone() />
                        {"?"}
                    </div>
                    <nav>
                        {about_button}
                        <button type="button">{fa_bars()}</button>
                    </nav>
                </div>
            </header>
        }
    }
}
