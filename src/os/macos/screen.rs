//! 📺 Screen information utilities.

extern {
    fn CGMainDisplayID() -> DisplayId;
}

/// Returns the ID of the main display.
#[inline]
pub fn main_display() -> DisplayId {
    unsafe { CGMainDisplayID() }
}

/// A unique identifier for an attached display.
pub type DisplayId = u32;
