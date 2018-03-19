//! 📺 Screen information utilities.

extern {
    fn CGMainDisplayID() -> DisplayId;

    fn CGGetOnlineDisplayList(
        max_displays: u32,
        online_displays: *mut DisplayId,
        displayCount: *mut u32
    ) -> CGError;
}

type CGError = i32;

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

/// Returns IDs for displays that are online (active, mirrored, or sleeping).
///
/// If the framebuffer hardware is connected, a display is considered connected
/// or online.
///
/// When hardware mirroring is used, a display can be online but not active or
/// drawable. Programs that manipulate display settings (such as gamma tables)
/// need access to all displays, including hardware mirrors, which are not
/// drawable.
#[inline]
pub fn online_displays() -> Option<Vec<DisplayId>> {
    macro_rules! handle {
        ($e:expr) => { match $e {
            0 => (),
            _ => return None,
        } }
    }

    let mut count = 0u32;
    handle!(unsafe { CGGetOnlineDisplayList(0, 0 as _, &mut count) });

    let mut buffer = vec![0; count as usize];
    let ptr = buffer.as_mut_ptr();
    handle!(unsafe { CGGetOnlineDisplayList(count, ptr, &mut count) });

    Some(buffer)
}

/// A unique identifier for an attached display.
pub type DisplayId = u32;
