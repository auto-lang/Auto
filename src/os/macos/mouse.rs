//! Mouse automation utilities.

use std::fmt;
use std::mem;

use objc::runtime::Class;

use super::{CGPoint, NS_EVENT};

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
