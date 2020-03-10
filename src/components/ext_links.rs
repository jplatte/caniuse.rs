use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::{
    icons::{fa_angle_left, fa_angle_right, fa_circle, fa_github, liberapay_icon},
    util::window,
};

pub struct ExtLinks {
    visible: bool,
    link: ComponentLink<Self>,
}

#[derive(Clone, Copy, Debug)]
pub enum Msg {
    Show,
    Hide,
}

impl Component for ExtLinks {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let visible = window()
            .local_storage()
            .ok()
            .flatten()
            .and_then(|st| st.get_item("showExtLinks").ok().flatten())
            .and_then(|val| val.parse().ok())
            .unwrap_or(false);

        Self { visible, link }
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        match msg {
            Msg::Show => self.visible = true,
            Msg::Hide => self.visible = false,
        }

        if let Ok(Some(st)) = window().local_storage() {
            st.set_item("showExtLinks", &format!("{:?}", self.visible)).unwrap();
        }

        true
    }

    fn view(&self) -> Html {
        if self.visible {
            let onclick = self.link.callback(|_| Msg::Hide);

            html! {
                <div class="ext-links">
                    <div class="inner">
                        <a href="https://github.com/jplatte/caniuse.rs">
                            {fa_github()}{"GitHub"}
                        </a>
                        <a href="https://git.sr.ht/~jplatte/caniuse.rs">
                            {fa_circle()}{"sourcehut"}
                        </a>
                        <a href="https://liberapay.com/jplatte/">
                            {liberapay_icon()}{"Liberapay"}
                        </a>
                        <button onclick=onclick>{fa_angle_right()}</button>
                    </div>
                </div>
            }
        } else {
            let onclick = self.link.callback(|_| Msg::Show);

            html! {
                <div class="ext-links closed">
                    <button onclick=onclick>{fa_angle_left()}</button>
                </div>
            }
        }
    }
}
