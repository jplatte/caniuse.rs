use std::rc::Rc;

use gloo_events::EventListener;
use gloo_utils::document;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::{html, Component, Context, Html, NodeRef};
use yew_router::BrowserRouter;

use crate::{
    components::{
        index::{Explore, IndexContents},
        About, FeaturePage, Header, Index, VersionPage,
    },
    AppRoute, FEATURES, VERSIONS,
};

pub struct App {
    input_ref: NodeRef,
    search_query: Rc<String>,

    _key_listener: EventListener,
}

pub enum Msg {
    FocusInput,
    Search(Rc<String>),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link2 = ctx.link().clone();
        let _key_listener = EventListener::new(&document(), "keypress", move |event| {
            let event = event.dyn_ref::<KeyboardEvent>().expect("wrong event type");
            if event.key().as_str() == "s" {
                link2.send_message(Msg::FocusInput);
            }
        });

        Self { input_ref: NodeRef::default(), search_query: Rc::new(String::new()), _key_listener }
    }

    fn update(&mut self, _: &Context<Self>, msg: Msg) -> bool {
        match msg {
            Msg::FocusInput => {
                self.input_ref.cast::<HtmlInputElement>().unwrap().focus().unwrap();
                false
            }
            Msg::Search(query) => {
                self.search_query = query;
                true
            }
        }
    }

    fn changed(&mut self, _: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        type Switch = yew_router::Switch<AppRoute>;

        let search_query = self.search_query.clone();
        let render_route = move |route| match &route {
            AppRoute::Index | AppRoute::RecentlyStabilized | AppRoute::Unstable => {
                let show = if search_query.is_empty() {
                    IndexContents::Explore(match &route {
                        AppRoute::Index => Explore::Stable,
                        AppRoute::RecentlyStabilized => Explore::RecentlyStabilized,
                        AppRoute::Unstable => Explore::Unstable,
                        _ => unreachable!(),
                    })
                } else {
                    IndexContents::SearchResults { search_query: search_query.clone() }
                };

                html! { <Index show={show} /> }
            }
            #[allow(clippy::let_unit_value)] // buggy lint
            AppRoute::About => html! { <About /> },
            AppRoute::Feature { name: slug } => match FEATURES.iter().find(|f| f.slug == slug) {
                Some(&data) => html! { <FeaturePage data={data} /> },
                None => html! { "error: feature not found!" },
            },
            AppRoute::Version { number } => match VERSIONS.iter().find(|v| v.number == number) {
                Some(&data) => html! { <VersionPage data={data} /> },
                None => html! { "error: version not found!" },
            },
        };

        let input = self.input_ref.clone();
        let oninput = ctx.link().callback(move |_e| {
            Msg::Search(Rc::new(input.cast::<HtmlInputElement>().unwrap().value()))
        });

        html! {
            <BrowserRouter>
                <Header input_ref={self.input_ref.clone()} oninput={oninput} />
                <div class="page">
                    <Switch render={render_route} />
                </div>
            </BrowserRouter>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(Msg::FocusInput);
        }
    }
}
