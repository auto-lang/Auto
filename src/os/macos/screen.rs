//! 📺 Screen information utilities.

use std::os::raw;
use std::ptr;

use libc::size_t;
use objc::runtime::Class;

use super::{CGFloat, CGRect, CGSize, NSObject, NSObjectRef};
use color::Rgb;

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

    fn CGDisplayScreenSize(display: Display) -> CGSize;

    fn CGDisplayBounds(display: Display) -> CGRect;

    fn CGDisplayPixelsHigh(display: Display) -> size_t;

    fn CGDisplayPixelsWide(display: Display) -> size_t;

    fn CGDisplayCreateImageForRect(display: Display, rect: CGRect) -> Option<CGImage>;

    fn CGImageRelease(image: CGImageRef);
}

lazy_static! {
    static ref NS_BITMAP: &'static Class = Class::get("NSBitmapImageRep").unwrap();
}

type CGError = i32;

type CGDisplayListGetter = unsafe extern fn(u32, *mut Display, *mut u32) -> CGError;

type CGImageRef = ptr::NonNull<raw::c_void>;

struct CGImage(CGImageRef);

impl Drop for CGImage {
    fn drop(&mut self) {
        unsafe { CGImageRelease(self.0) };
    }
}

/// The location and dimensions of a display.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Bounds {
    /// Coordinates of the origin.
    pub origin: (f64, f64),
    /// Height and width of the bounds.
    pub size: (f64, f64),
}

impl From<CGRect> for Bounds {
    #[inline]
    fn from(rect: CGRect) -> Bounds {
        Bounds {
            origin: (rect.origin.x as _, rect.origin.y as _),
            size: (rect.size.width as _, rect.size.height as _),
        }
    }
}

/// A monitor display.
#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Display(u32);

fn displays_with(get: CGDisplayListGetter) -> Vec<Display> {
    let mut count = 0u32;
    if unsafe { get(0, 0 as _, &mut count) } != 0 {
        return Vec::new();
    }

    let mut buffer = vec![Display(0); count as usize];
    if unsafe { get(count, buffer.as_mut_ptr(), &mut count) } != 0 {
        buffer.clear();
    }

    buffer
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
    pub fn online() -> Vec<Display> {
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
    pub fn active() -> Vec<Display> {
        displays_with(CGGetActiveDisplayList)
    }

    /// Returns the color at the location relative to the origin of the display.
    pub fn color_at(self, (x, y): (f64, f64)) -> Option<Rgb> {
        let rect  = CGRect::new(x as _, y as _, 1.0, 1.0);
        let image = unsafe { CGDisplayCreateImageForRect(self, rect) }?;

        let bitmap: NSObject = NSObject::alloc(&NS_BITMAP);
        let bitmap = bitmap.inner();

        unsafe {
            let _: NSObjectRef  = msg_send![bitmap, initWithCGImage:image];
            let color: NSObject = msg_send![bitmap, colorAtX:0usize y:0usize];
            let color = color.inner();

            let r: CGFloat = msg_send![color, redComponent];
            let g: CGFloat = msg_send![color, greenComponent];
            let b: CGFloat = msg_send![color, blueComponent];

            Some(Rgb { red: r as _, green: g as _, blue: b as _ })
        }
    }

    /// Returns the width and height of the display in millimeters, or 0 if the
    /// display is not valid.
    #[inline]
    pub fn size(self) -> (f64, f64) {
        let CGSize { width, height } = unsafe { CGDisplayScreenSize(self) };
        (width as _, height as _)
    }

    /// Returns the bounds of the display in the global coordinate space.
    #[inline]
    pub fn bounds(self) -> Bounds {
        unsafe { CGDisplayBounds(self).into() }
    }

    /// Returns the width and height in pixel units.
    #[inline]
    pub fn pixels(self) -> (usize, usize) {
        unsafe { (
            CGDisplayPixelsWide(self) as usize,
            CGDisplayPixelsHigh(self) as usize,
        ) }
    }
}

#[cfg(all(test, nightly))]
mod benches {
    use super::*;
    use test::{Bencher, black_box};

    #[bench]
    fn color_at(b: &mut Bencher) {
        let display = Display::main();
        let loc = (0.0, 0.0);
        b.iter(|| {
            black_box(display.color_at(black_box(loc)));
        });
    }
}
