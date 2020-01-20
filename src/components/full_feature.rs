use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::{components::FeatureSkel, FeatureData};

#[derive(Clone, Properties)]
pub struct Props {
    pub data: Option<FeatureData>,
}

pub struct FullFeature {
    props: Props,
}

impl Component for FullFeature {
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

        let title = html! { {f.title} };

        let maybe_flag = match f.flag {
            Some(f) => html! {
                <div class="flag">{"Feature flag: "}{f}</div>
            },
            None => html! {},
        };

        // TODO: maybe_impl_pr_link, maybe_stabilization_pr_link

        let items = if f.items.is_empty() {
            html! {}
        } else {
            html! { {view_items(f.items)} }
        };

        html! {
            <FeatureSkel title=title>
                {maybe_flag}
                <span class="version stable">{"Rust "}{f.version}</span>
                {items}
            </FeatureSkel>
        }
    }
}

fn view_items(items: &[&str]) -> Html {
    let mut items = items.iter().map(|i| html! { <li><code>{i}</code></li> });
    html! {
        <>
            {"Items"}
            <ul>
                { for items }
            </ul>
        </>
    }
}
