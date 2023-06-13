use gloo_utils::window;
use yew::{html, Component, Context, Html, Properties};

use crate::{
    components::FeatureEntry,
    util::{home_button, maybe_link, Void},
    VersionData, FEATURES,
};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub data: VersionData,
}

pub struct VersionPage;

impl Component for VersionPage {
    type Message = Void;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        // Ugly hack because I don't want to write my own component that resets
        // the scroll position on route change. See also
        // https://github.com/yewstack/yew/issues/1099
        window().scroll_to_with_x_and_y(0.0, 0.0);

        Self
    }

    fn update(&mut self, _: &Context<Self>, void: Self::Message) -> bool {
        match void {}
    }

    fn changed(&mut self, _: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let v = &ctx.props().data;

        let maybe_blog_link =
            maybe_link("Blog post", "https://blog.rust-lang.org/", v.blog_post_path);
        let maybe_release_notes = maybe_link(
            "Release notes",
            "https://github.com/rust-lang/rust/blob/master/RELEASES.md#",
            v.release_notes,
        );
        let maybe_gh_milestone_link = maybe_link(
            "GitHub milestone",
            "https://github.com/rust-lang/rust/milestone/",
            v.gh_milestone_id,
        );

        let maybe_release_date = match v.release_date {
            Some(release_date) => html! {
                <>
                    <span>{"Release date:"}</span>
                    <time datetime={release_date}>{release_date}</time>
                </>
            },
            None => html! {},
        };

        let features = FEATURES
            .iter()
            .filter(|f| matches!(f.version, Some(fv) if fv.number == v.number))
            .map(|&f| html! { <FeatureEntry key={f.slug} data={f} show_version=false /> });

        html! {
            <>
                {home_button()}
                <div class="box">
                    <h3 class="title">{"Rust "}{v.number}</h3>
                    <div class="info">
                        {maybe_release_date}
                    </div>
                    <ul class="links">
                        {maybe_blog_link}
                        {maybe_release_notes}
                        {maybe_gh_milestone_link}
                    </ul>
                </div>
                <div class="feature-list">{ for features }</div>
            </>
        }
    }
}
