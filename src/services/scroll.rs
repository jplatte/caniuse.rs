use std::fmt;

use gloo::events::EventListener;
use yew::callback::Callback;

use crate::util::window;

/// A service that fires events when the browser window scrolls.
#[derive(Debug)]
pub struct ScrollService;

/// A handle to the event listener for scroll events.
#[must_use]
pub struct ScrollTask(EventListener);

impl fmt::Debug for ScrollTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ScrollTask")
    }
}

impl ScrollService {
    /// Creates a new ScrollService.
    pub fn new() -> ScrollService {
        ScrollService
    }

    /// Register a callback that will be called when the browser window resizes.
    pub fn register(&mut self, callback: Callback<()>) -> ScrollTask {
        ScrollTask(EventListener::new(&window(), "scroll", move |_| callback.emit(())))
    }
}
