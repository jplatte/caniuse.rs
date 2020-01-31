use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::{util::Void, Channel};

pub struct SupportIndicator {
    props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    #[props(required)]
    pub channel: Channel,
    #[props(required)]
    pub version: Option<&'static str>,
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
        match self.props.version {
            None => html! { <div class="version none">{"Unstable"}</div> },
            Some(v) => {
                let classes = match self.props.channel {
                    Channel::Nightly => "version nightly",
                    Channel::Beta => "version beta",
                    Channel::Stable => "version stable",
                };

                html! { <div class=classes>{"Rust "}{v}</div> }
            }
        }
    }
}
