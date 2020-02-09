use yew::{html, Callback, Component, ComponentLink, Html, InputData, Properties, ShouldRender};

use crate::util::Void;

pub struct Header {
    pub oninput: Callback<InputData>,
}

#[derive(Clone, Properties)]
pub struct Props {
    #[props(required)]
    pub oninput: Callback<InputData>,
}

impl Component for Header {
    type Message = Void;
    type Properties = Props;

    fn create(props: Props, _: ComponentLink<Self>) -> Self {
        Self { oninput: props.oninput }
    }

    fn update(&mut self, msg: Void) -> ShouldRender {
        match msg {}
    }

    fn change(&mut self, props: Props) -> ShouldRender {
        self.oninput = props.oninput;
        true
    }

    fn view(&self) -> Html {
        html! {
            <header>
                <div class="caniuse">
                    <label for="query">{"Can I use"}</label>
                    <input id="query" type="search" oninput=self.oninput.clone() />
                    {"?"}
                </div>
            </header>
        }
    }
}
