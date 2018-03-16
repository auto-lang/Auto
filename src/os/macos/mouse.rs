//! Mouse automation utilities.

use std::mem;

use objc::runtime::Class;

use super::{CGPoint, NS_EVENT};

/// A type that can be used to get the current mouse location.
#[derive(Copy, Clone)]
pub struct Location {
    ns_event: &'static Class
}

impl Iterator for Location {
    type Item = (f64, f64);

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
        mem::transmute::<CGPoint, _>(msg_send![ns_event, mouseLocation])
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
