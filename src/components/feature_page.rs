use yew::{html, Component, Context, Html, Properties};

use crate::{
    util::{home_button, maybe_link, view_text, Void},
    AppRoute, FeatureData, RouterLink,
};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub data: FeatureData,
}

pub struct FeaturePage;

impl Component for FeaturePage {
    type Message = Void;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: &Context<Self>, void: Self::Message) -> bool {
        match void {}
    }

    fn changed(&mut self, _: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let f = &ctx.props().data;

        // TODO: Colorization?
        let version = match f.version {
            Some(v) => html! {
                <RouterLink to={AppRoute::Version { number: v.number.into() }}>
                    {v.number}
                </RouterLink>
            },
            None => html! { "none (unstable)" },
        };

        let flag_info = match f.flag {
            Some(flag) => html! {
                <>
                    <span>{"Feature flag:"}</span>
                    <span><code>{view_text(flag)}</code></span>
                </>
            },
            None => html! {},
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
                {home_button()}
                <div class="box">
                    <h3 class="title">
                        {view_text(f.title)}
                    </h3>
                    <div class="info">
                        <span>{"Since version:"}</span>
                        <span>{version}</span>
                        {flag_info}
                    </div>
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
    let items = items.iter().map(|item| {
        if item.contains('\n') {
            html! { <li><pre>{item}</pre></li> }
        } else {
            html! { <li><code>{item}</code></li> }
        }
    });
    html! {
        <div class="items">
            {"Items"}
            <ul>
                { for items }
            </ul>
        </div>
    }
}
