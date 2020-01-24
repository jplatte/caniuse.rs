use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::{
    components::FeatureSkel,
    util::{view_text, Void},
    AppRoute, FeatureData,
};

#[derive(Clone, Properties)]
pub struct Props {
    #[props(required)]
    pub data: FeatureData,
}

pub struct Feature {
    props: Props,
}

impl Component for Feature {
    type Message = Void;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, void: Self::Message) -> ShouldRender {
        match void {}
    }

    fn view(&self) -> Html {
        type RouterAnchor = yew_router::components::RouterAnchor<AppRoute>;

        let f = &self.props.data;
        let title = html! { {view_text(f.title)} };

        let maybe_flag = match f.flag {
            Some(flag) if f.version == "nightly" => {
                html! { <div class="flag">{"Feature flag: "}{view_text(flag)}</div> }
            }
            _ => html! {},
        };

        let items = if f.items.is_empty() {
            html! {}
        } else {
            html! { {view_items(f.items)} }
        };

        let maybe_details = match f.flag {
            Some(flag) => html! {
                <RouterAnchor route=AppRoute::Feature(flag.into())>
                    {"Details"}
                </RouterAnchor>
            },
            None => html! {},
        };

        html! {
            <FeatureSkel title=title version=f.version>
                {maybe_flag}
                {items}
                {maybe_details}
            </FeatureSkel>
        }
    }
}

fn view_items(items: &[&str]) -> Html {
    let mut items = items.iter().map(|i| html! { <li><code>{i}</code></li> });
    html! {
        <details>
            <summary>{"Items"}</summary>
            <ul>
                { for items }
            </ul>
        </details>
    }
}
