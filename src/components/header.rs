use std::mem;

use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::{
    html, Bridge, Bridged, Callback, Component, ComponentLink, Html, InputData, NodeRef,
    Properties, ShouldRender,
};
use yew_router::agent::RouteAgent;

use crate::{
    icons::{fa_bars, fa_question_circle},
    services::click::{ClickService, ClickTask},
    util::{document, document_element, window},
    AppRoute, RouterAnchor,
};

pub struct Header {
    link: ComponentLink<Self>,
    props: Props,
    is_menu_open: bool,

    document_click_task: Option<ClickTask>,
    _router: Box<dyn Bridge<RouteAgent>>,
}

pub enum Msg {
    OpenMenu,
    CloseMenu,
    UpdateTheme(&'static str),
}

#[derive(Clone, Properties)]
pub struct Props {
    #[prop_or_default]
    pub input_ref: NodeRef,
    pub oninput: Callback<InputData>,
}

impl Component for Header {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Props, link: ComponentLink<Self>) -> Self {
        // Workaround for the RouterAnchor click handler unconditionally preventing the default
        // handler (and thus the click service handler) from running.
        let _router = RouteAgent::bridge(link.callback(|_| Msg::CloseMenu));
        Self { link, props, is_menu_open: false, document_click_task: None, _router }
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
            Msg::UpdateTheme(theme) => {
                document_element()
                    .dyn_into::<HtmlElement>()
                    .unwrap()
                    .dataset()
                    .set("theme", theme)
                    .unwrap();

                if let Ok(Some(st)) = window().local_storage() {
                    st.set_item("theme", theme).unwrap();
                }

                true
            }
        }
    }

    fn change(&mut self, props: Props) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
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

        let set_theme = |theme: &'static str| self.link.callback(move |_| Msg::UpdateTheme(theme));

        let root: HtmlElement = document_element().dyn_into().unwrap();
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
                            <li>
                                <RouterAnchor route=AppRoute::About>
                                    {"About"}{fa_question_circle()}
                                </RouterAnchor>
                            </li>
                        </ul>
                    </nav>
                </div>
            </header>
        }
    }
}
