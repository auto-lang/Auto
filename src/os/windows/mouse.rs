//! 🖱️ Mouse automation utilities.

use std::mem;

use winapi::shared::windef::POINT;
use winapi::um::winuser::{GetCursorPos, SetCursorPos};

/// Returns the current mouse location.
///
/// The calling process must have `WINSTA_READATTRIBUTES` access to the window
/// station.
pub fn location() -> Option<Location> {
    unsafe {
        let mut point = mem::uninitialized::<POINT>();
        if GetCursorPos(&mut point) != 0 {
            Some((point.x as _, point.y as _))
        } else {
            None
        }
    }
}

/// Moves the cursor to the specified screen coordinates. If the new coordinates
/// are not within the screen rectangle, the system automatically adjusts the
/// coordinates so that the cursor stays within the rectangle.
///
/// The calling process must have `WINSTA_WRITEATTRIBUTES` access to the window
/// station.
#[inline]
pub fn set_location((x, y): (usize, usize)) -> bool {
    unsafe { SetCursorPos(x as _, y as _) != 0 }
}

/// A location on the screen.
pub type Location = (usize, usize);

#[cfg(test)]
mod tests {
    #[test]
    fn location() {
        let orig = super::location().expect("Cannot get mouse location");
        let other = (40, 10);

        assert!(super::set_location(other));
        assert_eq!(super::location(), Some(other));

        assert!(super::set_location(orig));
        assert_eq!(super::location(), Some(orig));
    }
}
