use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::util::Void;

pub struct About;

impl Component for About {
    type Message = Void;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, msg: Void) -> ShouldRender {
        match msg {}
    }

    fn view(&self) -> Html {
        html! {}
    }
}
