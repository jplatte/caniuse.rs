use std::fmt::Display;

use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::{
    util::{back_button, view_text, Void},
    AppRoute, FeatureData, RouterAnchor,
};

#[derive(Clone, Properties)]
pub struct Props {
    pub data: FeatureData,
}

pub struct FeaturePage {
    props: Props,
}

impl Component for FeaturePage {
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

        // TODO: Colorization?
        let version = match f.version {
            Some(v) => html! {
                <RouterAnchor route=AppRoute::Version(v.number.into())>{v.number}</RouterAnchor>
            },
            None => html! { "none (unstable)" },
        };

        let info = match f.flag {
            Some(flag) => html! {
                <div class="info">
                    <span>{"Since version:"}</span>
                    <span>{version}</span>
                    <span>{"Feature flag:"}</span>
                    <span>{view_text(flag)}</span>
                </div>
            },
            None => html! { <div>{version}</div> },
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
        let maybe_doc_link = maybe_link("Documentation", "https://doc.rust-lang.org/", f.doc_path);
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
            <>
                {back_button()}
                <div class="box">
                    <h3 class="title">
                        {view_text(f.title)}
                    </h3>
                    {info}
                    <ul class="links">
                        {maybe_rfc_link}
                        {maybe_impl_pr_link}
                        {maybe_tracking_issue_link}
                        {maybe_stabilization_pr_link}
                        {maybe_doc_link}
                        {maybe_edition_guide_link}
                        {maybe_unstable_book_link}
                    </ul>
                    {maybe_items}
                </div>
            </>
        }
    }
}

fn view_items(items: &[&str]) -> Html {
    let items = items.iter().map(|i| html! { <li><code>{i}</code></li> });
    html! {
        <div class="items">
            {"Items"}
            <ul>
                { for items }
            </ul>
        </div>
    }
}
