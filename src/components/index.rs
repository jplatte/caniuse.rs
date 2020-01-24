use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};

use crate::{
    components::{Feature, MatchedFeature},
    search::extract_search_terms,
    FeatureData, FEATURES,
};

pub struct Index {
    link: ComponentLink<Self>,
    current_search_terms: Vec<String>,
    current_search_results: Vec<FeatureData>,
}

pub enum Msg {
    Search(String),
}

impl Component for Index {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, current_search_terms: Vec::new(), current_search_results: Vec::new() }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Search(query) => {
                let search_terms = extract_search_terms(&query).unwrap_or_default();
                let features_to_search = if !self.current_search_terms.is_empty()
                    && self.current_search_terms.len() <= search_terms.len()
                {
                    let len = self.current_search_terms.len();

                    if self.current_search_terms[..] == search_terms[..len]
                        || (self.current_search_terms[..len - 1] == search_terms[..len - 1]
                            && search_terms[len - 1]
                                .starts_with(&self.current_search_terms[len - 1]))
                    {
                        &self.current_search_results
                    } else {
                        FEATURES
                    }
                } else {
                    FEATURES
                };

                self.current_search_results = if search_terms.is_empty() {
                    Vec::new()
                } else {
                    features_to_search
                        .iter()
                        .filter(|f| f.does_match(&search_terms))
                        .copied()
                        .collect()
                };
                self.current_search_terms = search_terms;

                true
            }
        }
    }

    fn view(&self) -> Html {
        let features = if self.current_search_terms.is_empty() {
            let mut list = FEATURES.iter().map(|&f| html! { <Feature data=f /> });
            html! { { for list } }
        } else {
            let mut list = self.current_search_results.iter().map(|&f| {
                let m = f.get_matches(&self.current_search_terms).expect("matching feature");
                html! { <MatchedFeature data=f match_=m /> }
            });

            html! { { for list } }
        };

        html! {
            <>
                {"Can I use "}
                <input type="search" oninput=self.link.callback(|e: InputData| Msg::Search(e.value)) />
                {" ?"}
                <ul class="feature-list">{ features }</ul>
            </>
        }
    }
}
