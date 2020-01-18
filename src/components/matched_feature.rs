use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::{
    components::FeatureSkel,
    features::{FeatureData, Match, Span},
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
        let m = &self.props.match_;

        let desc = match m.desc_span.clone() {
            Some(s) => view_text_with_match(f.desc_short, s),
            None => html! { {f.desc_short} },
        };

        let maybe_flag = match f.flag {
            Some(f) => {
                let val = match m.flag_span.clone() {
                    Some(s) => view_text_with_match(f, s),
                    None => html! { {f} },
                };

                html! {
                    <div class="flag">{"Feature flag: "}{val}</div>
                }
            }
            None => {
                assert!(m.flag_span.is_none());
                html! {}
            }
        };

        let items = if f.items.is_empty() {
            html! {}
        } else {
            html! { <ul>{view_matched_items(f.items, &m.item_spans)}</ul> }
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

fn view_matched_items(items: &[&str], item_spans: &[Option<Span>]) -> Html {
    let mut res = items.iter().zip(item_spans).filter_map(|(item, span)| {
        span.as_ref().map(|s| {
            html! {
                <li>{view_text_with_match(item, s.clone())}</li>
            }
        })
    });

    let more_items_indicator = if item_spans.iter().any(|s| s.is_none()) {
        html! { <li>{"…"}</li> }
    } else {
        html! {}
    };

    html! { <>{ for res }{more_items_indicator}</> }
}

fn view_text_with_match(text: &str, s: Span) -> Html {
    html! {
        <>
            {&text[..s.start]}
            <span class="match">{&text[s.clone()]}</span>
            {&text[s.end..]}
        </>
    }
}
