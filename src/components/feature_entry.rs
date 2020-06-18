use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::{
    data::{Channel, FeatureData},
    util::{view_text, Void},
    AppRoute, RouterAnchor,
};

#[derive(Clone, Properties)]
pub struct Props {
    pub data: FeatureData,
}

pub struct FeatureEntry {
    props: Props,
}

impl Component for FeatureEntry {
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
        let f = self.props.data;
        let v = f.version;

        let maybe_flag = match f.flag {
            Some(flag) if v.is_none() => html! {
                <div class="flag">
                    {"Feature flag: "}<code>{view_text(flag)}</code>
                </div>
            },
            _ => {
                html! {}
            }
        };

        let support_indicator = match v {
            None => html! { <div class="version none">{"Unstable"}</div> },
            Some(version) => {
                let classes = match version.channel {
                    Channel::Nightly => "version nightly",
                    Channel::Beta => "version beta",
                    Channel::Stable => "version stable",
                };

                html! {
                    <div class=classes>
                        <RouterAnchor route=AppRoute::Version(version.number.into())>
                            {"Rust "}{version.number}
                        </RouterAnchor>
                    </div>
                }
            }
        };

        html! {
            <div class="feature-entry">
                <div class="box">
                    <RouterAnchor route=AppRoute::Feature(f.slug.into()) classes="title">
                        <h3>{view_text(f.title)}</h3>
                    </RouterAnchor>
                    {maybe_flag}
                </div>
                {support_indicator}
            </div>
        }
    }
}
