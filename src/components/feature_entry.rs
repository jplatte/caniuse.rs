use yew::{html, Classes, Component, Context, Html, Properties};

use crate::{
    data::{Channel, FeatureData},
    util::{view_text, Void},
    AppRoute, RouterLink,
};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub data: FeatureData,
    #[prop_or(true)]
    pub show_version: bool,
}

pub struct FeatureEntry;

impl Component for FeatureEntry {
    type Message = Void;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: &Context<Self>, void: Self::Message) -> bool {
        match void {}
    }

    fn changed(&mut self, _: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let f = &ctx.props().data;
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

        let support_indicator = if ctx.props().show_version {
            match v {
                None => html! { <div class="version none">{"Unstable"}</div> },
                Some(version) => {
                    let classes = match version.channel {
                        Channel::Nightly => "version nightly",
                        Channel::Beta => "version beta",
                        Channel::Stable => "version stable",
                    };

                    html! {
                        <div class={classes}>
                            <RouterLink to={AppRoute::Version { number: version.number.into() }}>
                                {"Rust "}{version.number}
                            </RouterLink>
                        </div>
                    }
                }
            }
        } else {
            html! {}
        };

        let classes: Classes = "title".into();
        html! {
            <div class="feature-entry">
                <div class="box">
                    <RouterLink to={AppRoute::Feature { name: f.slug.into() }} classes={classes}>
                        <h3>{view_text(f.title)}</h3>
                    </RouterLink>
                    {maybe_flag}
                </div>
                {support_indicator}
            </div>
        }
    }
}
