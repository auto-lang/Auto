//! macOS-specific functionality.

#![allow(non_snake_case)]

use std::mem;
use std::os::raw::{c_int, c_void};

use objc::runtime::Class;
use objc::{Encode, Encoding};

#[link(name = "Cocoa", kind = "framework")]
extern {
    fn CFRelease(_: *mut c_void);

    fn CGEventPost(tap_location: c_int, event: CGEvent);
}

pub mod mouse;

lazy_static! {
    static ref NS_EVENT: &'static Class = Class::get("NSEvent").unwrap();
}

#[repr(C)]
#[derive(Copy, Clone)]
struct CGPoint {
    x: f64,
    y: f64,
}

impl From<CGPoint> for (f64, f64) {
    #[inline]
    fn from(point: CGPoint) -> (f64, f64) {
        unsafe { mem::transmute(point) }
    }
}

impl From<(f64, f64)> for CGPoint {
    #[inline]
    fn from(tuple: (f64, f64)) -> CGPoint {
        unsafe { mem::transmute(tuple) }
    }
}

type CGEvent = *mut c_void;

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
