use std::borrow::Cow;

use gloo_history::{BrowserHistory, History};
use gloo_utils::window;
use xilem_html::{elements::a, interfaces::Element, ViewExt, ViewSequence};

use crate::AppState;

pub trait Route: Clone {
    fn to_path(&self) -> Cow<'static, str>;
}

#[derive(Clone)]
pub enum AppRoute {
    List(ListRoute),
    Feature { slug: Cow<'static, str> },
    Version { number: Cow<'static, str> },
    About,
}

impl Route for AppRoute {
    fn to_path(&self) -> Cow<'static, str> {
        match self {
            AppRoute::List(ListRoute::Stable) => "/".into(),
            AppRoute::List(ListRoute::RecentlyStabilized) => "/recent".into(),
            AppRoute::List(ListRoute::Unstable) => "/unstable".into(),
            AppRoute::List(ListRoute::SearchResults { input }) => format!("/?s={input}").into(),
            AppRoute::Feature { slug } => format!("/features/{slug}").into(),
            AppRoute::Version { number } => format!("/versions/{number}").into(),
            AppRoute::About => "/about".into(),
        }
    }
}

#[derive(Clone, Default)]
pub enum ListRoute {
    #[default]
    Stable,
    RecentlyStabilized,
    Unstable,
    SearchResults {
        input: String,
    },
}

impl AppRoute {
    pub fn new() -> Self {
        let path = window().location().pathname().unwrap();
        match path.as_str() {
            "/about" => AppRoute::About,
            "/recent" => AppRoute::List(ListRoute::RecentlyStabilized),
            "/unstable" => AppRoute::List(ListRoute::Unstable),
            p => {
                if let Some(feature_name) = p.strip_prefix("/features/") {
                    AppRoute::Feature { slug: feature_name.to_owned().into() }
                } else if let Some(version_number) = p.strip_prefix("/versions/") {
                    AppRoute::Version { number: version_number.to_owned().into() }
                } else {
                    AppRoute::List(ListRoute::Stable)
                }
            }
        }
    }
}

pub(crate) fn route_link<ViewSeq>(route: AppRoute, children: ViewSeq) -> impl Element<AppState>
where
    ViewSeq: ViewSequence<AppRoute>,
{
    route_link_generic(route, children).adapt_state(|data: &mut AppState| &mut data.route)
}

pub fn route_link_generic<T: Route, A, ViewSeq>(route: T, children: ViewSeq) -> impl Element<T, A>
where
    ViewSeq: ViewSequence<T, A>,
{
    let history = BrowserHistory::new();
    a(children)
        .attr("href", route.to_path())
        .on_click(move |state: &mut T, evt| {
            history.push(route.to_path());
            *state = route.clone();
            evt.prevent_default();
        })
        .passive(false)
}
