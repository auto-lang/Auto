//! Keyboard automation utilities.

use std::ptr;

use super::{CGEvent, CGEventSource};

extern {
    fn CGEventCreateKeyboardEvent(
        source: CGEventSource,
        virtual_key: CGKeyCode,
        key_down: bool,
    ) -> CGEvent;
}

type CGKeyCode = u16;

declare_event!("A keyboard event that can be posted into the Quartz event stream.");

impl Event {
    /// Creates a new event for the virtual key.
    #[inline]
    pub fn new(virtual_key: u16, key_down: bool) -> Event {
        Event(super::Event(unsafe {
            CGEventCreateKeyboardEvent(ptr::null(), virtual_key, key_down)
        }))
    }
}
