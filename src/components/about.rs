use yew::{html, Component, Context, Html};

use crate::util::{home_button, Void};

pub struct About;

impl Component for About {
    type Message = Void;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: &Context<Self>, msg: Void) -> bool {
        match msg {}
    }

    fn changed(&mut self, _: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <>
                {home_button()}
                <div class="about box">
                    <h3>{"About caniuse.rs"}</h3>
                    <p>
                        {"Created by Jonas Platte, in Rust, using "}
                        <a href="https://yew.rs/">{"Yew"}</a>{"."}
                    </p>
                    <p>
                        {"You can find the code for this site on "}
                        <a href="https://github.com/jplatte/caniuse.rs">{"GitHub"}</a>{"."}
                    </p>
                    <h3>{"About the creator"}</h3>
                    <p>
                        {"I'm Jonas and I work on free software in my spare time, usually on "}
                        {"projects written in Rust and / or for the Linux desktop. I am a "}
                        {"maintainer of the "}<a href="https://ruma.dev/">{"ruma"}</a>{" project "}
                        {" and have made minor contributions to many other open-source projects "}
                        {"over the course of the years."}
                    </p>
                    <p>
                        {"I've also created "}<a href="https://turbo.fish/">{"turbo.fish"}</a>{"."}
                    </p>
                    <p>
                        {"You can find me on"}
                        <ul>
                            <li><a href="https://github.com/jplatte">{"GitHub"}</a></li>
                            <li><a href="https://git.sr.ht/~jplatte">{"sourcehut"}</a></li>
                            <li><a href="https://blog.turbo.fish/">{"My blog"}</a></li>
                        </ul>
                    </p>
                    <p>
                        {"If you want to support me financially, you can do so on "}
                        <ul>
                            <li><a href="https://liberapay.com/jplatte">{"Liberapay"}</a></li>
                        </ul>
                    </p>
                </div>
            </>
        }
    }
}
