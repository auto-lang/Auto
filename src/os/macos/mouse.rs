//! Mouse automation utilities.

use std::{fmt, mem, ptr};
use std::os::raw;

use objc::runtime::Class;

use super::{CGEvent, CGEventPost, CGEventType, CGPoint, NS_EVENT};

extern {
    fn CGEventCreateMouseEvent(
        source: *const raw::c_void,
        mouse_type: CGEventType,
        mouse_cursor_position: CGPoint,
        mouse_button: raw::c_int,
    ) -> *mut raw::c_void;

    fn CGEventGetLocation(event: CGEvent) -> CGPoint;

    fn CGEventSetLocation(event: CGEvent, location: CGPoint);

    fn CGWarpMouseCursorPosition(new_cursor_position: CGPoint) -> CGPoint;
}

/// A button on the mouse.
#[derive(Copy, Clone)]
pub enum Button {
    /// Left button where the index finger would press.
    Left,
    /// Right button where the middle finger might press.
    Right,
}

/// A mouse event that can be posted into the Quartz event stream.
#[derive(Debug)]
pub struct Event(CGEvent);

unsafe impl Send for Event {}
unsafe impl Sync for Event {}

impl Drop for Event {
    fn drop(&mut self) {
        unsafe { super::CFRelease(self.0) };
    }
}

impl Event {
    /// Creates a new mouse event for `button` of `kind` at `location`.
    ///
    /// This function allocates a new `CGEvent`.
    pub fn new(button: Button, kind: EventKind, location: (f64, f64)) -> Event {
        use super::CGEventType::*;

        let event_type = match (button, kind) {
            (Button::Left,   EventKind::Down)    => LeftMouseDown,
            (Button::Left,   EventKind::Up)      => LeftMouseUp,
            (Button::Left,   EventKind::Moved)   => MouseMoved,
            (Button::Left,   EventKind::Dragged) => LeftMouseDragged,
            (Button::Right,  EventKind::Down)    => RightMouseDown,
            (Button::Right,  EventKind::Up)      => RightMouseUp,
            (Button::Right,  EventKind::Moved)   => MouseMoved,
            (Button::Right,  EventKind::Dragged) => RightMouseDragged,
        };

        unsafe { Event(CGEventCreateMouseEvent(
            ptr::null(),
            event_type,
            location.into(),
            button as raw::c_int,
        )) }
    }

    /// Returns the location of the inner Quartz mouse event.
    #[inline]
    pub fn location(&self) -> (f64, f64) {
        unsafe { CGEventGetLocation(self.0).into() }
    }

    /// Sets the location of the inner Quartz mouse event.
    #[inline]
    pub fn set_location(&mut self, location: (f64, f64)) {
        unsafe { CGEventSetLocation(self.0, location.into()) }
    }

    /// Posts `self` to the Quartz event stream at the event location.
    #[inline]
    pub fn post(&self, location: super::EventLocation) {
        unsafe { CGEventPost(location as raw::c_int, self.0) };
    }
}

/// The kind of operation being performed by the mouse event.
#[derive(Copy, Clone)]
pub enum EventKind {
    /// Mouse pressed down.
    Down,
    /// Mouse released.
    Up,
    /// Mouse moved from one location to another.
    Moved,
    /// Mouse dragged across one location to another.
    Dragged,
}

/// A type that can be used to get the current mouse location as an (x, y) pair.
///
/// In macOS, values near 0 for x and y are located at the bottom, left-hand
/// side of the screen.
#[derive(Copy, Clone)]
pub struct Location {
    ns_event: &'static Class
}

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Location").finish()
    }
}

impl Iterator for Location {
    type Item = (f64, f64);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        unsafe { Some(Self::get_from(self.ns_event)) }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::max_value(), None)
    }
}

impl Location {
    unsafe fn get_from(ns_event: &Class) -> (f64, f64) {
        From::<CGPoint>::from(msg_send![ns_event, mouseLocation])
    }

    /// Returns the current mouse location.
    pub fn get() -> (f64, f64) {
        unsafe { Self::get_from(&NS_EVENT) }
    }

    /// Returns an iterator over current mouse locations.
    pub fn iter() -> Location {
        Location { ns_event: &NS_EVENT }
    }
}

#[cfg(all(test, nightly))]
mod benches {
    use super::*;
    use test::{Bencher, black_box};

    #[bench]
    fn location_get_100(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..100 {
                black_box(Location::get());
            }
        });
    }

    #[bench]
    fn location_iter_100(b: &mut Bencher) {
        b.iter(|| {
            for loc in Location::iter().take(100) {
                black_box(loc);
            }
        })
    }
}
