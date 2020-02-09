use std::mem;

use stdweb::web::document;
use yew::{
    html, Bridge, Bridged, Callback, Component, ComponentLink, Html, InputData, NodeRef,
    Properties, ShouldRender,
};
use yew_router::{agent::RouteAgent, route::Route};

use crate::{
    icons::{fa_bars, fa_heart, Style},
    services::click::{ClickService, ClickTask},
    AppRoute,
};

pub struct Header {
    link: ComponentLink<Self>,
    props: Props,
    on_about_page: bool,
    is_menu_open: bool,

    document_click_task: Option<ClickTask>,
    _router: Box<dyn Bridge<RouteAgent>>,
}

pub enum Msg {
    OpenMenu,
    CloseMenu,
    UpdateAboutButton(Route),
}

#[derive(Clone, Properties)]
pub struct Props {
    pub input_ref: NodeRef,
    #[props(required)]
    pub oninput: Callback<InputData>,
}

impl Component for Header {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Props, link: ComponentLink<Self>) -> Self {
        let _router = RouteAgent::bridge(link.callback(Msg::UpdateAboutButton));
        Self {
            link,
            props,
            on_about_page: false,
            is_menu_open: false,
            document_click_task: None,
            _router,
        }
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        match msg {
            Msg::OpenMenu => {
                if !self.is_menu_open {
                    self.document_click_task = Some(
                        ClickService::new(document().body().unwrap().into())
                            .register(self.link.callback(|_| Msg::CloseMenu)),
                    );
                }

                !mem::replace(&mut self.is_menu_open, true)
            }
            Msg::CloseMenu => {
                if self.is_menu_open {
                    self.document_click_task = None;
                }

                mem::replace(&mut self.is_menu_open, false)
            }
            Msg::UpdateAboutButton(active_route) => {
                let mut on_about_page = active_route.as_str() == "/about";
                mem::swap(&mut on_about_page, &mut self.on_about_page);
                self.on_about_page != on_about_page
            }
        }
    }

    fn change(&mut self, props: Props) -> ShouldRender {
        self.props = props;
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

        let (menu_button, menu_classes) = if self.is_menu_open {
            (
                html! {
                    <button type="button" class="active">
                        {fa_bars()}
                    </button>
                },
                "menu active",
            )
        } else {
            (
                html! {
                    <button type="button" onclick=self.link.callback(|_| Msg::OpenMenu)>
                        {fa_bars()}
                    </button>
                },
                "menu",
            )
        };

        html! {
            <header>
                <div class="inner">
                    <div class="caniuse">
                        <label for="query">{"Can I use"}</label>
                        <input ref=self.props.input_ref.clone() id="query" type="search"
                            oninput=self.props.oninput.clone() />
                        {"?"}
                    </div>
                    <nav>
                        {about_button}
                        {menu_button}
                        <ul class={"menu ".to_owned() + menu_classes}>
                            <li class="toggle-nightmode">
                                {"Night mode"}<br />
                                <small><pre>{"unimplemented()"}</pre></small>
                            </li>
                        </ul>
                    </nav>
                </div>
            </header>
        }
    }
}
