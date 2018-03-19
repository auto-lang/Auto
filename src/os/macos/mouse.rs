//! 🖱️ Mouse automation utilities.

use std::{fmt, mem, ptr};
use std::os::raw;

use objc::runtime::Class;

use super::{CFObjectRef, CGEventType, CGEventSource, CGPoint, NS_EVENT};

extern {
    fn CGEventCreateMouseEvent(
        source: CGEventSource,
        mouse_type: CGEventType,
        mouse_cursor_position: CGPoint,
        mouse_button: raw::c_int,
    ) -> *mut raw::c_void;

    fn CGEventGetLocation(event: CFObjectRef) -> CGPoint;

    fn CGEventGetUnflippedLocation(event: CFObjectRef) -> CGPoint;

    fn CGEventSetLocation(event: CFObjectRef, location: CGPoint);

    fn CGWarpMouseCursorPosition(new_cursor_position: CGPoint) -> CGPoint;
}

unsafe fn location_from(ns_event: &Class) -> Location {
    From::<CGPoint>::from(msg_send![ns_event, mouseLocation])
}

/// Returns the current mouse location.
pub fn location() -> Location {
    unsafe { location_from(&NS_EVENT) }
}

/// Returns an iterator over current mouse locations.
pub fn location_iter() -> LocationIter {
    LocationIter { ns_event: &NS_EVENT }
}

/// Moves the mouse cursor without generating events.
#[inline]
pub fn warp_location(location: Location) {
    unsafe { CGWarpMouseCursorPosition(location.into()) };
}

/// A location on the screen.
pub type Location = (f64, f64);

impl From<CGPoint> for Location {
    #[inline]
    fn from(point: CGPoint) -> Location {
        (point.x as _, point.y as _)
    }
}

impl From<Location> for CGPoint {
    #[inline]
    fn from((x, y): Location) -> CGPoint {
        CGPoint { x: x as _, y: y as _ }
    }
}

/// A button on the mouse.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Button {
    /// Left button where the index finger would press.
    Left,
    /// Right button where the middle finger might press.
    Right,
}

impl From<(Button, EventKind)> for super::CGEventType {
    fn from(pair: (Button, EventKind)) -> Self {
        use super::CGEventType::*;
        match pair {
            (Button::Left,   EventKind::Down)    => LeftMouseDown,
            (Button::Left,   EventKind::Up)      => LeftMouseUp,
            (Button::Left,   EventKind::Moved)   => MouseMoved,
            (Button::Left,   EventKind::Dragged) => LeftMouseDragged,
            (Button::Right,  EventKind::Down)    => RightMouseDown,
            (Button::Right,  EventKind::Up)      => RightMouseUp,
            (Button::Right,  EventKind::Moved)   => MouseMoved,
            (Button::Right,  EventKind::Dragged) => RightMouseDragged,
        }
    }
}

declare_event!("A mouse event that can be posted into the Quartz event stream.");

impl Event {
    /// Creates a new mouse event for `button` of `kind` at `location`.
    ///
    /// This function allocates a new `CGEvent`.
    pub fn new(button: Button, kind: EventKind, location: Location) -> Event {
        unsafe {
            let event = CGEventCreateMouseEvent(
                ptr::null(),
                (button, kind).into(),
                location.into(),
                button as raw::c_int,
            );
            mem::transmute(event)
        }
    }

    /// Returns the location of the inner Quartz mouse event.
    #[inline]
    pub fn location(&self) -> Location {
        unsafe { CGEventGetLocation(((self.0).0).0).into() }
    }

    /// Returns the location of the inner Quartz mouse event relative to the
    /// lower-left corner of the main display.
    #[inline]
    pub fn location_unflipped(&self) -> Location {
        unsafe { CGEventGetUnflippedLocation(((self.0).0).0).into() }
    }

    /// Sets the location of the inner Quartz mouse event.
    #[inline]
    pub fn set_location(&mut self, location: Location) {
        unsafe { CGEventSetLocation(((self.0).0).0, location.into()) }
    }
}

/// The kind of operation being performed by the mouse event.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
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
pub struct LocationIter {
    ns_event: &'static Class
}

impl fmt::Debug for LocationIter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Location").finish()
    }
}

impl Iterator for LocationIter {
    type Item = Location;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        unsafe { Some(location_from(self.ns_event)) }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::max_value(), None)
    }
}

#[cfg(all(test, nightly))]
mod benches {
    use super::*;
    use test::{Bencher, black_box};

    #[bench]
    fn location_100(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..100 {
                black_box(location());
            }
        });
    }

    #[bench]
    fn location_iter_100(b: &mut Bencher) {
        b.iter(|| {
            for loc in location_iter().take(100) {
                black_box(loc);
            }
        })
    }
}
