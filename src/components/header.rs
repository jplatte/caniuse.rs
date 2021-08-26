use std::mem;

use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, MouseEvent};
use yew::{
    html, Callback, Component, ComponentLink, Html, InputData, NodeRef, Properties, ShouldRender,
};

use crate::{
    icons::{fa_bars, fa_moon, fa_question_circle, fa_sun},
    util::{document_body, document_element, window},
    AppRoute, RouterAnchor,
};

pub struct Header {
    link: ComponentLink<Self>,
    props: Props,
    is_menu_open: bool,

    document_click_listener: Option<EventListener>,
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
        Self { link, props, is_menu_open: false, document_click_listener: None }
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        match msg {
            Msg::OpenMenu => {
                if !self.is_menu_open {
                    self.document_click_listener =
                        Some(EventListener::new(&document_body(), "click", {
                            let link = self.link.clone();
                            move |_| link.send_message(Msg::CloseMenu)
                        }));
                }

                !mem::replace(&mut self.is_menu_open, true)
            }
            Msg::CloseMenu => {
                if self.is_menu_open {
                    self.document_click_listener = None;
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
                    <button type="button" class="active" aria-hidden="true">
                        {fa_bars()}
                    </button>
                },
                "menu active",
            )
        } else {
            let open_menu = self.link.callback(|ev: MouseEvent| {
                ev.stop_propagation();
                Msg::OpenMenu
            });

            (
                html! {
                    <button type="button" onclick=open_menu>
                        {fa_bars()}
                    </button>
                },
                "menu",
            )
        };

        let set_theme = |theme: &'static str| self.link.callback(move |_| Msg::UpdateTheme(theme));

        let root: HtmlElement = document_element().dyn_into().unwrap();
        let theme_anchor = if root.dataset().get("theme").unwrap() == "dark" {
            html! {
                <a onclick=set_theme("light")>{fa_sun()}{"Light theme"}</a>
            }
        } else {
            html! {
                <a onclick=set_theme("dark")>{fa_moon()}{"Dark theme"}</a>
            }
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
                    <nav aria-label="Site navigation">
                        {menu_button}
                        <ul class={"menu ".to_owned() + menu_classes}>
                            <li>{theme_anchor}</li>
                            <li>
                                <RouterAnchor route=AppRoute::About>
                                    {fa_question_circle()}{"About"}
                                </RouterAnchor>
                            </li>
                        </ul>
                    </nav>
                </div>
            </header>
        }
    }
}
