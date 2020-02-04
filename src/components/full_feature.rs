use std::fmt::Display;

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

        fn maybe_link<T: Display>(text: &str, link_base: &str, opt_rest: Option<T>) -> Html {
            match opt_rest {
                Some(id) => html! { <li><a href=format!("{}{}", link_base, id)>{text}</a></li> },
                None => html! {},
            }
        };

        let maybe_rfc_link =
            maybe_link("RFC", "https://github.com/rust-lang/rfcs/issues/", f.rfc_id);
        let maybe_impl_pr_link = maybe_link(
            "Implementation PR",
            "https://github.com/rust-lang/rust/pull/",
            f.impl_pr_id,
        );
        let maybe_tracking_issue_link = maybe_link(
            "Tracking issue",
            "https://github.com/rust-lang/rust/issues/",
            f.tracking_issue_id,
        );
        let maybe_stabilization_pr_link = maybe_link(
            "Stabilization PR",
            "https://github.com/rust-lang/rust/pull/",
            f.stabilization_pr_id,
        );
        let maybe_edition_guide_link = maybe_link(
            "Edition Guide",
            "https://doc.rust-lang.org/edition-guide/",
            f.edition_guide_path,
        );
        let maybe_unstable_book_link = maybe_link(
            "Unstable book",
            "https://doc.rust-lang.org/unstable-book/",
            f.unstable_book_path,
        );

        let maybe_items = if f.items.is_empty() {
            html! {}
        } else {
            view_items(f.items)
        };

        html! {
            <li class="feature-box">
                <div class="feature">
                    <h3 class="title">
                        {view_text(f.title)}
                    </h3>
                    {maybe_flag}
                    <ul class="links">
                        {maybe_rfc_link}
                        {maybe_impl_pr_link}
                        {maybe_tracking_issue_link}
                        {maybe_stabilization_pr_link}
                        {maybe_edition_guide_link}
                        {maybe_unstable_book_link}
                    </ul>
                    {maybe_items}
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
