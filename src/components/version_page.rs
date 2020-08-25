use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::{
    util::{back_button, maybe_link, Void},
    VersionData,
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

        html! {
            <>
                {back_button()}
                <div class="box">
                    <h3 class="title">{"Rust "}{v.number}</h3>
                    <div class="info">
                        <span>{"Release date:"}</span>
                        <span>{v.release_date}</span>
                    </div>
                    <ul class="links">
                        {maybe_blog_link}
                        {maybe_release_notes}
                        {maybe_gh_milestone_link}
                    </ul>
                </div>
            </>
        }
    }
}
