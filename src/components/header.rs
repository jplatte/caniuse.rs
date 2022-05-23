use std::mem;

use gloo::{
    events::EventListener,
    utils::{body, document_element, window},
};
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, InputEvent, MouseEvent};
use yew::{html, Callback, Component, Context, Html, NodeRef, Properties};
use yew_router::{
    history::{History, Location},
    scope_ext::RouterScopeExt,
};

use crate::{
    icons::{fa_bars, fa_moon, fa_question_circle, fa_sun},
    AppRoute, RouterLink,
};

pub struct Header {
    is_menu_open: bool,
    document_click_listener: Option<EventListener>,
}

pub enum Msg {
    OpenMenu,
    CloseMenu,
    UpdateTheme(&'static str),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub input_ref: NodeRef,
    pub oninput: Callback<InputEvent>,
}

impl Component for Header {
    type Message = Msg;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self { is_menu_open: false, document_click_listener: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Msg) -> bool {
        match msg {
            Msg::OpenMenu => {
                if !self.is_menu_open {
                    self.document_click_listener = Some(EventListener::new(&body(), "click", {
                        let link = ctx.link().clone();
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

    fn changed(&mut self, _: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
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
            let open_menu = ctx.link().callback(|ev: MouseEvent| {
                ev.stop_propagation();
                Msg::OpenMenu
            });

            (
                html! {
                    <button type="button" onclick={open_menu}>
                        {fa_bars()}
                    </button>
                },
                "menu",
            )
        };

        let set_theme = |theme: &'static str| ctx.link().callback(move |_| Msg::UpdateTheme(theme));

        let root: HtmlElement = document_element().dyn_into().unwrap();
        let theme_anchor = if root.dataset().get("theme").unwrap() == "dark" {
            html! {
                <a onclick={set_theme("light")}>{fa_sun()}{"Light theme"}</a>
            }
        } else {
            html! {
                <a onclick={set_theme("dark")}>{fa_moon()}{"Dark theme"}</a>
            }
        };

        let history = ctx.link().history().unwrap();
        let cb = ctx.props().oninput.clone();
        let oninput = move |ev| {
            if history.location().route::<AppRoute>() != Some(AppRoute::Index) {
                history.push(AppRoute::Index);
            }
            cb.emit(ev);
        };

        html! {
            <header>
                <div class="inner">
                    <div class="caniuse">
                        <label for="query">{"Can I use"}</label>
                        <input ref={ctx.props().input_ref.clone()} id="query" type="search"
                            oninput={oninput} />
                        {"?"}
                    </div>
                    <nav aria-label="Site navigation">
                        {menu_button}
                        <ul class={"menu ".to_owned() + menu_classes}>
                            <li>{theme_anchor}</li>
                            <li>
                                <RouterLink to={AppRoute::About}>
                                    {fa_question_circle()}{"About"}
                                </RouterLink>
                            </li>
                        </ul>
                    </nav>
                </div>
            </header>
        }
    }
}
