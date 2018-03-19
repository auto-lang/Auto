//! 📺 Screen information utilities.

extern {
    fn CGMainDisplayID() -> DisplayId;
}

/// Returns the ID of the main display.
///
/// The main display is the display with its screen location at (0,0) in the
/// global display coordinate space. In a system without display mirroring, the
/// display with the menu bar is typically the main display.
///
/// If mirroring is enabled and the menu bar appears on more than one display,
/// this function provides a reliable way to find the main display.
///
/// In case of hardware mirroring, the drawable display becomes the main
/// display. In case of software mirroring, the display with the highest
/// resolution and deepest pixel depth typically becomes the main display.
#[inline]
pub fn main_display() -> DisplayId {
    unsafe { CGMainDisplayID() }
}

/// A unique identifier for an attached display.
pub type DisplayId = u32;
