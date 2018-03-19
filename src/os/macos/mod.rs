//! 🏎️ macOS-specific functionality.

#![allow(improper_ctypes)]
#![allow(non_snake_case)]

use std::fmt;
use std::ffi::CStr;
use std::os::raw;
use std::ptr;

use objc::runtime::{Class, Object};
use objc::{Encode, Encoding};

#[link(name = "Cocoa", kind = "framework")]
extern {
    fn CFRelease(_: CFObjectRef);

    fn CGEventPost(tap_location: raw::c_int, event: CFObjectRef);

    fn CGEventCreateCopy(event: CFObjectRef) -> CFObject;

    fn CGEventGetFlags(event: CFObjectRef) -> EventFlags;

    fn CGEventSetFlags(event: CFObjectRef, flags: EventFlags);
}

#[macro_use]
mod macros;

pub mod app;
pub mod keyboard;
pub mod mouse;
pub mod screen;
pub mod wheel;

lazy_static! {
    static ref NS_EVENT: &'static Class = Class::get("NSEvent").unwrap();
}

unsafe fn ns_string_encode_utf8(ns_string: Option<NSObject>) -> Option<String> {
    if let Some(s) = ns_string {
        let s = CStr::from_ptr(msg_send![s.inner(), UTF8String]);
        Some(s.to_string_lossy().into())
    } else {
        None
    }
}

type CFObjectRef = ptr::NonNull<raw::c_void>;
type NSObjectRef = ptr::NonNull<Object>;

macro_rules! impl_object {
    ($obj:ident, $inner:ty, $($drop:tt)+) => {
        #[repr(C)]
        #[derive(PartialEq, Eq, Hash)]
        struct $obj($inner);

        impl Drop for $obj {
            #[inline]
            $($drop)+
        }

        impl fmt::Debug for $obj {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        unsafe impl Send for $obj {}
        unsafe impl Sync for $obj {}
    }
}

impl_object!(CFObject, CFObjectRef, fn drop(&mut self) {
    unsafe { CFRelease(self.0) };
});

impl_object!(NSObject, NSObjectRef, fn drop(&mut self) {
    let ptr = self.0.as_ptr();
    unsafe { msg_send![ptr, release] };
});

impl NSObject {
    fn inner(&self) -> &Object {
        unsafe { self.0.as_ref() }
    }
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

#[repr(C)]
#[derive(Copy, Clone)]
struct CGSize {
    width: CGFloat,
    height: CGFloat,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct CGRect {
    origin: CGPoint,
    size:   CGSize,
}

type CGEvent = CFObject;
type CGEventSource = *const Object;

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(dead_code)]
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
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Event(CGEvent);

impl Clone for Event {
    #[inline]
    fn clone(&self) -> Event {
        unsafe { Event(CGEventCreateCopy((self.0).0)) }
    }
}

impl Event {
    /// Posts `self` to the Quartz event stream at the event location.
    #[inline]
    pub fn post(&self, location: EventLocation) {
        unsafe { CGEventPost(location as raw::c_int, (self.0).0) };
    }

    /// Returns the flags of the inner Quartz event.
    #[inline]
    pub fn flags(&self) -> EventFlags {
        unsafe { CGEventGetFlags((self.0).0) }
    }

    /// Sets the flags of the inner Quartz event.
    #[inline]
    pub fn set_flags(&mut self, flags: EventFlags) {
        unsafe { CGEventSetFlags((self.0).0, flags) };
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
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
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
