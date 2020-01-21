use yew::{
    html, Children, Component, ComponentLink, Html, Properties, Renderable as _, ShouldRender,
};

use crate::components::SupportIndicator;

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
