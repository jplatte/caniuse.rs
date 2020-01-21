use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::{
    components::FeatureSkel,
    features::{FeatureData, Match},
    util::{view_text_with_matches, Span, Void},
};

#[derive(Clone, Properties)]
pub struct Props {
    pub data: Option<FeatureData>,
    pub match_: Match,
}

pub struct MatchedFeature {
    props: Props,
}

impl Component for MatchedFeature {
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
        let f = match self.props.data {
            Some(data) => data,
            None => return html! {}, // meh
        };
        let m = &self.props.match_;

        let title = view_text_with_matches(f.title, &m.title_spans);

        let maybe_flag = match f.flag {
            Some(flag) => {
                let text = html! {
                    <>{"Feature flag: "}{view_text_with_matches(flag, &m.flag_spans)}</>
                };
                if f.version == "nightly" {
                    html! { <div class="flag">{text}</div> }
                } else {
                    html! { <div class="flag muted">{text}{" (no longer needed)"}</div> }
                }
            }
            None => {
                assert!(m.flag_spans.is_empty());
                html! {}
            }
        };

        let items = if f.items.is_empty() {
            html! {}
        } else {
            html! { <ul>{view_matched_items(f.items, &m.item_spans)}</ul> }
        };

        html! {
            <FeatureSkel title=title version=f.version>
                {maybe_flag}
                {items}
            </FeatureSkel>
        }
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

    html! { <>{ for res }{more_items_indicator}</> }
}
