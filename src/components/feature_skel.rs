use yew::{
    html, Children, Component, ComponentLink, Html, Properties, Renderable as _, ShouldRender,
};

#[derive(Clone, Properties)]
pub struct Props {
    pub children: Children,
    pub desc: Html,
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
        true
    }

    fn view(&self) -> Html {
        html! {
            <li class="feature-box">
                <div class="feature">
                    <h3 class="desc">{self.props.desc.clone()}</h3>
                    { self.props.children.render() }
                </div>
            </li>
        }
    }
}
