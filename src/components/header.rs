use std::mem;

use stdweb::{
    js,
    unstable::TryInto,
    web::{document, HtmlElement, IHtmlElement},
};
use yew::{
    events::ClickEvent, html, Bridge, Bridged, Callback, Component, ComponentLink, Html, InputData,
    NodeRef, Properties, ShouldRender,
};
use yew_router::{agent::RouteAgent, route::Route};

use crate::{
    icons::{fa_bars, fa_heart, Style},
    services::click::{ClickService, ClickTask},
    AppRoute, RouterButton,
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
    UpdateTheme(&'static str),
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
            Msg::UpdateTheme(theme) => {
                js! {
                    const theme = @{theme};
                    document.documentElement.dataset.theme = theme;
                    localStorage.setItem("theme", theme);
                };

                true
            }
        }
    }

    fn change(&mut self, props: Props) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
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

        let set_theme =
            |theme: &'static str| self.link.callback(move |_: ClickEvent| Msg::UpdateTheme(theme));

        let root: HtmlElement = document().document_element().unwrap().try_into().unwrap();
        let (light_btn_class, dark_btn_class) = if root.dataset().get("theme").unwrap() == "dark" {
            ("", "active")
        } else {
            ("active", "")
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
                            <li class="theme-select">
                                <span>{"Theme"}</span>
                                <button class=light_btn_class onclick=set_theme("light")>
                                    {"Light"}
                                </button>
                                <button class=dark_btn_class onclick=set_theme("dark")>
                                    {"Dark"}
                                </button>
                            </li>
                        </ul>
                    </nav>
                </div>
            </header>
        }
    }
}
