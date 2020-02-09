use std::fmt;
use stdweb::{js, web::Element, Value};
use yew::callback::Callback;

/// A service that fires events when the browser window resizes.
#[derive(Debug)]
pub struct ClickService {
    elem: Element,
}

/// A handle to the event listener for resize events.
#[must_use]
pub struct ClickTask {
    elem: Element,
    handle: Option<Value>,
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

    /// Register a callback that will be called when the browser window Clicks.
    pub fn register(&mut self, callback: Callback<()>) -> ClickTask {
        let element = &self.elem;
        let callback = move || callback.emit(());
        let handle = js! {
            var callback = @{callback};
            var handle = function() {
                callback();
            };
            @{element}.addEventListener("click", handle);
            return handle;
        };
        ClickTask { elem: self.elem.clone(), handle: Some(handle) }
    }
}

impl Drop for ClickTask {
    fn drop(&mut self) {
        let element = &self.elem;
        let handle = self.handle.take().expect("Click task already empty.");
        js! {
            @(no_return)
            @{element}.removeEventListener("click", @{handle})
        }
    }
}
