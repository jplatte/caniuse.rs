use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::{
    features::{Channel, FeatureData, Match},
    search::Span,
    util::{view_text_with_matches, Void},
    AppRoute,
};

#[derive(Clone, Properties)]
pub struct Props {
    #[props(required)]
    pub data: FeatureData,
    pub match_: Match,
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
        type RouterAnchor = yew_router::components::RouterAnchor<AppRoute>;

        let f = self.props.data;
        let m = &self.props.match_;

        let maybe_flag = match f.flag {
            Some(flag) if f.version.is_none() || !m.flag_spans.is_empty() => html! {
                <div class="flag">
                    {"Feature flag: "}{view_text_with_matches(flag, &m.flag_spans)}
                </div>
            },
            _ => {
                html! {}
            }
        };

        let items = if f.items.is_empty() {
            html! {}
        } else if m.item_spans.is_empty() {
            view_items(f.items)
        } else {
            // TODO: Add ability to show any items that don't match
            view_matched_items(f.items, &m.item_spans)
        };

        let support_indicator = match f.version {
            None => html! { <div class="version none">{"Unstable"}</div> },
            Some(v) => {
                let classes = match f.channel {
                    Channel::Nightly => "version nightly",
                    Channel::Beta => "version beta",
                    Channel::Stable => "version stable",
                };

                html! { <div class=classes>{"Rust "}{v}</div> }
            }
        };

        html! {
            <div class="feature-box">
                <div class="feature">
                    <h3 class="title">
                        <RouterAnchor route=AppRoute::Feature(f.slug.into())>
                            {view_text_with_matches(f.title, &m.title_spans)}
                        </RouterAnchor>
                    </h3>
                    {maybe_flag}
                {items}
                </div>
                {support_indicator}
            </div>
        }
    }
}

fn view_items(items: &[&str]) -> Html {
    let mut items = items.iter().map(|i| html! { <li><code>{i}</code></li> });
    html! {
        <details class="items">
            <summary>{"Items"}</summary>
            <ul>
                { for items }
            </ul>
        </details>
    }
}

fn view_matched_items(items: &[&str], item_spans: &[Vec<Span>]) -> Html {
    let mut res = items.iter().zip(item_spans).filter(|(_, spans)| !spans.is_empty()).map(
        |(item, spans)| html! { <li><code>{view_text_with_matches(item, &spans)}</code></li> },
    );

    let more_items_indicator = if item_spans.iter().any(|s| s.is_empty()) {
        html! { <li>{"…"}</li> }
    } else {
        html! {}
    };

    html! {
        <div class="items">
            {"Items"}
            <ul>
                { for res }
                {more_items_indicator}
            </ul>
        </div>
    }
}
