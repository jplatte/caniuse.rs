use yew::{
    html, Children, Component, ComponentLink, Html, Properties, Renderable as _, ShouldRender,
};

use crate::{components::SupportIndicator, util::Void};

#[derive(Clone, Properties)]
pub struct Props {
    pub children: Children,
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
        html! {
            <li class="feature-box">
                <div class="feature">
                    <h3 class="title">{self.props.title.clone()}</h3>
                    { self.props.children.render() }
                </div>
                <SupportIndicator version=self.props.version /*ctx=support_ctx*/ />
            </li>
        }
    }
}
