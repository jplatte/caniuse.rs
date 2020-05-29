use std::fmt;

use gloo::events::EventListener;
use web_sys::Element;
use yew::callback::Callback;

/// A service that fires events when a specific element is clicked.
#[derive(Debug)]
pub struct ClickService {
    elem: Element,
}

/// A handle to the event listener for click events.
#[must_use]
#[allow(dead_code)]
pub struct ClickTask {
    elem: Element,
    handle: EventListener,
}

impl fmt::Debug for ClickTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ClickTask")
    }
}

impl ClickService {
    /// Creates a new ClickService.
    pub fn new(elem: Element) -> ClickService {
        ClickService { elem }
    }

    /// Register a callback that will be called when the user clicks the element.
    pub fn register(&mut self, callback: Callback<()>) -> ClickTask {
        let handle = EventListener::new(&self.elem, "click", move |_| callback.emit(()));
        ClickTask { elem: self.elem.clone(), handle }
    }
}
