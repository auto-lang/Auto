//! macOS-specific functionality.

#![allow(non_snake_case)]

use std::os::raw;

use objc::runtime::Class;
use objc::{Encode, Encoding};

#[link(name = "Cocoa", kind = "framework")]
extern {
    fn CFRelease(_: *mut raw::c_void);

    fn CGEventPost(tap_location:raw::c_int, event: CGEvent);

    fn CGEventCreateCopy(event: CGEvent) -> CGEvent;

    fn CGEventGetFlags(event: CGEvent) -> EventFlags;

    fn CGEventSetFlags(event: CGEvent, flags: EventFlags);
}

#[macro_use]
mod macros;

pub mod keyboard;
pub mod mouse;
pub mod wheel;

lazy_static! {
    static ref NS_EVENT: &'static Class = Class::get("NSEvent").unwrap();
}

cfg_if! {
    if #[cfg(target_pointer_width = "64")] {
        type CGFloat = raw::c_double;
    } else {
        type CGFloat = raw::c_float;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct CGPoint {
    x: CGFloat,
    y: CGFloat,
}

type CGEvent = *mut raw::c_void;
type CGEventSource = *const raw::c_void;

#[repr(C)]
#[derive(Copy, Clone)]
enum CGEventType {
    // The null event.
    Null,

    // Mouse events.
    LeftMouseDown,
    LeftMouseUp,
    RightMouseDown,
    RightMouseUp,
    MouseMoved,
    LeftMouseDragged,
    RightMouseDragged,

    // Keyboard events.
    KeyDown,
    KeyUp,
    FlagsChanged,

    // Specialized control devices.
    ScrollWheel,
    TabletPointer,
    TabletProximity,
    OtherMouseDown,
    OtherMouseUp,
    OtherMouseDragged,

    // Out of band event types. These are delivered to the event tap callback
    // to notify it of unusual conditions that disable the event tap.
    TapDisabledByTimeout,
    TapDisabledByUserInput,
}

unsafe impl Encode for CGPoint {
    fn encode() -> Encoding {
        let inner = f64::encode();
        let encoding = format!("{{CGPoint={0}{0}}}", inner.as_str());
        unsafe { Encoding::from_str(&encoding) }
    }
}

/// An event that can be posted into the Quartz event stream.
#[derive(Debug)]
pub struct Event(CGEvent);

unsafe impl Send for Event {}
unsafe impl Sync for Event {}

impl Clone for Event {
    #[inline]
    fn clone(&self) -> Event {
        unsafe { Event(CGEventCreateCopy(self.0)) }
    }
}

impl Drop for Event {
    #[inline]
    fn drop(&mut self) {
        unsafe { CFRelease(self.0) };
    }
}

impl Event {
    /// Posts `self` to the Quartz event stream at the event location.
    #[inline]
    pub fn post(&self, location: EventLocation) {
        unsafe { CGEventPost(location as raw::c_int, self.0) };
    }

    /// Returns the flags of the inner Quartz event.
    #[inline]
    pub fn flags(&self) -> EventFlags {
        unsafe { CGEventGetFlags(self.0) }
    }

    /// Sets the flags of the inner Quartz event.
    #[inline]
    pub fn set_flags(&mut self, flags: EventFlags) {
        unsafe { CGEventSetFlags(self.0, flags) };
    }

    /// Sets the bits of `flags` in the flags of the inner Quartz event.
    #[inline]
    pub fn enable_flags(&mut self, flags: EventFlags) {
        let prev = self.flags();
        self.set_flags(prev | flags);
    }
}

bitflags! {
    /// Flags for indicating modifier key states, as well as other event-related
    /// states.
    #[repr(C)]
    pub struct EventFlags: u64 {
        /// Indicates that the Caps Lock key is down.
        const ALPHA_SHIFT   = 0x10000;
        /// Indicates that the Shift key is down.
        const SHIFT         = 0x20000;
        /// Indicates that the Control key is down.
        const CONTROL       = 0x40000;
        /// Indicates that the Alt or Option key is down.
        const ALTERNATE     = 0x80000;
        /// Indicates that the Command key is down.
        const COMMAND       = 0x100000;

        /// Indicates that the Help modifier key is down.
        ///
        /// This key is not present on most keyboards, and is different than the
        /// Help key found in the same row as Home and Page Up.
        const HELP          = 0x400000;
        /// Indicates that the Fn (Function) key is down.
        ///
        /// This key is found primarily on laptop keyboards.
        const SECONDARY_FN  = 0x800000;

        /// Identifies key events from the numeric keypad area on extended
        /// keyboards.
        const NUMERIC_PAD   = 0x200000;
        /// Indicates that mouse and pen movements are not being coalesced.
        const NON_COALESCED = 0x100;
    }
}

/// An event location.
#[derive(Copy, Clone)]
pub enum EventLocation {
    /// The event is placed at the point where HID system events enter the
    /// window server.
    Hid,
    /// The event is placed at the point where HID system and remote control
    /// events enter a login session.
    Session,
    /// The event is placed at the point where session events have been
    /// annotated to flow to an application.
    AnnotatedSession,
}
