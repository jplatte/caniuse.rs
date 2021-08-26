use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::{
    components::FeatureEntry,
    util::{home_button, maybe_link, window, Void},
    VersionData, FEATURES,
};

#[derive(Clone, Properties)]
pub struct Props {
    pub data: VersionData,
}

pub struct VersionPage {
    props: Props,
}

impl Component for VersionPage {
    type Message = Void;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        // Ugly hack because I don't want to write my own component that resets
        // the scroll position on route change. See also
        // https://github.com/yewstack/yew/issues/1099
        window().scroll_to_with_x_and_y(0.0, 0.0);

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
        let v = &self.props.data;

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
                    <time datetime=release_date>{release_date}</time>
                </>
            },
            None => html! {},
        };

        let features = FEATURES
            .iter()
            .filter(|f| matches!(f.version, Some(fv) if fv.number == v.number))
            .map(|&f| html! { <FeatureEntry key=f.slug data=f show_version=false /> });

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
