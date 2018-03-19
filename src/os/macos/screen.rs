//! 📺 Screen information utilities.

extern {
    fn CGMainDisplayID() -> Display;

    fn CGGetOnlineDisplayList(
        max_displays: u32,
        online_displays: *mut Display,
        displayCount: *mut u32
    ) -> CGError;

    fn CGGetActiveDisplayList(
        max_displays: u32,
        online_displays: *mut Display,
        displayCount: *mut u32
    ) -> CGError;
}

type CGError = i32;

type CGDisplayListGetter = unsafe extern fn(u32, *mut Display, *mut u32) -> CGError;

/// A monitor display.
#[repr(C)]
#[derive(Copy, Clone, Debug, Hash)]
pub struct Display(u32);

fn displays_with(get: CGDisplayListGetter) -> Option<Vec<Display>> {
    let mut count = 0u32;
    if unsafe { get(0, 0 as _, &mut count) } == 0 {
        let mut buffer = vec![Display(0); count as usize];
        if unsafe { get(count, buffer.as_mut_ptr(), &mut count) } == 0 {
            return Some(buffer);
        }
    }
    None
}

impl Display {
    /// Returns the main display.
    ///
    /// The result is the display with its screen location at (0,0) in the
    /// global display coordinate space. In a system without display mirroring,
    /// the display with the menu bar is typically the main display.
    ///
    /// If mirroring is enabled and the menu bar appears on more than one
    /// display, this function provides a reliable way to find the main display.
    ///
    /// In case of hardware mirroring, the drawable display becomes the main
    /// display. In case of software mirroring, the display with the highest
    /// resolution and deepest pixel depth typically becomes the main display.
    pub fn main() -> Display {
        unsafe { CGMainDisplayID() }
    }

    /// Returns all displays that are online (active, mirrored, or sleeping).
    ///
    /// If the framebuffer hardware is connected, a display is considered connected
    /// or online.
    ///
    /// When hardware mirroring is used, a display can be online but not active or
    /// drawable. Programs that manipulate display settings (such as gamma tables)
    /// need access to all displays, including hardware mirrors, which are not
    /// drawable.
    pub fn online() -> Option<Vec<Display>> {
        displays_with(CGGetOnlineDisplayList)
    }

    /// Returns all displays that are active (or drawable).
    ///
    /// The first entry is the main display. In case of mirroring, the first
    /// entry is the largest drawable display or, if all are the same size, the
    /// display with the greatest pixel depth.
    ///
    /// Note that when hardware mirroring is being used between displays, only
    /// the primary display is active and appears in the list. When software
    /// mirroring is being used, all the mirrored displays are active and appear
    /// in the list.
    pub fn active() -> Option<Vec<Display>> {
        displays_with(CGGetActiveDisplayList)
    }
}
