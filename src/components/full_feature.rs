use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::{
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

        let maybe_flag = match f.flag {
            Some(flag) => html! { <div class="flag">{"Feature flag: "}{view_text(flag)}</div> },
            None => html! {},
        };

        let maybe_impl_pr_link = match f.impl_pr_id {
            Some(pr_id) => html! {
                <a href={format!("https://github.com/rust-lang/rust/pull/{}", pr_id)}>
                    {"Implementation PR"}
                </a>
            },
            None => html! {},
        };

        let maybe_stabilization_pr_link = match f.stabilization_pr_id {
            Some(pr_id) => html! {
                <a href={format!("https://github.com/rust-lang/rust/pull/{}", pr_id)}>
                    {"Stabilization PR"}
                </a>
            },
            None => html! {},
        };

        let items = if f.items.is_empty() {
            html! {}
        } else {
            html! { {view_items(f.items)} }
        };

        html! {
            <li class="feature-box">
                <div class="feature">
                    <h3 class="title">
                        {view_text(f.title)}
                    </h3>
                    {maybe_flag}
                    {maybe_impl_pr_link}
                    {maybe_stabilization_pr_link}
                    {items}
                </div>
            </li>
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
