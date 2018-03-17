//! Keyboard automation utilities.

use std::ptr;

use super::{CGEvent, CGEventSource};

pub mod key_code;

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
    pub fn new(key: u16, down: bool) -> Event {
        Event(super::Event(unsafe {
            CGEventCreateKeyboardEvent(ptr::null(), key, down)
        }))
    }
}
