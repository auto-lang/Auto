//! 🎡 Scroll wheel automation utilities.

use std::os::raw;
use std::ptr;

use super::{CGEvent, CGEventSource};

extern {
    fn CGEventCreateScrollWheelEvent(
        source: CGEventSource,
        units: raw::c_int,
        wheelCount: u32,
        wheel1: i32,
        ...
    ) -> CGEvent;
}

/// The unit of measurement for a scroll wheel event.
///
/// By default, the ratio is about ten pixels per line.
pub enum ScrollUnit {
    /// Produces an event that most applications interpret as a smooth scrolling
    /// event.
    Pixel,
    /// The offset is by lines.
    Line,
}

declare_event!("A scroll wheel event that can be posted into the Quartz event stream.");

impl Event {
    /// Creates a new scroll wheel event with `unit`-sized offsets in `wheels`.
    ///
    /// The number of wheels must be between 1 and 3, inclusive. Type checking
    /// statically ensures an input within the valid range.
    ///
    /// This function allocates a new `CGEvent`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # #[cfg(target_os = "macos")] {
    /// use auto::os::macos::{wheel, EventLocation};
    ///
    /// let unit  = wheel::ScrollUnit::Line;
    /// let event = wheel::Event::new(unit, [-5, 20]);
    /// # return;
    /// event.post(EventLocation::Session);
    /// # }
    /// ```
    pub fn new<W: Wheels>(unit: ScrollUnit, wheels: W) -> Event {
        let slice = wheels.as_ref();
        let count = slice.len() as u32;

        let null = ptr::null();
        let unit = unit as raw::c_int;

        Event(super::Event(unsafe { match count {
            1 => CGEventCreateScrollWheelEvent(
                null, unit, count, slice[0]
            ),
            2 => CGEventCreateScrollWheelEvent(
                null, unit, count, slice[0], slice[1]
            ),
            _ => CGEventCreateScrollWheelEvent(
                null, unit, count, slice[0], slice[1], slice[2]
            ),
        } }))
    }
}

/// A vector of scroll wheel values.
pub trait Wheels: ::private::Priv + AsRef<[i32]> {}

macro_rules! impl_wheels {
    ($($n:expr)+) => { $(
        impl ::private::Priv for [i32; $n] {}
        impl Wheels for [i32; $n] {}
    )+ }
}

impl_wheels! { 1 2 3 }
