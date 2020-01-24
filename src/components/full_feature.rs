use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::{
    components::FeatureSkel,
    util::{view_text, Void},
    FeatureData,
};

#[derive(Clone, Properties)]
pub struct Props {
    #[props(required)]
    pub data: FeatureData,
}

pub struct FullFeature {
    props: Props,
}

impl Component for FullFeature {
    type Message = Void;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, void: Self::Message) -> ShouldRender {
        match void {}
    }

    fn view(&self) -> Html {
        let f = &self.props.data;
        let title = html! { {f.title} };

        let maybe_flag = match f.flag {
            Some(flag) => {
                let text = html! { <>{"Feature flag: "}{view_text(flag)}</> };
                if f.version == "nightly" {
                    html! { <div class="flag">{text}</div> }
                } else {
                    html! { <div class="flag muted">{text}{" (no longer needed)"}</div> }
                }
            }
            None => html! {},
        };

        // TODO: maybe_impl_pr_link, maybe_stabilization_pr_link

        let items = if f.items.is_empty() {
            html! {}
        } else {
            html! { {view_items(f.items)} }
        };

        // Use FeatureSkel at all?
        html! {
            <FeatureSkel title=title>
                {maybe_flag}
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
