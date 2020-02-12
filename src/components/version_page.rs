use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::{
    util::{maybe_link, Void},
    VersionData,
};

#[derive(Clone, Properties)]
pub struct Props {
    #[props(required)]
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

    fn view(&self) -> Html {
        let v = &self.props.data;

        let maybe_blog_link =
            maybe_link("Blog post", "https://blog.rust-lang.org/", v.blog_post_path);

        html! {
            <div class="box">
                <h3 class="title">{"Rust "}{v.number}</h3>
                <ul class="links">
                    {maybe_blog_link}
                </ul>
            </div>
        }
    }
}
