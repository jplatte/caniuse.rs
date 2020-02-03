use std::fmt;

use stdweb::{js, Value};
use yew::callback::Callback;

/// A service that fires events when the browser window scrolls.
#[derive(Debug)]
pub struct ScrollService;

/// A handle to the event listener for scroll events.
#[must_use]
pub struct ScrollTask(Option<Value>);

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
        let callback = move || callback.emit(());
        let handle = js! {
            var callback = @{callback};
            var handle = function() {
                callback();
            };
            window.addEventListener("scroll", handle);
            return handle;
        };
        ScrollTask(Some(handle))
    }
}

impl Drop for ScrollTask {
    fn drop(&mut self) {
        let handle = self.0.take().expect("Scroll task already empty.");
        js! {
            @(no_return)
            window.removeEventListener("scroll", @{handle});
        }
    }
}
