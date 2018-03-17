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
    pub fn new<T: Into<u16>>(key: T, down: bool) -> Event {
        Event(super::Event(unsafe {
            CGEventCreateKeyboardEvent(ptr::null(), key.into(), down)
        }))
    }
}

macro_rules! keys {
    ($($name:ident = $val:expr),+ $(,)*) => {
        keys! { $(
            concat!(stringify!($name), " key code.");
            $name = $val,
        )+ }
    };
    ($($doc:expr; $name:ident = $val:expr,)+) => {
        /// A virtual key code used in keyboard events.
        #[repr(u16)]
        #[derive(Copy, Clone, Debug)]
        pub enum KeyCode { $(
            #[doc = $doc]
            $name = $val,
        )+ }
    };
}

keys! {
    // Independent of keyboard layout
    Return        = 0x24,
    Tab           = 0x30,
    Space         = 0x31,
    Delete        = 0x33,
    Escape        = 0x35,
    Command       = 0x37,
    Shift         = 0x38,
    CapsLock      = 0x39,
    Option        = 0x3A,
    Control       = 0x3B,
    RightCommand  = 0x36,
    RightShift    = 0x3C,
    RightOption   = 0x3D,
    RightControl  = 0x3E,
    Function      = 0x3F,
    VolumeUp      = 0x48,
    VolumeDown    = 0x49,
    Mute          = 0x4A,
    F1            = 0x7A,
    F2            = 0x78,
    F3            = 0x63,
    F4            = 0x76,
    F5            = 0x60,
    F6            = 0x61,
    F7            = 0x62,
    F8            = 0x64,
    F9            = 0x65,
    F10           = 0x6D,
    F11           = 0x67,
    F12           = 0x6F,
    F13           = 0x69,
    F14           = 0x6B,
    F15           = 0x71,
    F16           = 0x6A,
    F17           = 0x40,
    F18           = 0x4F,
    F19           = 0x50,
    F20           = 0x5A,
    Help          = 0x72,
    Home          = 0x73,
    PageUp        = 0x74,
    ForwardDelete = 0x75,
    End           = 0x77,
    PageDown      = 0x79,
    LeftArrow     = 0x7B,
    RightArrow    = 0x7C,
    DownArrow     = 0x7D,
    UpArrow       = 0x7E,
}

impl From<KeyCode> for u16 {
    #[inline]
    fn from(key_code: KeyCode) -> u16 {
        key_code as u16
    }
}
