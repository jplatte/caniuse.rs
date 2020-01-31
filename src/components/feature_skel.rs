use yew::{
    html, Children, Component, ComponentLink, Html, Properties, Renderable as _, ShouldRender,
};

use crate::{
    components::SupportIndicator,
    search::Span,
    util::{view_text_with_matches, Void},
    AppRoute, FeatureData,
};

#[derive(Clone, Properties)]
pub struct Props {
    #[props(required)]
    pub children: Children,
    #[props(required)]
    pub feature: FeatureData,
    pub title_matches: Vec<Span>,
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
        type RouterAnchor = yew_router::components::RouterAnchor<AppRoute>;
        let f = self.props.feature;

        html! {
            <li class="feature-box">
                <div class="feature">
                    <h3 class="title">
                        <RouterAnchor route=AppRoute::Feature(f.slug.into())>
                            {view_text_with_matches(f.title, &self.props.title_matches)}
                        </RouterAnchor>
                    </h3>
                    { self.props.children.render() }
                </div>
                <SupportIndicator
                    channel=f.channel
                    version=f.version
                    // ctx=support_ctx
                    />
            </li>
        }
    }
}
