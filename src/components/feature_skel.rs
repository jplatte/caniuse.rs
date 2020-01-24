use yew::{
    html, Children, Component, ComponentLink, Html, Properties, Renderable as _, ShouldRender,
};

use crate::{components::SupportIndicator, util::Void};

#[derive(Clone, Properties)]
pub struct Props {
    #[props(required)]
    pub children: Children,
    #[props(required)]
    pub title: Html,
    pub version: &'static str,
}

pub struct FeatureSkel {
    props: Props,
}

impl Component for FeatureSkel {
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
        let maybe_support_indicator = if self.props.version.is_empty() {
            html! {}
        } else {
            html! { <SupportIndicator version=self.props.version /*ctx=support_ctx*/ /> }
        };

        html! {
            <li class="feature-box">
                <div class="feature">
                    <h3 class="title">{self.props.title.clone()}</h3>
                    { self.props.children.render() }
                </div>
                {maybe_support_indicator}
            </li>
        }
    }
}
