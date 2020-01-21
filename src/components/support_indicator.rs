use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::util::Void;

pub struct SupportIndicator {
    props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub version: &'static str,
}

impl Component for SupportIndicator {
    type Message = Void;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, void: Self::Message) -> ShouldRender {
        match void {}
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        if self.props.version == "nightly" {
            html! { <div class="version unsupported">{"Rust "}{self.props.version}</div> }
        } else {
            html! { <div class="version supported">{"Rust "}{self.props.version}</div> }
        }
    }
}
