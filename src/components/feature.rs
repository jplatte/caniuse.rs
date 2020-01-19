use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::{components::FeatureSkel, util::view_text, FeatureData};

#[derive(Clone, Properties)]
pub struct Props {
    pub data: Option<FeatureData>,
}

pub struct Feature {
    props: Props,
}

impl Component for Feature {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let f = match self.props.data {
            Some(data) => data,
            None => return html! {}, // meh
        };

        let desc = html! { {view_text(f.desc_short)} };

        let maybe_flag = match f.flag {
            Some(f) => html! {
                <div class="flag">{"Feature flag: "}{view_text(f)}</div>
            },
            None => html! {},
        };

        let items = if f.items.is_empty() {
            html! {}
        } else {
            html! { {view_items(f.items)} }
        };

        html! {
            <FeatureSkel desc=desc>
                {maybe_flag}
                <span class="version stable">{"Rust "}{f.version}</span>
                {items}
            </FeatureSkel>
        }
    }
}

fn view_items(items: &[&str]) -> Html {
    let mut items = items.iter().map(|i| html! { <li>{i}</li> });
    html! {
        <details>
            <summary>{"Items"}</summary>
            <ul>
                { for items }
            </ul>
        </details>
    }
}
