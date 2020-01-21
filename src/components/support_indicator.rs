use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct SupportIndicator {
    props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub version: &'static str,
}

impl Component for SupportIndicator {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
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
